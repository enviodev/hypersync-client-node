/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface StreamConfig {
  columnMapping?: ColumnMapping
  eventSignature?: string
  hexOutput?: HexOutput
  batchSize?: number
  maxBatchSize?: number
  minBatchSize?: number
  concurrency?: number
  maxNumBlocks?: number
  maxNumTransactions?: number
  maxNumLogs?: number
  maxNumTraces?: number
  responseBytesCeiling?: number
  responseBytesFloor?: number
  reverse?: boolean
}
export const enum HexOutput {
  NoEncode = 'NoEncode',
  Prefixed = 'Prefixed',
  NonPrefixed = 'NonPrefixed'
}
export const enum DataType {
  Float64 = 'Float64',
  Float32 = 'Float32',
  UInt64 = 'UInt64',
  UInt32 = 'UInt32',
  Int64 = 'Int64',
  Int32 = 'Int32'
}
export interface ColumnMapping {
  block?: Record<string, DataType>
  transaction?: Record<string, DataType>
  log?: Record<string, DataType>
  trace?: Record<string, DataType>
  decodedLog?: Record<string, DataType>
}
export interface ClientConfig {
  url?: string
  bearerToken?: string
  httpReqTimeoutMillis?: number
  maxNumRetries?: number
  retryBackoffMs?: number
  retryBaseMs?: number
  retryCeilingMs?: number
}
/**
 * Returns a query for all Blocks and Transactions within the block range (from_block, to_block]
 * If to_block is None then query runs to the head of the chain.
 */
export declare function presetQueryBlocksAndTransactions(fromBlock: number, toBlock?: number | undefined | null): Query
/**
 * Returns a query object for all Blocks and hashes of the Transactions within the block range
 * (from_block, to_block].  Also returns the block_hash and block_number fields on each Transaction
 * so it can be mapped to a block.  If to_block is None then query runs to the head of the chain.
 */
export declare function presetQueryBlocksAndTransactionHashes(fromBlock: number, toBlock?: number | undefined | null): Query
/**
 * Returns a query object for all Logs within the block range from the given address.
 * If to_block is None then query runs to the head of the chain.
 */
export declare function presetQueryLogs(contractAddress: string, fromBlock: number, toBlock?: number | undefined | null): Query
/**
 * Returns a query for all Logs within the block range from the given address with a
 * matching topic0 event signature.  Topic0 is the keccak256 hash of the event signature.
 * If to_block is None then query runs to the head of the chain.
 */
