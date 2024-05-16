use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    path::PathBuf,
    sync::Arc,
    time::Instant,
};

use alloy_dyn_abi::{DynSolType, DynSolValue, ResolveSolEvent};
use anyhow::{anyhow, Context, Result};
use arrow2::{
    array::{
        Array, BinaryArray, MutableArray, MutableBinaryArray, MutableBooleanArray,
        MutableUtf8Array, Utf8Array,
    },
    datatypes::{DataType, Field, Schema},
    io::parquet::{
        read::ParquetError,
        write::{
            array_to_columns, to_parquet_schema, to_parquet_type, transverse, CompressedPage,
            DynIter, DynStreamingIterator, Encoding, FallibleStreamingIterator, RowGroupIter,
            WriteOptions,
        },
    },
};
use parquet2::write::FileStreamer;
use rayon::prelude::*;
use skar_net_types::Query;
use skar_schema::{concat_chunks, empty_chunk, project_schema};
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::{
    column_mapping, rayon_async, types::StreamConfig, ArrowBatch, ArrowChunk, Client, ParquetConfig,
};

pub async fn create_parquet_folder(
    client: &Client,
    query: Query,
    config: ParquetConfig,
) -> Result<()> {
    let path = PathBuf::from(config.path);

    tokio::fs::create_dir_all(&path)
        .await
        .context("create parquet dir")?;

    let mut blocks_path = path.clone();
    blocks_path.push("blocks.parquet");
    let (mut blocks_sender, blocks_join) = spawn_writer(
        blocks_path,
        &skar_schema::block_header(),
        Some(&query.field_selection.block),
        &config.column_mapping.block,
        config.hex_output,
    )?;

    let mut transactions_path = path.clone();
    transactions_path.push("transactions.parquet");
    let (mut transactions_sender, transactions_join) = spawn_writer(
        transactions_path,
        &skar_schema::transaction(),
        Some(&query.field_selection.transaction),
        &config.column_mapping.transaction,
        config.hex_output,
    )?;

    let mut logs_path = path.clone();
    logs_path.push("logs.parquet");
    let (mut logs_sender, logs_join) = spawn_writer(
        logs_path,
        &skar_schema::log(),
        Some(&query.field_selection.log),
        &config.column_mapping.log,
        config.hex_output,
    )?;

    let mut traces_path = path.clone();
    traces_path.push("traces.parquet");
    let (mut traces_sender, traces_join) = spawn_writer(
        traces_path,
        &skar_schema::trace(),
        Some(&query.field_selection.trace),
        &config.column_mapping.trace,
        config.hex_output,
    )?;

    let event_signature = match &config.event_signature {
        Some(sig) => Some(alloy_json_abi::Event::parse(sig).context("parse event signature")?),
        None => None,
    };

    let mut decoded_logs_path = path.clone();
    decoded_logs_path.push("decoded_logs.parquet");
    let (mut decoded_logs_sender, decoded_logs_join) = spawn_writer(
        decoded_logs_path,
        &schema_from_event_signature(&event_signature)
            .context("arrow schema from event signature")?,
        None,
        &config.column_mapping.decoded_log,
        config.hex_output,
    )?;

    let mut rx = client
        .stream::<crate::ArrowIpc>(
            query,
            StreamConfig {
                concurrency: config.concurrency,
                batch_size: config.batch_size,
                retry: config.retry,
            },
        )
        .await
        .context("start stream")?;

    while let Some(resp) = rx.recv().await {
        let resp = resp.context("get query response")?;

        log::trace!("got data up to block {}", resp.next_block);

        let blocks_fut = async move {
            for batch in resp.data.blocks {
                blocks_sender
                    .send(batch)
                    .await
                    .context("write blocks chunk to parquet")?;
            }

            Ok::<_, anyhow::Error>(blocks_sender)
        };

        let txs_fut = async move {
            for batch in resp.data.transactions {
                transactions_sender
                    .send(batch)
                    .await
                    .context("write transactions chunk to parquet")?;
            }

            Ok::<_, anyhow::Error>(transactions_sender)
        };

        let logs_fut = {
            let data = resp.data.logs.clone();
            async move {
                for batch in data {
                    logs_sender
                        .send(batch)
                        .await
                        .context("write logs chunk to parquet")?;
                }

                Ok::<_, anyhow::Error>(logs_sender)
            }
        };

        let traces_fut = async move {
            for batch in resp.data.traces {
                traces_sender
                    .send(batch)
                    .await
                    .context("write traces chunk to parquet")?;
            }

            Ok::<_, anyhow::Error>(traces_sender)
        };

        let sig = Arc::new(event_signature.clone());
        let decoded_logs_fut = async move {
            for batch in resp.data.logs {
                let sig = sig.clone();
                let batch = rayon_async::spawn(move || decode_logs_batch(&sig, batch))
                    .await
                    .context("join decode logs task")?
                    .context("decode logs")?;

                decoded_logs_sender
                    .send(batch)
                    .await
                    .context("write decoded_logs chunk to parquet")?;
            }

            Ok::<_, anyhow::Error>(decoded_logs_sender)
        };

        let start = Instant::now();

        (
            blocks_sender,
            transactions_sender,
            logs_sender,
            traces_sender,
            decoded_logs_sender,
        ) = futures::future::try_join5(blocks_fut, txs_fut, logs_fut, traces_fut, decoded_logs_fut)
            .await
            .context("write to parquet")?;

        log::trace!("wrote to parquet in {} ms", start.elapsed().as_millis());
    }

    std::mem::drop(blocks_sender);
    std::mem::drop(transactions_sender);
    std::mem::drop(logs_sender);
    std::mem::drop(traces_sender);
    std::mem::drop(decoded_logs_sender);

    blocks_join
        .await
        .context("join blocks task")?
        .context("finish blocks file")?;
    transactions_join
        .await
        .context("join transactions task")?
        .context("finish transactions file")?;
    logs_join
        .await
        .context("join logs task")?
        .context("finish logs file")?;
    traces_join
        .await
        .context("join traces task")?
        .context("finish traces file")?;
    decoded_logs_join
        .await
        .context("join decoded_logs task")?
        .context("finish decoded_logs file")?;

    Ok(())
}

