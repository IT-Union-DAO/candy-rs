import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type CandyValue = { 'Int' : bigint } |
  { 'Nat' : bigint } |
  { 'Empty' : null } |
  { 'Nat16' : number } |
  { 'Nat32' : number } |
  { 'Nat64' : bigint } |
  { 'Blob' : Uint8Array } |
  { 'Bool' : boolean } |
  { 'Int8' : number } |
  { 'Nat8' : number } |
  { 'Nats' : Array<bigint> } |
  { 'Text' : string } |
  { 'Int16' : number } |
  { 'Int32' : number } |
  { 'Int64' : bigint } |
  { 'Option' : [] | [CandyValue] } |
  { 'Floats' : Array<number> } |
  { 'Float' : number } |
  { 'Principal' : Principal } |
  { 'Array' : Array<CandyValue> } |
  { 'Class' : Array<Property> };
export interface Property {
  'value' : CandyValue,
  'name' : string,
  'immutable' : boolean,
}
export interface _SERVICE {
  'get_blob' : ActorMethod<[], CandyValue>,
  'get_bool' : ActorMethod<[], CandyValue>,
}
