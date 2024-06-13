import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type CandyShared = { 'Int' : bigint } |
  { 'Map' : Array<[string, CandyShared]> } |
  { 'Nat' : bigint } |
  { 'Set' : Array<CandyShared> } |
  { 'Nat16' : number } |
  { 'Nat32' : number } |
  { 'Nat64' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Bool' : boolean } |
  { 'Int8' : number } |
  { 'Ints' : Array<bigint> } |
  { 'Nat8' : number } |
  { 'Nats' : Array<bigint> } |
  { 'Text' : string } |
  { 'Bytes' : Uint8Array | number[] } |
  { 'Int16' : number } |
  { 'Int32' : number } |
  { 'Int64' : bigint } |
  { 'Option' : [] | [CandyShared] } |
  { 'Floats' : Array<number> } |
  { 'Float' : number } |
  { 'Principal' : Principal } |
  { 'Array' : Array<CandyShared> } |
  { 'ValueMap' : Array<[CandyShared, CandyShared]> } |
  { 'Class' : Array<PropertyShared> };
export type ChunkingType = { 'eof' : null } |
  { 'chunk' : null };
export interface PropertyShared {
  'value' : CandyShared,
  'name' : string,
  'immutable' : boolean,
}
export interface _SERVICE {
  '__get_candid' : ActorMethod<[], string>,
  'candy_array_to_json' : ActorMethod<[], string>,
  'candy_blob_to_blob' : ActorMethod<[], Uint8Array | number[]>,
  'candy_blob_to_json' : ActorMethod<[], string>,
  'candy_bool_to_json' : ActorMethod<[], string>,
  'candy_bytes_to_blob' : ActorMethod<[], Uint8Array | number[]>,
  'candy_bytes_to_json' : ActorMethod<[], string>,
  'candy_class_to_json' : ActorMethod<[], string>,
  'candy_float_to_blob' : ActorMethod<[], Uint8Array | number[]>,
  'candy_float_to_json' : ActorMethod<[], string>,
  'candy_floats_to_json' : ActorMethod<[], string>,
  'candy_int16_to_json' : ActorMethod<[], string>,
  'candy_int32_to_json' : ActorMethod<[], string>,
  'candy_int64_to_json' : ActorMethod<[], string>,
  'candy_int8_to_json' : ActorMethod<[], string>,
  'candy_int_to_blob' : ActorMethod<[], Uint8Array | number[]>,
  'candy_int_to_json' : ActorMethod<[], string>,
  'candy_ints_to_json' : ActorMethod<[], string>,
  'candy_nat16_to_blob' : ActorMethod<[], Uint8Array | number[]>,
  'candy_nat16_to_json' : ActorMethod<[], string>,
  'candy_nat32_to_blob' : ActorMethod<[], Uint8Array | number[]>,
  'candy_nat32_to_json' : ActorMethod<[], string>,
  'candy_nat64_to_blob' : ActorMethod<[], Uint8Array | number[]>,
  'candy_nat64_to_json' : ActorMethod<[], string>,
  'candy_nat8_to_blob' : ActorMethod<[], Uint8Array | number[]>,
  'candy_nat8_to_json' : ActorMethod<[], string>,
  'candy_nat_to_blob' : ActorMethod<[], Uint8Array | number[]>,
  'candy_nat_to_json' : ActorMethod<[], string>,
  'candy_nats_to_json' : ActorMethod<[], string>,
  'candy_option_some_to_json' : ActorMethod<[], string>,
  'candy_option_to_json' : ActorMethod<[], string>,
  'candy_principal_to_blob' : ActorMethod<[], Uint8Array | number[]>,
  'candy_principal_to_json' : ActorMethod<[], string>,
  'candy_text_to_blob' : ActorMethod<[], Uint8Array | number[]>,
  'candy_text_to_json' : ActorMethod<[], string>,
  'count_chunks' : ActorMethod<[Array<[bigint, bigint, CandyShared]>], bigint>,
  'from_buffer' : ActorMethod<[], Array<CandyShared>>,
  'get_candy' : ActorMethod<[string, string], CandyShared>,
  'get_chunk_array_size' : ActorMethod<[], bigint>,
  'get_data_chunk_from_addressed_chunk_array' : ActorMethod<
    [bigint, bigint],
    CandyShared
  >,
  'get_data_zone_size' : ActorMethod<[], bigint>,
  'get_flatten_chunk_array' : ActorMethod<[], Uint8Array | number[]>,
  'get_workspace_chunk' : ActorMethod<
    [Array<[bigint, bigint, CandyShared]>, bigint, bigint],
    [ChunkingType, Array<[bigint, bigint, CandyShared]>]
  >,
  'get_ws_chunk_size' : ActorMethod<
    [Array<[bigint, bigint, CandyShared]>, bigint],
    bigint
  >,
  'size_of_candy_array' : ActorMethod<[], bigint>,
  'size_of_candy_blob' : ActorMethod<[], bigint>,
  'size_of_candy_bool' : ActorMethod<[], bigint>,
  'size_of_candy_bytes' : ActorMethod<[], bigint>,
  'size_of_candy_class' : ActorMethod<[], bigint>,
  'size_of_candy_float' : ActorMethod<[], bigint>,
  'size_of_candy_floats' : ActorMethod<[], bigint>,
  'size_of_candy_int' : ActorMethod<[], bigint>,
  'size_of_candy_int16' : ActorMethod<[], bigint>,
  'size_of_candy_int32' : ActorMethod<[], bigint>,
  'size_of_candy_int64' : ActorMethod<[], bigint>,
  'size_of_candy_int8' : ActorMethod<[], bigint>,
  'size_of_candy_ints' : ActorMethod<[], bigint>,
  'size_of_candy_map' : ActorMethod<[], bigint>,
  'size_of_candy_nat' : ActorMethod<[], bigint>,
  'size_of_candy_nat16' : ActorMethod<[], bigint>,
  'size_of_candy_nat32' : ActorMethod<[], bigint>,
  'size_of_candy_nat64' : ActorMethod<[], bigint>,
  'size_of_candy_nat8' : ActorMethod<[], bigint>,
  'size_of_candy_nats' : ActorMethod<[], bigint>,
  'size_of_candy_option' : ActorMethod<[], bigint>,
  'size_of_candy_option_none' : ActorMethod<[], bigint>,
  'size_of_candy_principal' : ActorMethod<[], bigint>,
  'size_of_candy_set' : ActorMethod<[], bigint>,
  'size_of_candy_text' : ActorMethod<[], bigint>,
  'test_chunk_array' : ActorMethod<[], Array<[bigint, bigint, CandyShared]>>,
  'to_bytes_buffer' : ActorMethod<[], Array<Uint8Array | number[]>>,
  'to_workspace_and_back' : ActorMethod<
    [Array<[bigint, bigint, CandyShared]>],
    Array<[bigint, bigint, CandyShared]>
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