fn hex_encode_chunk(chunk: &ArrowChunk) -> anyhow::Result<ArrowChunk> {
    let cols = chunk
        .columns()
        .par_iter()
        .map(|col| {
            let col = match col.data_type() {
                DataType::Binary => Box::new(hex_encode(col.as_any().downcast_ref().unwrap())),
                _ => col.clone(),
            };

            Ok::<_, anyhow::Error>(col)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(ArrowChunk::new(cols))
}

fn hex_encode(input: &BinaryArray<i32>) -> Utf8Array<i32> {
    let mut arr = MutableUtf8Array::new();

    for buf in input.iter() {
        arr.push(buf.map(faster_hex::hex_string));
    }

    arr.into()
}

fn spawn_writer(
    path: PathBuf,
    schema: &Schema,
    field_selection: Option<&BTreeSet<String>>,
    mapping: &BTreeMap<String, column_mapping::DataType>,
    hex_output: bool,
) -> Result<(mpsc::Sender<ArrowBatch>, JoinHandle<Result<()>>)> {
    let schema = if let Some(field_selection) = field_selection {
        project_schema(schema, field_selection).context("project schema")?
    } else {
        schema.clone()
    };
    let schema = column_mapping::apply_to_schema(&schema, mapping)
        .context("apply column mapping to schema")?;
    let schema = if hex_output {
        Schema::from(
            schema
                .fields
                .iter()
                .map(|field| match field.data_type() {
                    DataType::Binary => {
                        Field::new(field.name.clone(), DataType::Utf8, field.is_nullable)
                    }
                    _ => field.clone(),
                })
                .collect::<Vec<_>>(),
        )
    } else {
        schema
    };

    let (tx, rx) = mpsc::channel(64);

    let mapping = Arc::new(mapping.clone());

    let handle = tokio::task::spawn(async move {
        match run_writer(rx, path, schema, mapping, hex_output).await {
            Ok(v) => Ok(v),
            Err(e) => {
                log::error!("failed to run parquet writer: {:?}", e);
                Err(e)
            }
        }
    });

    Ok((tx, handle))
}

async fn run_writer(
    mut rx: mpsc::Receiver<ArrowBatch>,
    path: PathBuf,
    schema: Schema,
    mapping: Arc<BTreeMap<String, crate::DataType>>,
    hex_output: bool,
) -> Result<()> {
    let write_options = parquet2::write::WriteOptions {
        write_statistics: true,
        version: parquet2::write::Version::V2,
    };

    let file = tokio::fs::File::create(&path)
        .await
        .context("create parquet file")?
        .compat();

    let parquet_schema = to_parquet_schema(&schema).context("to parquet schema")?;

    let schema = Arc::new(schema);

    let mut writer = FileStreamer::new(file, parquet_schema, write_options, None);

    let num_cpus = num_cpus::get();
    let mut encode_jobs = VecDeque::<EncodeFut>::with_capacity(num_cpus);

    let mut data = Vec::new();
    let mut total_rows = 0;
    loop {
        let mut stop = false;
        if let Some(batch) = rx.recv().await {
            total_rows += batch.chunk.len();
            data.push(batch);
        } else {
            stop = true;
        }

        if !data.is_empty() && (stop || total_rows >= ROW_GROUP_MAX_ROWS) {
            if encode_jobs.len() >= num_cpus {
                let fut = encode_jobs.pop_front().unwrap();
                let rg = fut
                    .await
                    .context("join prepare task")?
                    .context("prepare row group")?;
                writer
                    .write(rg)
                    .await
                    .context("write encoded row group to file")?;
            }

            let batches = std::mem::take(&mut data);
            total_rows = 0;
            let chunks = batches
                .into_iter()
                .map(|b| Arc::new(b.chunk))
                .collect::<Vec<_>>();

            let chunk = concat_chunks(chunks.as_slice()).context("concat chunks")?;

            let mapping = mapping.clone();
            let schema = schema.clone();
            let fut = rayon_async::spawn(move || {
                let field_names = schema
                    .fields
                    .iter()
                    .map(|f| f.name.as_str())
                    .collect::<Vec<&str>>();
                let chunk = column_mapping::apply_to_chunk(&chunk, &field_names, &mapping)
                    .context("apply column mapping to batch")?;

                let chunk = if hex_output {
                    hex_encode_chunk(&chunk).context("hex encode batch")?
                } else {
                    chunk
                };

                encode_row_group(
                    ArrowBatch { chunk, schema },
                    WriteOptions {
                        write_statistics: true,
                        version: arrow2::io::parquet::write::Version::V2,
                        compression: arrow2::io::parquet::write::CompressionOptions::Lz4Raw,
                        data_pagesize_limit: None,
                    },
                )
            });

            encode_jobs.push_back(fut);
        }

        if stop {
            break;
        }
    }

    while let Some(fut) = encode_jobs.pop_front() {
        let rg = fut
            .await
            .context("join prepare task")?
            .context("prepare row group")?;
        writer
            .write(rg)
            .await
            .context("write encoded row group to file")?;
    }

    let _size = writer.end(None).await.context("write footer")?;

    Ok(())
}

type EncodeFut = tokio::sync::oneshot::Receiver<
    Result<
        DynIter<
            'static,
            std::result::Result<
                DynStreamingIterator<'static, CompressedPage, arrow2::error::Error>,
                arrow2::error::Error,
            >,
        >,
    >,
>;

fn encode_row_group(
    batch: ArrowBatch,
    write_options: WriteOptions,
) -> Result<RowGroupIter<'static, arrow2::error::Error>> {
    let fields = batch
        .schema
        .fields
        .iter()
        .map(|field| to_parquet_type(field).context("map to parquet field"))
        .collect::<Result<Vec<_>>>()?;
    let encodings = batch
        .schema
        .fields
        .iter()
        .map(|f| transverse(&f.data_type, |_| Encoding::Plain))
        .collect::<Vec<_>>();

    let data = batch
        .chunk
        .into_arrays()
        .into_iter()
        .zip(fields)
        .zip(encodings)
        .flat_map(move |((array, type_), encoding)| {
            let encoded_columns = array_to_columns(array, type_, write_options, &encoding).unwrap();
            encoded_columns
                .into_iter()
                .map(|encoded_pages| {
                    let pages = encoded_pages;

                    let pages = DynIter::new(
                        pages
                            .into_iter()
                            .map(|x| x.map_err(|e| ParquetError::OutOfSpec(e.to_string()))),
                    );

                    let compressed_pages = pages
                        .map(|page| {
                            let page = page?;
                            arrow2::io::parquet::write::compress(
                                page,
                                vec![],
                                write_options.compression,
                            )
                            .map_err(arrow2::error::Error::from)
                        })
                        .collect::<Vec<_>>();

                    Ok(DynStreamingIterator::new(CompressedPageIter {
                        data: compressed_pages.into_iter(),
                        current: None,
                    }))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok(DynIter::new(data.into_iter()))
}

struct CompressedPageIter {
    data: std::vec::IntoIter<std::result::Result<CompressedPage, arrow2::error::Error>>,
    current: Option<CompressedPage>,
}

impl FallibleStreamingIterator for CompressedPageIter {
    type Item = CompressedPage;
    type Error = arrow2::error::Error;

    fn get(&self) -> Option<&Self::Item> {
        self.current.as_ref()
    }

    fn advance(&mut self) -> std::result::Result<(), Self::Error> {
        self.current = match self.data.next() {
            Some(page) => Some(page?),
            None => None,
        };
        Ok(())
    }
}

const ROW_GROUP_MAX_ROWS: usize = 10_000;

fn decode_logs_batch(sig: &Option<alloy_json_abi::Event>, batch: ArrowBatch) -> Result<ArrowBatch> {
    let schema =
        schema_from_event_signature(sig).context("build arrow schema from event signature")?;

    if batch.chunk.is_empty() {
        return Ok(ArrowBatch {
            chunk: empty_chunk(&schema),
            schema: Arc::new(schema),
        });
    }

    let sig = match sig {
        Some(sig) => sig,
        None => {
            return Ok(ArrowBatch {
                chunk: empty_chunk(&schema),
                schema: Arc::new(schema),
            })
        }
    };

    let event = sig.resolve().context("resolve signature into event")?;

    let topic_cols = event
        .indexed()
        .iter()
        .zip(["topic1", "topic2", "topic3"].iter())
        .map(|(decoder, topic_name)| {
            let col = batch
                .column::<BinaryArray<i32>>(topic_name)
                .context("get column")?;
            let col = decode_col(col, decoder).context("decode column")?;
            Ok::<_, anyhow::Error>(col)
        })
        .collect::<Result<Vec<_>>>()?;

    let body_cols = if event.body() == [DynSolType::Uint(256)] {
        let data = batch
            .column::<BinaryArray<i32>>("data")
            .context("get column")?;
        vec![decode_erc20_amount(data, &DynSolType::Uint(256)).context("decode amount column")?]
    } else if !event.body().is_empty() {
        let data = batch
            .column::<BinaryArray<i32>>("data")
            .context("get column")?;

        let tuple_decoder = DynSolType::Tuple(event.body().to_vec());

        let mut decoded_tuples = Vec::with_capacity(data.len());
        for val in data.values_iter() {
            let tuple = tuple_decoder
                .abi_decode(val)
                .context("decode body tuple")?
                .as_tuple()
                .context("expected tuple after decoding")?
                .to_vec();

            if tuple.len() != event.body().len() {
                return Err(anyhow!(
                    "expected tuple of length {} after decoding",
                    event.body().len()
                ));
            }

            decoded_tuples.push(tuple);
        }

        let mut decoded_cols = Vec::with_capacity(event.body().len());

        for (i, ty) in event.body().iter().enumerate() {
            decoded_cols.push(
                decode_body_col(decoded_tuples.iter().map(|t| t.get(i).unwrap()), ty)
                    .context("decode body column")?,
            );
        }

        decoded_cols
    } else {
        Vec::new()
    };

    let mut cols = topic_cols;
    cols.extend_from_slice(&body_cols);

    let chunk = ArrowChunk::try_new(cols).context("create arrow chunk")?;

    Ok(ArrowBatch {
        chunk,
        schema: Arc::new(schema),
    })
}

fn decode_body_col<'a, I: ExactSizeIterator<Item = &'a DynSolValue>>(
    vals: I,
    ty: &DynSolType,
) -> Result<Box<dyn Array>> {
    match ty {
        DynSolType::Bool => {
            let mut builder = MutableBooleanArray::with_capacity(vals.len());

            for val in vals {
                match val {
                    DynSolValue::Bool(b) => builder.push(Some(*b)),
                    v => {
                        return Err(anyhow!(
                            "unexpected output type from decode: {:?}",
                            v.as_type()
                        ))
                    }
                }
            }

            Ok(builder.as_box())
        }
        _ => {
            let mut builder = MutableBinaryArray::<i32>::new();

            for val in vals {
                match val {
                    DynSolValue::Int(v, _) => builder.push(Some(v.to_be_bytes::<32>())),
                    DynSolValue::Uint(v, _) => builder.push(Some(v.to_be_bytes::<32>())),
                    DynSolValue::FixedBytes(v, _) => builder.push(Some(v)),
                    DynSolValue::Address(v) => builder.push(Some(v)),
                    DynSolValue::Bytes(v) => builder.push(Some(v)),
                    DynSolValue::String(v) => builder.push(Some(v)),
                    v => {
                        return Err(anyhow!(
                            "unexpected output type from decode: {:?}",
                            v.as_type()
                        ))
                    }
                }
            }

            Ok(builder.as_box())
        }
    }
}

fn decode_erc20_amount(data: &BinaryArray<i32>, decoder: &DynSolType) -> Result<Box<dyn Array>> {
    let mut builder = MutableBinaryArray::<i32>::new();

    for val in data.values_iter() {
        // Check if we are decoding a single u256 and the body is empty
        //
        // This case can happen when decoding zero value erc20 transfers
        let v = if val.is_empty() {
            [0; 32].as_slice()
        } else {
            val
        };

        match decoder.abi_decode(v).context("decode val")? {
            DynSolValue::Uint(v, _) => builder.push(Some(v.to_be_bytes::<32>())),
            v => {
                return Err(anyhow!(
                    "unexpected output type from decode: {:?}",
                    v.as_type()
                ))
            }
        }
    }

    Ok(builder.as_box())
}

fn decode_col(col: &BinaryArray<i32>, decoder: &DynSolType) -> Result<Box<dyn Array>> {
    match decoder {
        DynSolType::Bool => {
            let mut builder = MutableBooleanArray::with_capacity(col.len());

            for val in col.iter() {
                let val = val.context("found null value")?;
                match decoder.abi_decode(val).context("decode sol value")? {
                    DynSolValue::Bool(b) => builder.push(Some(b)),
                    v => {
                        return Err(anyhow!(
                            "unexpected output type from decode: {:?}",
                            v.as_type()
                        ))
                    }
                }
            }

            Ok(builder.as_box())
        }
        _ => {
            let mut builder = MutableBinaryArray::<i32>::new();

            for val in col.iter() {
                let val = val.context("found null value")?;

                match decoder.abi_decode(val).context("decode sol value")? {
                    DynSolValue::Int(v, _) => builder.push(Some(v.to_be_bytes::<32>())),
                    DynSolValue::Uint(v, _) => builder.push(Some(v.to_be_bytes::<32>())),
                    DynSolValue::FixedBytes(v, _) => builder.push(Some(v)),
                    DynSolValue::Address(v) => builder.push(Some(v)),
                    DynSolValue::Bytes(v) => builder.push(Some(v)),
                    DynSolValue::String(v) => builder.push(Some(v)),
                    v => {
                        return Err(anyhow!(
                            "unexpected output type from decode: {:?}",
                            v.as_type()
                        ))
                    }
                }
            }

            Ok(builder.as_box())
        }
    }
}

fn schema_from_event_signature(sig: &Option<alloy_json_abi::Event>) -> Result<Schema> {
    let sig = match sig {
        Some(sig) => sig,
        None => {
            return Ok(Schema::from(vec![Field::new(
                "dummy",
                DataType::Boolean,
                true,
            )]));
        }
    };

    let event = sig.resolve().context("resolve signature into event")?;

    let mut fields: Vec<Field> = Vec::with_capacity(sig.inputs.len());

    for (input, resolved_type) in sig
        .inputs
        .iter()
        .zip(event.indexed().iter().chain(event.body().iter()))
    {
        if input.name.is_empty() {
            return Err(anyhow!("empty param names are not supported"));
        }

        if fields
            .iter()
            .any(|f| f.name.as_str() == input.name.as_str())
        {
            return Err(anyhow!("duplicate param name: {}", input.name));
        }

        let ty = DynSolType::parse(&input.ty).context("parse solidity type")?;

        if &ty != resolved_type {
            return Err(anyhow!("Internal error: Parsed type doesn't match resolved type. This should never happen."));
        }

        let dt = simple_type_to_data_type(&ty).context("convert simple type to arrow datatype")?;
        fields.push(Field::new(input.name.clone(), dt, false));
    }

    Ok(Schema::from(fields))
}

fn simple_type_to_data_type(ty: &DynSolType) -> Result<DataType> {
    match ty {
        DynSolType::Bool => Ok(DataType::Boolean),
        DynSolType::Int(_) => Ok(DataType::Binary),
        DynSolType::Uint(_) => Ok(DataType::Binary),
        DynSolType::FixedBytes(_) => Ok(DataType::Binary),
        DynSolType::Address => Ok(DataType::Binary),
        DynSolType::Bytes => Ok(DataType::Binary),
        DynSolType::String => Ok(DataType::Binary),
        ty => Err(anyhow!(
            "Complex types are not supported. Unexpected type: {}",
            ty
        )),
    }
}
