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
export interface PropertyShared {
  'value' : CandyShared,
  'name' : string,
  'immutable' : boolean,
}
export interface _SERVICE {
  'candyArrayToJson' : ActorMethod<[], string>,
  'candyBlobToBlob' : ActorMethod<[], Uint8Array | number[]>,
  'candyBlobToJson' : ActorMethod<[], string>,
  'candyBoolToJson' : ActorMethod<[], string>,
  'candyBytesToBlob' : ActorMethod<[], Uint8Array | number[]>,
  'candyBytesToJson' : ActorMethod<[], string>,
  'candyClassToJson' : ActorMethod<[], string>,
  'candyFloatToBlob' : ActorMethod<[], Uint8Array | number[]>,
  'candyFloatToJson' : ActorMethod<[], string>,
  'candyFloatsToJson' : ActorMethod<[], string>,
  'candyInt16ToJson' : ActorMethod<[], string>,
  'candyInt32ToJson' : ActorMethod<[], string>,
  'candyInt64ToJson' : ActorMethod<[], string>,
  'candyInt8ToJson' : ActorMethod<[], string>,
  'candyIntToBlob' : ActorMethod<[], Uint8Array | number[]>,
  'candyIntToJson' : ActorMethod<[], string>,
  'candyIntsToJson' : ActorMethod<[], string>,
  'candyNat16ToBlob' : ActorMethod<[], Uint8Array | number[]>,
  'candyNat16ToJson' : ActorMethod<[], string>,
  'candyNat32ToBlob' : ActorMethod<[], Uint8Array | number[]>,
  'candyNat32ToJson' : ActorMethod<[], string>,
  'candyNat64ToBlob' : ActorMethod<[], Uint8Array | number[]>,
  'candyNat64ToJson' : ActorMethod<[], string>,
  'candyNat8ToBlob' : ActorMethod<[], Uint8Array | number[]>,
  'candyNat8ToJson' : ActorMethod<[], string>,
  'candyNatToBlob' : ActorMethod<[], Uint8Array | number[]>,
  'candyNatToJson' : ActorMethod<[], string>,
  'candyNatsToJson' : ActorMethod<[], string>,
  'candyOptionToJson' : ActorMethod<[], string>,
  'candyPrincipalToBlob' : ActorMethod<[], Uint8Array | number[]>,
  'candyPrincipalToJson' : ActorMethod<[], string>,
  'candySomeToJson' : ActorMethod<[], string>,
  'candyTextToBlob' : ActorMethod<[], Uint8Array | number[]>,
  'candyTextToJson' : ActorMethod<[], string>,
  'getArray' : ActorMethod<[], CandyShared>,
  'getBlob' : ActorMethod<[], CandyShared>,
  'getBool' : ActorMethod<[], CandyShared>,
  'getBytes' : ActorMethod<[], CandyShared>,
  'getCandyMap' : ActorMethod<[], CandyShared>,
  'getCandySet' : ActorMethod<[], CandyShared>,
  'getCandyValueMap' : ActorMethod<[], CandyShared>,
  'getClass' : ActorMethod<[], CandyShared>,
  'getFloat' : ActorMethod<[], CandyShared>,
  'getFloats' : ActorMethod<[], CandyShared>,
  'getInt' : ActorMethod<[], CandyShared>,
  'getInt16' : ActorMethod<[], CandyShared>,
  'getInt32' : ActorMethod<[], CandyShared>,
  'getInt64' : ActorMethod<[], CandyShared>,
  'getInt8' : ActorMethod<[], CandyShared>,
  'getInts' : ActorMethod<[], CandyShared>,
  'getNat' : ActorMethod<[], CandyShared>,
  'getNat16' : ActorMethod<[], CandyShared>,
  'getNat32' : ActorMethod<[], CandyShared>,
  'getNat64' : ActorMethod<[], CandyShared>,
  'getNat8' : ActorMethod<[], CandyShared>,
  'getNats' : ActorMethod<[], CandyShared>,
  'getNullOption' : ActorMethod<[], CandyShared>,
  'getOptSome' : ActorMethod<[], CandyShared>,
  'getPrincipal' : ActorMethod<[], CandyShared>,
  'getText' : ActorMethod<[], CandyShared>,
  'sizeOfCandyArray' : ActorMethod<[], bigint>,
  'sizeOfCandyBlob' : ActorMethod<[], bigint>,
  'sizeOfCandyBool' : ActorMethod<[], bigint>,
  'sizeOfCandyBytes' : ActorMethod<[], bigint>,
  'sizeOfCandyClass' : ActorMethod<[], bigint>,
  'sizeOfCandyFloat' : ActorMethod<[], bigint>,
  'sizeOfCandyFloats' : ActorMethod<[], bigint>,
  'sizeOfCandyInt' : ActorMethod<[], bigint>,
  'sizeOfCandyInt16' : ActorMethod<[], bigint>,
  'sizeOfCandyInt32' : ActorMethod<[], bigint>,
  'sizeOfCandyInt64' : ActorMethod<[], bigint>,
  'sizeOfCandyInt8' : ActorMethod<[], bigint>,
  'sizeOfCandyInts' : ActorMethod<[], bigint>,
  'sizeOfCandyNat' : ActorMethod<[], bigint>,
  'sizeOfCandyNat16' : ActorMethod<[], bigint>,
  'sizeOfCandyNat32' : ActorMethod<[], bigint>,
  'sizeOfCandyNat64' : ActorMethod<[], bigint>,
  'sizeOfCandyNat8' : ActorMethod<[], bigint>,
  'sizeOfCandyNats' : ActorMethod<[], bigint>,
  'sizeOfCandyOption' : ActorMethod<[], bigint>,
  'sizeOfCandyOptionNone' : ActorMethod<[], bigint>,
  'sizeOfCandyPrincipal' : ActorMethod<[], bigint>,
  'sizeOfCandySet' : ActorMethod<[], bigint>,
  'sizeOfCandyText' : ActorMethod<[], bigint>,
  'sizeOfCandyValueMap' : ActorMethod<[], bigint>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
