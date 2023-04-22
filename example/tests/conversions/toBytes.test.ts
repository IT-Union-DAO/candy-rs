import { _SERVICE as RustCanister, CandyShared } from "dfx-gen/rust/rust.did";
import { _SERVICE as MotokoCanister } from "dfx-gen/motoko/motoko.did";
import { createMotokoActor, createRustActor } from "../utils";
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

describe("CandyShared to Blob conversion should:", () => {
  let rustCanister: RustCanister;
  let motokoCanister: MotokoCanister;

  beforeAll(async () => {
    rustCanister = createRustActor();
    motokoCanister = createMotokoActor();
  });

  test("produce equal results for CandyShared.Blob", async () => {
    const rustBlob = await rustCanister.candy_blob_to_blob();
    const motokoBlob = await motokoCanister.candyBlobToBlob();
    expect(rustBlob).toEqual(motokoBlob);
    expect(rustBlob).toEqual([0, 1, 2, 3, 4, 5, 6, 7]);
  });

  test("produce equal results for CandyShared.Bytes", async () => {
    const rustBlob = await rustCanister.candy_bytes_to_blob();
    const motokoBlob = await motokoCanister.candyBytesToBlob();
    expect(rustBlob).toEqual(motokoBlob);
    expect(rustBlob).toEqual([0, 1, 2, 3, 4, 5, 6, 7]);
  });

  //Text
  test("produce equal results for CandyShared.Text", async () => {
    const rustBlob = await rustCanister.candy_text_to_blob();
    const motokoBlob = await motokoCanister.candyTextToBlob();
    expect(rustBlob).toEqual(motokoBlob);
  });

  //Nat
  test("produce equal results for CandyShared.Nat", async () => {
    const rustBlob = await rustCanister.candy_nat_to_blob();
    const motokoBlob = await motokoCanister.candyNatToBlob();
    expect(rustBlob).toEqual(motokoBlob);
    expect(rustBlob).toEqual([73, 150, 2, 210]);
  });

  //Int
  test("produce equal results for CandyShared.Int", async () => {
    const rustBlob = await rustCanister.candy_int_to_blob();
    const motokoBlob = await motokoCanister.candyIntToBlob();
    expect(rustBlob).toEqual(motokoBlob);
  });

  //Nat8
  test("produce equal results for CandyShared.Nat8", async () => {
    const rustBlob = await rustCanister.candy_nat8_to_blob();
    const motokoBlob = await motokoCanister.candyNat8ToBlob();
    expect(rustBlob).toEqual(motokoBlob);
    expect(rustBlob).toEqual([255]);
  });

  //Nat16
  test("produce equal results for CandyShared.Nat16", async () => {
    const rustBlob = await rustCanister.candy_nat16_to_blob();
    const motokoBlob = await motokoCanister.candyNat16ToBlob();
    expect(rustBlob).toEqual(motokoBlob);
    expect(rustBlob).toEqual([255, 255]);
  });

  //Nat32
  test("produce equal results for CandyShared.Nat32", async () => {
    const rustBlob = await rustCanister.candy_nat32_to_blob();
    const motokoBlob = await motokoCanister.candyNat32ToBlob();
    expect(rustBlob).toEqual(motokoBlob);
    expect(rustBlob).toEqual([7, 91, 205, 21]);
  });

  //Nat64
  test("produce equal results for CandyShared.Nat64", async () => {
    const rustBlob = await rustCanister.candy_nat64_to_blob();
    const motokoBlob = await motokoCanister.candyNat64ToBlob();
    expect(rustBlob).toEqual(motokoBlob);
    expect(rustBlob).toEqual([127, 255, 255, 255, 255, 255, 255, 255]);
  });

  //Principal
  test("produce equal results for CandyShared.Principal", async () => {
    const rustBlob = await rustCanister.candy_principal_to_blob();
    const motokoBlob = await motokoCanister.candyPrincipalToBlob();
    expect(rustBlob).toEqual(motokoBlob);
    expect(rustBlob).toEqual([]);
  });

  //Float
  test("trap for CandyShared.Float", async () => {
    let rustError;
    let motokoError;
    try {
      const rustBlob = await rustCanister.candy_float_to_blob();
    } catch (e) {
      rustError = e;
    }
    try {
      const motokoBlob = await motokoCanister.candyFloatToBlob();
    } catch (e) {
      motokoError = e;
    }

    expect(rustError).toBeDefined();
    expect(motokoError).toBeDefined();
  });
});