export declare function presetQueryLogsOfEvent(contractAddress: string, topic0: string, fromBlock: number, toBlock?: number | undefined | null): Query
export interface LogSelection {
  /**
   * Address of the contract, any logs that has any of these addresses will be returned.
   * Empty means match all.
   */
  address?: Array<string>
  /**
   * Topics to match, each member of the top level array is another array, if the nth topic matches any
   *  topic specified in topics[n] the log will be returned. Empty means match all.
   */
  topics?: Array<Array<string>>
}
export interface TransactionSelection {
  /**
   * Address the transaction should originate from. If transaction.from matches any of these, the transaction
   *  will be returned. Keep in mind that this has an and relationship with to filter, so each transaction should
   *  match both of them. Empty means match all.
   */
  from?: Array<string>
  /**
   * Address the transaction should go to. If transaction.to matches any of these, the transaction will
   *  be returned. Keep in mind that this has an and relationship with from filter, so each transaction should
   *  match both of them. Empty means match all.
   */
  to?: Array<string>
  /** If first 4 bytes of transaction input matches any of these, transaction will be returned. Empty means match all. */
  sighash?: Array<string>
  /** If tx.status matches this it will be returned. */
  status?: number
  /** If transaction.type matches any of these values, the transaction will be returned */
  kind?: Array<number>
  contractAddress?: Array<string>
}
export interface FieldSelection {
  block?: Array<string>
  transaction?: Array<string>
  log?: Array<string>
  trace?: Array<string>
}
export interface TraceSelection {
  from?: Array<string>
  to?: Array<string>
  address?: Array<string>
  callType?: Array<string>
  rewardType?: Array<string>
  kind?: Array<string>
  sighash?: Array<string>
}
export interface BlockSelection {
  /**
   * Hash of a block, any blocks that have one of these hashes will be returned.
   * Empty means match all.
   */
  hash?: Array<string>
  /**
   * Miner address of a block, any blocks that have one of these miners will be returned.
   * Empty means match all.
   */
  miner?: Array<string>
}
export const enum JoinMode {
  Default = 0,
  JoinAll = 1,
  JoinNothing = 2
}
export interface Query {
  /** The block to start the query from */
  fromBlock: number
  /**
   * The block to end the query at. If not specified, the query will go until the
   *  end of data. Exclusive, the returned range will be [from_block..to_block).
   *
   * The query will return before it reaches this target block if it hits the time limit
   *  configured on the server. The user should continue their query by putting the
   *  next_block field in the response into from_block field of their next query. This implements
   *  pagination.
   */
  toBlock?: number
  /**
   * List of log selections, these have an or relationship between them, so the query will return logs
   * that match any of these selections.
   */
  logs?: Array<LogSelection>
  /**
   * List of transaction selections, the query will return transactions that match any of these selections and
   *  it will return transactions that are related to the returned logs.
   */
  transactions?: Array<TransactionSelection>
  /**
   * List of trace selections, the query will return traces that match any of these selections and
   *  it will re turn traces that are related to the returned logs.
   */
  traces?: Array<TraceSelection>
  /** List of block selections, the query will return blocks that match any of these selections */
  blocks?: Array<BlockSelection>
  /**
   * Weather to include all blocks regardless of if they are related to a returned transaction or log. Normally
   *  the server will return only the blocks that are related to the transaction or logs in the response. But if this
   *  is set to true, the server will return data for all blocks in the requested range [from_block, to_block).
   */
  includeAllBlocks?: boolean
  /**
   * Field selection. The user can select which fields they are interested in, requesting less fields will improve
   *  query execution time and reduce the payload size so the user should always use a minimal number of fields.
   */
  fieldSelection: FieldSelection
  /**
   * Maximum number of blocks that should be returned, the server might return more blocks than this number but
   *  it won't overshoot by too much.
   */
  maxNumBlocks?: number
  /**
   * Maximum number of transactions that should be returned, the server might return more transactions than this number but
   *  it won't overshoot by too much.
   */
  maxNumTransactions?: number
  /**
   * Maximum number of logs that should be returned, the server might return more logs than this number but
   *  it won't overshoot by too much.
   */
  maxNumLogs?: number
  /**
   * Maximum number of traces that should be returned, the server might return more traces than this number but
   *  it won't overshoot by too much.
   */
  maxNumTraces?: number
  /**
   * Selects join mode for the query,
   * Default: join in this order logs -> transactions -> traces -> blocks
   * JoinAll: join everything to everything. For example if logSelection matches log0, we get the
   * associated transaction of log0 and then we get associated logs of that transaction as well. Applites similarly
   * to blocks, traces.
   * JoinNothing: join nothing.
   */
  joinMode?: JoinMode
}
/** Data relating to a single event (log) */
export interface Event {
  /** Transaction that triggered this event */
  transaction?: Transaction
  /** Block that this event happened in */
  block?: Block
  /** Evm log data */
  log: Log
}
/**
 * Evm log object
 *
 * See ethereum rpc spec for the meaning of fields
 */
export interface Log {
  removed?: boolean
  logIndex?: number
  transactionIndex?: number
  transactionHash?: string
  blockHash?: string
  blockNumber?: number
  address?: string
  data?: string
  topics: Array<string | undefined | null>
}
/**
 * Evm transaction object
 *
 * See ethereum rpc spec for the meaning of fields
 */
export interface Transaction {
  blockHash?: string
  blockNumber?: number
  from?: string
  gas?: string
  gasPrice?: string
  hash?: string
  input?: string
  nonce?: string
  to?: string
  transactionIndex?: number
  value?: string
  v?: string
  r?: string
  s?: string
  yParity?: string
  maxPriorityFeePerGas?: string
  maxFeePerGas?: string
  chainId?: number
  accessList?: Array<AccessList>
  maxFeePerBlobGas?: string
  blobVersionedHashes?: Array<string>
  cumulativeGasUsed?: string
  effectiveGasPrice?: string
  gasUsed?: string
  contractAddress?: string
  logsBloom?: string
  kind?: number
  root?: string
  status?: number
  l1Fee?: string
  l1GasPrice?: string
  l1GasUsed?: string
  l1FeeScalar?: number
  gasUsedForL1?: string
}
/**
 * Evm withdrawal object
 *
 * See ethereum rpc spec for the meaning of fields
 */
export interface Withdrawal {
  index?: string
  validatorIndex?: string
  address?: string
  amount?: string
}
/**
 * Evm access list object
 *
 * See ethereum rpc spec for the meaning of fields
 */
export interface AccessList {
  address?: string
  storageKeys?: Array<string>
}
/**
 * Evm block header object
 *
 * See ethereum rpc spec for the meaning of fields
 */
export interface Block {
  number?: number
  hash?: string
  parentHash?: string
  nonce?: string
  sha3Uncles?: string
  logsBloom?: string
  transactionsRoot?: string
  stateRoot?: string
  receiptsRoot?: string
  miner?: string
  difficulty?: string
  totalDifficulty?: string
  extraData?: string
  size?: string
  gasLimit?: string
  gasUsed?: string
  timestamp?: string
  uncles?: Array<string>
  baseFeePerGas?: string
  blobGasUsed?: string
  excessBlobGas?: string
  parentBeaconBlockRoot?: string
  withdrawalsRoot?: string
  withdrawals?: Array<Withdrawal>
  l1BlockNumber?: number
  sendCount?: string
  sendRoot?: string
  mixHash?: string
}
/**
 * Evm trace object
 *
 * See ethereum rpc spec for the meaning of fields
 */
