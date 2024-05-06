use std::collections::BTreeMap;

use anyhow::{anyhow, Context, Result};
use arrow2::array::{
    Array, BinaryArray, Float32Array, Float64Array, Int32Array, Int64Array, MutablePrimitiveArray,
    PrimitiveArray, UInt32Array, UInt64Array,
};
use arrow2::compute::cast;
use arrow2::datatypes::{DataType as ArrowDataType, Field, Schema};
use arrow2::types::NativeType;
use rayon::prelude::*;
use ruint::aliases::U256;
use serde::{Deserialize, Serialize};
use skar_schema::ArrowChunk;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ColumnMapping {
    #[serde(default)]
    pub block: BTreeMap<String, DataType>,
    #[serde(default)]
    pub transaction: BTreeMap<String, DataType>,
    #[serde(default)]
    pub log: BTreeMap<String, DataType>,
    #[serde(default)]
    pub trace: BTreeMap<String, DataType>,
    #[serde(default)]
    pub decoded_log: BTreeMap<String, DataType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Float64,
    Float32,
    UInt64,
    UInt32,
    Int64,
    Int32,
}

impl From<DataType> for ArrowDataType {
    fn from(value: DataType) -> Self {
        match value {
            DataType::Float64 => Self::Float64,
            DataType::Float32 => Self::Float32,
            DataType::UInt64 => Self::UInt64,
            DataType::UInt32 => Self::UInt32,
            DataType::Int64 => Self::Int64,
            DataType::Int32 => Self::Int32,
        }
    }
}

pub fn apply_to_chunk(
    chunk: &ArrowChunk,
    field_names: &[&str],
    mapping: &BTreeMap<String, DataType>,
) -> Result<ArrowChunk> {
    if mapping.is_empty() {
        return Ok(chunk.clone());
    }

    let columns = chunk
        .columns()
        .par_iter()
        .zip(field_names.par_iter())
        .map(|(col, &field_name)| {
            let col = match mapping.get(field_name) {
                Some(&dt) => map_column(&**col, dt)
                    .context(format!("apply cast to colum '{}'", field_name))?,
                None => col.clone(),
            };

            Ok(col)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(ArrowChunk::new(columns))
}

/// Warning: This function does not validate the mapping types!
/// So same mapping might fail if applied to actual data even if this function maps the schema normally.
pub fn apply_to_schema(schema: &Schema, mapping: &BTreeMap<String, DataType>) -> Result<Schema> {
    let fields = schema
        .fields
        .iter()
        .map(|field| match mapping.get(&field.name) {
            Some(&dt) => Field::new(&field.name, dt.into(), field.is_nullable),
            None => field.clone(),
        })
        .collect::<Vec<Field>>();

    Ok(Schema::from(fields))
}

pub fn map_column(col: &dyn Array, target_data_type: DataType) -> Result<Box<dyn Array + 'static>> {
    fn to_box<T: Array>(arr: T) -> Box<dyn Array> {
        Box::new(arr)
    }

    match target_data_type {
        DataType::Float64 => map_to_f64(col).map(to_box),
        DataType::Float32 => map_to_f32(col).map(to_box),
        DataType::UInt64 => map_to_uint64(col).map(to_box),
        DataType::UInt32 => map_to_uint32(col).map(to_box),
        DataType::Int64 => map_to_int64(col).map(to_box),
        DataType::Int32 => map_to_int32(col).map(to_box),
    }
}

fn map_to_f64(col: &dyn Array) -> Result<Float64Array> {
    match col.data_type() {
        &ArrowDataType::Binary => {
            binary_to_target_array(col.as_any().downcast_ref::<BinaryArray<i32>>().unwrap())
        }
        &ArrowDataType::UInt64 => Ok(cast::primitive_as_primitive(
            col.as_any().downcast_ref::<UInt64Array>().unwrap(),
            &ArrowDataType::Float64,
        )),
        dt => Err(anyhow!("Can't convert {:?} to f64", dt)),
    }
}

fn map_to_f32(col: &dyn Array) -> Result<Float32Array> {
    match col.data_type() {
        &ArrowDataType::Binary => {
            binary_to_target_array(col.as_any().downcast_ref::<BinaryArray<i32>>().unwrap())
        }
        &ArrowDataType::UInt64 => Ok(cast::primitive_as_primitive(
            col.as_any().downcast_ref::<UInt64Array>().unwrap(),
            &ArrowDataType::Float32,
        )),
        dt => Err(anyhow!("Can't convert {:?} to f32", dt)),
    }
}

fn map_to_uint64(col: &dyn Array) -> Result<UInt64Array> {
    match col.data_type() {
        &ArrowDataType::Binary => {
            binary_to_target_array(col.as_any().downcast_ref::<BinaryArray<i32>>().unwrap())
        }
        &ArrowDataType::UInt64 => Ok(cast::primitive_as_primitive(
            col.as_any().downcast_ref::<UInt64Array>().unwrap(),
            &ArrowDataType::UInt64,
        )),
        dt => Err(anyhow!("Can't convert {:?} to uint64", dt)),
    }
}

fn map_to_uint32(col: &dyn Array) -> Result<UInt32Array> {
    match col.data_type() {
        &ArrowDataType::Binary => {
            binary_to_target_array(col.as_any().downcast_ref::<BinaryArray<i32>>().unwrap())
        }
        &ArrowDataType::UInt64 => Ok(cast::primitive_as_primitive(
            col.as_any().downcast_ref::<UInt64Array>().unwrap(),
            &ArrowDataType::UInt32,
        )),
        dt => Err(anyhow!("Can't convert {:?} to uint32", dt)),
    }
}

fn map_to_int64(col: &dyn Array) -> Result<Int64Array> {
    match col.data_type() {
        &ArrowDataType::Binary => {
            binary_to_target_array(col.as_any().downcast_ref::<BinaryArray<i32>>().unwrap())
        }
        &ArrowDataType::UInt64 => Ok(cast::primitive_as_primitive(
            col.as_any().downcast_ref::<UInt64Array>().unwrap(),
            &ArrowDataType::Int64,
        )),
        dt => Err(anyhow!("Can't convert {:?} to int64", dt)),
    }
}

fn map_to_int32(col: &dyn Array) -> Result<Int32Array> {
    match col.data_type() {
        &ArrowDataType::Binary => {
            binary_to_target_array(col.as_any().downcast_ref::<BinaryArray<i32>>().unwrap())
        }
        &ArrowDataType::UInt64 => Ok(cast::primitive_as_primitive(
            col.as_any().downcast_ref::<UInt64Array>().unwrap(),
            &ArrowDataType::Int32,
        )),
        dt => Err(anyhow!("Can't convert {:?} to int32", dt)),
    }
}

fn binary_to_target_array<T: NativeType + TryFrom<U256>>(
    src: &BinaryArray<i32>,
) -> Result<PrimitiveArray<T>> {
    let mut out = MutablePrimitiveArray::with_capacity(src.len());

    for val in src.iter() {
        out.push(val.map(binary_to_target).transpose()?);
    }

    Ok(out.into())
}

fn binary_to_target<T: TryFrom<U256>>(src: &[u8]) -> Result<T> {
    let big_num = U256::from_be_slice(src);
    big_num
        .try_into()
        .map_err(|_e| anyhow!("failed to cast number to requested type"))
}
