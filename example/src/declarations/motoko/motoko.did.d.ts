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
  { 'Nats' : { 'thawed' : Array<bigint> } | { 'frozen' : Array<bigint> } } |
  { 'Text' : string } |
  { 'Bytes' : { 'thawed' : Uint8Array } | { 'frozen' : Uint8Array } } |
  { 'Int16' : number } |
  { 'Int32' : number } |
  { 'Int64' : bigint } |
  { 'Option' : [] | [CandyValue] } |
  { 'Floats' : { 'thawed' : Array<number> } | { 'frozen' : Array<number> } } |
  { 'Float' : number } |
  { 'Principal' : Principal } |
  {
    'Array' : { 'thawed' : Array<CandyValue> } |
      { 'frozen' : Array<CandyValue> }
  } |
  { 'Class' : Array<Property> };
export interface Property {
  'value' : CandyValue,
  'name' : string,
  'immutable' : boolean,
}
export interface _SERVICE {
  'getBlob' : ActorMethod<[], CandyValue>,
  'getBool' : ActorMethod<[], CandyValue>,
  'getClass' : ActorMethod<[], CandyValue>,
  'getEmpty' : ActorMethod<[], CandyValue>,
  'getFloat' : ActorMethod<[], CandyValue>,
  'getFrozenArray' : ActorMethod<[], CandyValue>,
  'getFrozenBytes' : ActorMethod<[], CandyValue>,
  'getFrozenFloats' : ActorMethod<[], CandyValue>,
  'getFrozenNats' : ActorMethod<[], CandyValue>,
  'getFrozenText' : ActorMethod<[], CandyValue>,
  'getInt' : ActorMethod<[], CandyValue>,
  'getInt16' : ActorMethod<[], CandyValue>,
  'getInt32' : ActorMethod<[], CandyValue>,
  'getInt64' : ActorMethod<[], CandyValue>,
  'getInt8' : ActorMethod<[], CandyValue>,
  'getNat' : ActorMethod<[], CandyValue>,
  'getNat16' : ActorMethod<[], CandyValue>,
  'getNat32' : ActorMethod<[], CandyValue>,
  'getNat64' : ActorMethod<[], CandyValue>,
  'getNat8' : ActorMethod<[], CandyValue>,
  'getNullOption' : ActorMethod<[], CandyValue>,
  'getOptSome' : ActorMethod<[], CandyValue>,
  'getPrincipal' : ActorMethod<[], CandyValue>,
  'getText' : ActorMethod<[], CandyValue>,
  'getThawedArray' : ActorMethod<[], CandyValue>,
  'getThawedBytes' : ActorMethod<[], CandyValue>,
  'getThawedFloats' : ActorMethod<[], CandyValue>,
  'getThawedNats' : ActorMethod<[], CandyValue>,
}
