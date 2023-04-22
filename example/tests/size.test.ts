import { _SERVICE as MotokoCanister } from "dfx-gen/candyFunctions/candyFunctions.did";
import { _SERVICE as RustCanister } from "dfx-gen/rust/rust.did";
import { createMotokoActor, createRustActor } from "./utils";

describe("CandyShared value size of rust and motoko libraries should be equal for", () => {
  let rustCanister: RustCanister;
  let motokoCanister: MotokoCanister;

  beforeAll(async () => {
    rustCanister = createRustActor();
    motokoCanister = createMotokoActor();
  });

  test("Int", async () => {
    const rustSize = await rustCanister.size_of_candy_int();
    const motokoSize = await motokoCanister.sizeOfCandyInt();
    expect(rustSize).toBe(motokoSize);
  });

  //Int8
  test("Int8", async () => {
    const rustSize = await rustCanister.size_of_candy_int8();
    const motokoSize = await motokoCanister.sizeOfCandyInt8();
    expect(rustSize).toBe(motokoSize);
  });

  //Int16
  test("Int16", async () => {
    const rustSize = await rustCanister.size_of_candy_int16();
    const motokoSize = await motokoCanister.sizeOfCandyInt16();
    expect(rustSize).toBe(motokoSize);
  });

  //Int32
  test("Int32", async () => {
    const rustSize = await rustCanister.size_of_candy_int32();
    const motokoSize = await motokoCanister.sizeOfCandyInt32();
    expect(rustSize).toBe(motokoSize);
  });

  //Int64
  test("Int64", async () => {
    const rustSize = await rustCanister.size_of_candy_int64();
    const motokoSize = await motokoCanister.sizeOfCandyInt64();
    expect(rustSize).toBe(motokoSize);
  });

  test("Nat", async () => {
    const rustSize = await rustCanister.size_of_candy_nat();
    const motokoSize = await motokoCanister.sizeOfCandyNat();
    expect(rustSize).toBe(motokoSize);
  });

  //Nat8
  test("Nat8", async () => {
    const rustSize = await rustCanister.size_of_candy_nat8();
    const motokoSize = await motokoCanister.sizeOfCandyNat8();
    expect(rustSize).toBe(motokoSize);
  });

  //Nat16
  test("Nat16", async () => {
    const rustSize = await rustCanister.size_of_candy_nat16();
    const motokoSize = await motokoCanister.sizeOfCandyNat16();
    expect(rustSize).toBe(motokoSize);
  });

  //Nat32
  test("Nat32", async () => {
    const rustSize = await rustCanister.size_of_candy_nat32();
    const motokoSize = await motokoCanister.sizeOfCandyNat32();
    expect(rustSize).toBe(motokoSize);
  });

  //Nat64
  test("Nat64", async () => {
    const rustSize = await rustCanister.size_of_candy_nat64();
    const motokoSize = await motokoCanister.sizeOfCandyNat64();
    expect(rustSize).toBe(motokoSize);
  });

  //Float
  test("Float", async () => {
    const rustSize = await rustCanister.size_of_candy_float();
    const motokoSize = await motokoCanister.sizeOfCandyFloat();
    expect(rustSize).toBe(motokoSize);
  });

  //Text
  test("Text", async () => {
    const rustSize = await rustCanister.size_of_candy_text();
    const motokoSize = await motokoCanister.sizeOfCandyText();
    expect(rustSize).toBe(motokoSize);
  });

  //Bool
  test("Bool", async () => {
    const rustSize = await rustCanister.size_of_candy_bool();
    const motokoSize = await motokoCanister.sizeOfCandyBool();
    expect(rustSize).toBe(motokoSize);
  });

  //Blob
  test("Blob", async () => {
    const rustSize = await rustCanister.size_of_candy_blob();
    const motokoSize = await motokoCanister.sizeOfCandyBlob();
    expect(rustSize).toBe(motokoSize);
  });

  //Class
  test("Class", async () => {
    const rustSize = await rustCanister.size_of_candy_class();
    const motokoSize = await motokoCanister.sizeOfCandyClass();
    expect(rustSize).toBe(motokoSize);
  });

  //Principal
  test("Principal", async () => {
    const rustSize = await rustCanister.size_of_candy_principal();
    const motokoSize = await motokoCanister.sizeOfCandyPrincipal();
    expect(rustSize).toBe(motokoSize);
  });

  //Option
  test("Option", async () => {
    const rustSize = await rustCanister.size_of_candy_option();
    const motokoSize = await motokoCanister.sizeOfCandyOption();
    expect(rustSize).toBe(motokoSize);
  });

  //Option None
  test("Option None", async () => {
    const rustSize = await rustCanister.size_of_candy_option_none();
    const motokoSize = await motokoCanister.sizeOfCandyOptionNone();
    expect(rustSize).toBe(motokoSize);
  });

  //Array
  test("Array", async () => {
    const rustSize = await rustCanister.size_of_candy_array();
    const motokoSize = await motokoCanister.sizeOfCandyArray();
    expect(rustSize).toBe(motokoSize);
  });

  //Bytes
  test("Bytes", async () => {
    const rustSize = await rustCanister.size_of_candy_bytes();
    const motokoSize = await motokoCanister.sizeOfCandyBytes();
    expect(rustSize).toBe(motokoSize);
  });

  //Floats
  test("Floats", async () => {
    const rustSize = await rustCanister.size_of_candy_floats();
    const motokoSize = await motokoCanister.sizeOfCandyFloats();
    expect(rustSize).toBe(motokoSize);
  });

  //Nats
  test("Nats", async () => {
    const rustSize = await rustCanister.size_of_candy_nats();
    const motokoSize = await motokoCanister.sizeOfCandyNats();
    expect(rustSize).toBe(motokoSize);
  });

  //Ints
  test("Ints", async () => {
    const rustSize = await rustCanister.size_of_candy_ints();
    const motokoSize = await motokoCanister.sizeOfCandyInts();
    expect(rustSize).toBe(motokoSize);
  });

  //Map
  test("Map", async () => {
    const rustSize = await rustCanister.size_of_candy_map();
    const motokoSize = await motokoCanister.sizeOfCandyMap();
    expect(rustSize).toBe(motokoSize);
  });

  //Set
  test("Set", async () => {
    const rustSize = await rustCanister.size_of_candy_set();
    const motokoSize = await motokoCanister.sizeOfCandySet();
    expect(rustSize).toBe(motokoSize);
  });
});
