import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Array = { 'thawed' : Array<CandyValue> } |
  { 'frozen' : Array<CandyValue> };
export type Bytes = { 'thawed' : Uint8Array } |
  { 'frozen' : Uint8Array };
export type CandyValue = { 'Int' : bigint } |
  { 'Map' : Array<[string, CandyValue]> } |
  { 'Nat' : bigint } |
  { 'Empty' : null } |
  { 'Nat16' : number } |
  { 'Nat32' : number } |
  { 'Nat64' : bigint } |
  { 'Blob' : Uint8Array } |
  { 'Bool' : boolean } |
  { 'Int8' : number } |
  { 'Nat8' : number } |
  { 'Nats' : Nats } |
  { 'Text' : string } |
  { 'Bytes' : Bytes } |
  { 'Int16' : number } |
  { 'Int32' : number } |
  { 'Int64' : bigint } |
  { 'Option' : [] | [CandyValue] } |
  { 'Floats' : Floats } |
  { 'Float' : number } |
  { 'Principal' : Principal } |
  { 'Array' : Array } |
  { 'Class' : Array<Property> };
export type ChunkingType = { 'eof' : null } |
  { 'chunk' : null };
export type Floats = { 'thawed' : Array<number> } |
  { 'frozen' : Array<number> };
export type Nats = { 'thawed' : Array<bigint> } |
  { 'frozen' : Array<bigint> };
export interface Property {
  'value' : CandyValue,
  'name' : string,
  'immutable' : boolean,
}
export interface _SERVICE {
  'get_blob' : ActorMethod<[], CandyValue>,
  'get_bool' : ActorMethod<[], CandyValue>,
  'get_type' : ActorMethod<[], ChunkingType>,
}
