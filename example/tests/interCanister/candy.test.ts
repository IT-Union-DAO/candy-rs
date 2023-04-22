import { _SERVICE as RustCaller, CandyShared } from "dfx-gen/rust/rust.did";
import { createRustActor } from "../utils";
import {
  CandyBlob,
  CandyBool,
  CandyBytes,
  CandyFloat,
  CandyInt,
  CandyInt16,
  CandyInt32,
  CandyInt64,
  CandyInt8,
  CandyInts,
  CandyMap,
  CandyNat,
  CandyNat16,
  CandyNat32,
  CandyNat64,
  CandyNat8,
  CandyNats,
  CandyOption,
  CandySet,
  CandyText,
} from "./../types/index";

import canister_ids from "../../.dfx/local/canister_ids.json";

describe("when calling Motoko canister from Rust canister it", () => {
  let rustCaller: RustCaller;

  beforeAll(async () => {
    rustCaller = createRustActor();
  });

  test("should accept Nat ", async () => {
    const candyNat: CandyNat = await interCanisterCallByRust<CandyNat>(
      "getNat"
    );
    expect(candyNat.Nat).toEqual(15n);
  });

  test("should accept Nat8 ", async () => {
    const candyNat8: CandyNat8 = await interCanisterCallByRust<CandyNat8>(
      "getNat8"
    );
    expect(candyNat8.Nat8).toEqual(15);
  });

  test("should accept Nat16 ", async () => {
    const candyNat16: CandyNat16 = await interCanisterCallByRust<CandyNat16>(
      "getNat16"
    );
    expect(candyNat16.Nat16).toEqual(15);
  });

  test("should accept Nat32 ", async () => {
    const candyNat32: CandyNat32 = await interCanisterCallByRust<CandyNat32>(
      "getNat32"
    );
    expect(candyNat32.Nat32).toEqual(15);
  });

  test("should accept Nat64 ", async () => {
    const candyNat64: CandyNat64 = await interCanisterCallByRust<CandyNat64>(
      "getNat64"
    );
    expect(candyNat64.Nat64).toEqual(15n);
  });

  test("should accept Blob ", async () => {
    const candyBlob: CandyBlob = await interCanisterCallByRust<CandyBlob>(
      "getBlob"
    );
    expect(candyBlob.Blob).toEqual([0, 0, 0, 0, 0, 48, 0, 211, 1, 1]);
  });

  test("should accept Bytes ", async () => {
    const candyBytes: CandyBytes = await interCanisterCallByRust<CandyBytes>(
      "getBytes"
    );
    expect(candyBytes.Bytes).toEqual([]);
  });

  test("should accept Bool ", async () => {
    const candyBool: CandyBool = await interCanisterCallByRust<CandyBool>(
      "getBool"
    );
    expect(candyBool.Bool).toEqual(true);
  });

  test("should accept Int ", async () => {
    const candyInt: CandyInt = await interCanisterCallByRust<CandyInt>(
      "getInt"
    );
    expect(candyInt.Int).toEqual(15n);
  });

  test("should accept Int8 ", async () => {
    const candyInt8: CandyInt8 = await interCanisterCallByRust<CandyInt8>(
      "getInt8"
    );
    expect(candyInt8.Int8).toEqual(15);
  });

  test("should accept Int16 ", async () => {
    const candyInt16: CandyInt16 = await interCanisterCallByRust<CandyInt16>(
      "getInt16"
    );
    expect(candyInt16.Int16).toEqual(15);
  });

  test("should accept Int32 ", async () => {
    const candyInt32: CandyInt32 = await interCanisterCallByRust<CandyInt32>(
      "getInt32"
    );
    expect(candyInt32.Int32).toEqual(15);
  });

  test("should accept Int64 ", async () => {
    const candyInt64: CandyInt64 = await interCanisterCallByRust<CandyInt64>(
      "getInt64"
    );
    expect(candyInt64.Int64).toEqual(15n);
  });

  test("should accept Ints ", async () => {
    const candyInts: CandyInts = await interCanisterCallByRust<CandyInts>(
      "getInts"
    );
    expect(candyInts.Ints).toEqual([1n, 2n, 3n, 4n]);
  });

  test("should accept Nats ", async () => {
    const candyNats: CandyNats = await interCanisterCallByRust<CandyNats>(
      "getNats"
    );
    expect(candyNats.Nats).toEqual([1n, 2n, 3n, 4n]);
  });

  test("should accept Text ", async () => {
    const candyText: CandyText = await interCanisterCallByRust<CandyText>(
      "getText"
    );
    expect(candyText.Text).toEqual("Hello, world!");
  });

  //Map
  test("should accept Map ", async () => {
    const candyMap: CandyMap = await interCanisterCallByRust<CandyMap>(
      "getCandyMap"
    );
    expect(candyMap.Map).toEqual([[{ Nat: 15n }, { Nat: 15n }]]);
  });

  //Set
  test("should accept Set ", async () => {
    const candySet: CandySet = await interCanisterCallByRust<CandySet>(
      "getCandySet"
    );
    expect(candySet.Set).toEqual([{ Nat: 15n }]);
  });

  //Float
  test("should accept Float ", async () => {
    const candyFloat: CandyFloat = await interCanisterCallByRust<CandyFloat>(
      "getFloat"
    );
    expect(candyFloat.Float).toEqual(15.0);
  });

  //Option
  test("should accept Option ", async () => {
    const candyOption: CandyOption = await interCanisterCallByRust<CandyOption>(
      "getOptSome"
    );
    expect(candyOption.Option).toContainEqual({ Nat: 15n });

    const candyOptionNone: CandyOption =
      await interCanisterCallByRust<CandyOption>("getNullOption");
    expect(candyOptionNone.Option).toEqual([]);
  });

  // function to make a inter canister call to get a CandyShared type
  const interCanisterCallByRust = async <T>(methodName: string) => {
    const val: CandyShared = await rustCaller.get_candy(
      canister_ids.motoko_callee.local,
      methodName
    );
    return val as T;
  };
});
