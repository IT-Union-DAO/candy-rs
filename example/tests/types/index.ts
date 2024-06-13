import {Principal} from "@dfinity/principal";
import {CandyShared, PropertyShared} from "dfx-gen/motoko/motoko.did";

export interface CandyInt {
    Int: bigint;
}

export interface CandyValueMap {
    ValueMap: Array<[CandyShared, CandyShared]>
}

export interface CandyMap {
    Map: Array<[string, CandyShared]>;
}

export interface CandySet {
    Set: Array<CandyShared>;
}

export interface CandyNat {
    Nat: bigint;
}

export interface CandyNat16 {
    Nat16: number;
}

export interface CandyNat32 {
    Nat32: number;
}

export interface CandyNat64 {
    Nat64: bigint;
}

export interface CandyBlob {
    Blob: Uint8Array | number[];
}

export interface CandyBool {
    Bool: boolean;
}

export interface CandyInt8 {
    Int8: number;
}

export interface CandyInt16 {
    Int16: number;
}

export interface CandyInt32 {
    Int32: number;
}

export interface CandyInt64 {
    Int64: bigint;
}

export interface CandyInts {
    Ints: Array<bigint>;
}

export interface CandyNat8 {
    Nat8: number;
}

export interface CandyNats {
    Nats: Array<bigint>;
}

export interface CandyText {
    Text: string;
}

export interface CandyBytes {
    Bytes: Uint8Array | number[];
}

export interface CandyOption {
    Option: [] | [CandyShared];
}

export interface CandyFloats {
    Floats: Array<number>;
}

export interface CandyFloat {
    Float: number;
}

export interface CandyPrincipal {
    Principal: Principal;
}

export interface CandyArray {
    Array: Array<CandyShared>;
}

export interface CandyClass {
    Class: Array<PropertyShared>;
}