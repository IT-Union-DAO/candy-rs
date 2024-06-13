import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type AddressedChunk = [bigint, bigint, CandyShared];
export type AddressedChunkArray = Array<AddressedChunk>;
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
  'countChunks' : ActorMethod<[AddressedChunkArray], bigint>,
  'fromBuffer' : ActorMethod<[], Array<CandyShared>>,
  'getChunkArraySize' : ActorMethod<[], bigint>,
  'getDataChunkFromAddressedChunkArray' : ActorMethod<
    [bigint, bigint],
    CandyShared
  >,
  'getDataZoneSize' : ActorMethod<[], bigint>,
  'getFlattenChunkedArray' : ActorMethod<[], Uint8Array | number[]>,
  'getWorkspaceChunk' : ActorMethod<
    [AddressedChunkArray, bigint, bigint],
    [{ 'eof' : null } | { 'chunk' : null }, AddressedChunkArray]
  >,
  'getWorkspaceChunkSize' : ActorMethod<[AddressedChunkArray, bigint], bigint>,
  'testChunkArray' : ActorMethod<[], AddressedChunkArray>,
  'toBytesBuffer' : ActorMethod<[], Array<Uint8Array | number[]>>,
  'toWorkspaceAndBack' : ActorMethod<
    [AddressedChunkArray],
    AddressedChunkArray
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
