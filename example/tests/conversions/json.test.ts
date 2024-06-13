import {_SERVICE as RustCanister} from "dfx-gen/rust/rust.did";
import {_SERVICE as MotokoCanister} from "dfx-gen/candyFunctions/candyFunctions.did";
import {createMotokoActor, createRustActor} from "../utils";

describe("CandyShared to JSON conversion:", () => {
    let rustCanister: RustCanister;
    let motokoCanister: MotokoCanister;

    beforeAll(async () => {
        rustCanister = createRustActor();
        motokoCanister = createMotokoActor();
    });

    test("produce equal results for CandyShared.Nat", async () => {
        const rustJson = await rustCanister.candy_nat_to_json();
        const motokoJson = await motokoCanister.candyNatToJson();
        expect(rustJson).toEqual("9_223_372_036_854_775_807");
        expect(motokoJson).toEqual("9223372036854775807");
    });

    //Nat8
    test("produce equal results for CandyShared.Nat8", async () => {
        const rustJson = await rustCanister.candy_nat8_to_json();
        const motokoJson = await motokoCanister.candyNat8ToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    //Nat16
    test("produce equal results for CandyShared.Nat16", async () => {
        const rustJson = await rustCanister.candy_nat16_to_json();
        const motokoJson = await motokoCanister.candyNat16ToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    //Nat32
    test("produce equal results for CandyShared.Nat32", async () => {
        const rustJson = await rustCanister.candy_nat32_to_json();
        const motokoJson = await motokoCanister.candyNat32ToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    test("produce equal results for CandyShared.Nat64", async () => {
        const rustJson = await rustCanister.candy_nat64_to_json();
        const motokoJson = await motokoCanister.candyNat64ToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    //Int
    test("produce equal results for CandyShared.Int", async () => {
        const rustJson = await rustCanister.candy_int_to_json();
        const motokoJson = await motokoCanister.candyIntToJson();
        expect(rustJson).toEqual("-123_456_789_000");
        expect(motokoJson).toEqual("-123456789000");
    });

    //Int8
    test("produce equal results for CandyShared.Int8", async () => {
        const rustJson = await rustCanister.candy_int8_to_json();
        const motokoJson = await motokoCanister.candyInt8ToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    // Int16
    test("produce equal results for CandyShared.Int16", async () => {
        const rustJson = await rustCanister.candy_int16_to_json();
        const motokoJson = await motokoCanister.candyInt16ToJson();
        expect(rustJson).toEqual(motokoJson);
    });
    //Int32
    test("produce equal results for CandyShared.Int32", async () => {
        const rustJson = await rustCanister.candy_int32_to_json();
        const motokoJson = await motokoCanister.candyInt32ToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    //Int64
    test("produce equal results for CandyShared.Int64", async () => {
        const rustJson = await rustCanister.candy_int64_to_json();
        const motokoJson = await motokoCanister.candyInt64ToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    //Text
    test("produce equal results for CandyShared.Text", async () => {
        const rustJson = await rustCanister.candy_text_to_json();
        const motokoJson = await motokoCanister.candyTextToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    //Class
    test("produce equal results for CandyShared.Class", async () => {
        const rustJson = await rustCanister.candy_class_to_json();
        const motokoJson = await motokoCanister.candyClassToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    //Array
    test("produce equal results for CandyShared.Array", async () => {
        const rustJson = await rustCanister.candy_array_to_json();
        const motokoJson = await motokoCanister.candyArrayToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    //Option
    test("produce equal results for CandyShared.Option", async () => {
        const rustJson = await rustCanister.candy_option_to_json();
        const motokoJson = await motokoCanister.candyOptionToJson();
        expect(rustJson).toEqual(motokoJson);

        const rustJson2 = await rustCanister.candy_option_some_to_json();
        const motokoJson2 = await motokoCanister.candySomeToJson();
        expect(rustJson2).toEqual(motokoJson2);
    });

    //Nats
    test("produce equal results for CandyShared.Nats", async () => {
        const rustJson = await rustCanister.candy_nats_to_json();
        const motokoJson = await motokoCanister.candyNatsToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    //Ints
    test("produce equal results for CandyShared.Ints", async () => {
        const rustJson = await rustCanister.candy_ints_to_json();
        const motokoJson = await motokoCanister.candyIntsToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    //Floats
    test("produce equal results for CandyShared.Floats", async () => {
        const rustJson = await rustCanister.candy_floats_to_json();
        const motokoJson = await motokoCanister.candyFloatsToJson();
        expect(rustJson).toEqual("[1,2,3.5,4.123]");
        expect(motokoJson).toEqual("[1,2,3.5,4.1230000000000002]");
    });

    //Float
    test("produce equal results for CandyShared.Float", async () => {
        const rustJson = await rustCanister.candy_float_to_json();
        const motokoJson = await motokoCanister.candyFloatToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    //Bytes
    test("produce equal results for CandyShared.Bytes", async () => {
        const rustJson = await rustCanister.candy_bytes_to_json();
        const motokoJson = await motokoCanister.candyBytesToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    //Blob
    test("produce equal results for CandyShared.Blob", async () => {
        const rustJson = await rustCanister.candy_blob_to_json();
        const motokoJson = await motokoCanister.candyBlobToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    //Principal
    test("produce equal results for CandyShared.Principal", async () => {
        const rustJson = await rustCanister.candy_principal_to_json();
        const motokoJson = await motokoCanister.candyPrincipalToJson();
        expect(rustJson).toEqual(motokoJson);
    });

    //Bool
    test("produce equal results for CandyShared.Bool", async () => {
        const rustJson = await rustCanister.candy_bool_to_json();
        const motokoJson = await motokoCanister.candyBoolToJson();
        expect(rustJson).toEqual(motokoJson);
    });
});