export interface Trace {
  from?: string
  to?: string
  callType?: string
  gas?: string
  input?: string
  init?: string
  value?: string
  author?: string
  rewardType?: string
  blockHash?: string
  blockNumber?: number
  address?: string
  code?: string
  gasUsed?: string
  output?: string
  subtraces?: number
  traceAddress?: Array<number>
  transactionHash?: string
  transactionPosition?: number
  kind?: string
  error?: string
}
/** Decoded EVM log */
export interface DecodedEvent {
  indexed: Array<DecodedSolValue>
  body: Array<DecodedSolValue>
}
export interface DecodedSolValue {
  val: boolean | bigint | string | Array<DecodedSolValue>
}
export interface RollbackGuard {
  /** Block number of the last scanned block */
  blockNumber: number
  /** Block timestamp of the last scanned block */
  timestamp: number
  /** Block hash of the last scanned block */
  hash: string
  /**
   * Block number of the first scanned block in memory.
   *
   * This might not be the first scanned block. It only includes blocks that are in memory (possible to be rolled back).
   */
  firstBlockNumber: number
  /**
   * Parent hash of the first scanned block in memory.
   *
   * This might not be the first scanned block. It only includes blocks that are in memory (possible to be rolled back).
   */
  firstParentHash: string
}
export interface QueryResponseData {
  blocks: Array<Block>
  transactions: Array<Transaction>
  logs: Array<Log>
  traces: Array<Trace>
}
export interface QueryResponse {
  /** Current height of the source hypersync instance */
  archiveHeight?: number
  /**
   * Next block to query for, the responses are paginated so,
   *  the caller should continue the query from this block if they
   *  didn't get responses up to the to_block they specified in the Query.
   */
  nextBlock: number
  /** Total time it took the hypersync instance to execute the query. */
  totalExecutionTime: number
  /** Response data */
  data: QueryResponseData
  /** Rollback guard, supposed to be used to detect rollbacks */
  rollbackGuard?: RollbackGuard
}
export interface EventResponse {
  /** Current height of the source hypersync instance */
  archiveHeight?: number
  /**
   * Next block to query for, the responses are paginated so,
   *  the caller should continue the query from this block if they
   *  didn't get responses up to the to_block they specified in the Query.
   */
  nextBlock: number
  /** Total time it took the hypersync instance to execute the query. */
  totalExecutionTime: number
  /** Response data */
  data: Array<Event>
  /** Rollback guard, supposed to be used to detect rollbacks */
  rollbackGuard?: RollbackGuard
}
export interface Events {
  /** Current height of the source hypersync instance */
  archiveHeight?: number
  /**
   * Next block to query for, the responses are paginated so,
   *  the caller should continue the query from this block if they
   *  didn't get responses up to the to_block they specified in the Query.
   */
  nextBlock: number
  /** Total time it took the hypersync instance to execute the query. */
  totalExecutionTime: number
  /** Response data */
  events: Array<Event>
  /** Rollback guard, supposed to be used to detect rollbacks */
  rollbackGuard?: RollbackGuard
}
export class Decoder {
  static fromSignatures(signatures: Array<string>): Decoder
  enableChecksummedAddresses(): void
  disableChecksummedAddresses(): void
  decodeLogs(logs: Array<Log>): Promise<Array<DecodedEvent | undefined | null>>
  decodeLogsSync(logs: Array<Log>): Array<DecodedEvent | undefined | null>
  decodeEvents(events: Array<Event>): Promise<Array<DecodedEvent | undefined | null>>
  decodeEventsSync(events: Array<Event>): Array<DecodedEvent | undefined | null>
}
export class HypersyncClient {
  /** Create a new client with given config */
  static new(cfg?: ClientConfig | undefined | null): HypersyncClient
  /** Get the height of the source hypersync instance */
  getHeight(): Promise<number>
  collect(query: Query, config: StreamConfig): Promise<QueryResponse>
  collectEvents(query: Query, config: StreamConfig): Promise<EventResponse>
  collectParquet(path: string, query: Query, config: StreamConfig): Promise<void>
  get(query: Query): Promise<QueryResponse>
  getEvents(query: Query): Promise<EventResponse>
  stream(query: Query, config: StreamConfig): Promise<QueryResponseStream>
  streamEvents(query: Query, config: StreamConfig): Promise<EventStream>
}
export class QueryResponseStream {
  close(): Promise<void>
  recv(): Promise<QueryResponse | null>
}
export class EventStream {
  close(): Promise<void>
  recv(): Promise<EventResponse | null>
}
