import { _SERVICE as WorkspaceCanister } from "dfx-gen/workspace/workspace.did";
import { _SERVICE as RustCanister } from "dfx-gen/rust/rust.did";
import { createRustActor, workspaceActor } from "./utils";
import { AddressedChunkArray } from "dfx-type/workspace/workspace";
import { beforeAll } from "@jest/globals";

describe("Workspace functionality should be equal for motoko and rust libraries", () => {
  let rustCanister: RustCanister;
  let motokoCanister: WorkspaceCanister;

  beforeAll(async () => {
    rustCanister = createRustActor();
    motokoCanister = workspaceActor();
  });

  describe("AddressedChunksArrays", () => {
    test("are equal", async () => {
      const rustArray = await rustCanister.test_chunk_array();
      const motokoArray = await motokoCanister.testChunkArray();
      expect(rustArray).toEqual(motokoArray);
    });

    test("sizes are not equal", async () => {
      const rustSize = await rustCanister.get_chunk_array_size();
      const motokoSize = await motokoCanister.getChunkArraySize();
      expect(rustSize).toEqual(BigInt(246));
      expect(motokoSize).toEqual(BigInt(120));
    });

    test("flattened bytes are equal", async () => {
      const rustBytes = await rustCanister.get_flatten_chunk_array();
      const motokoBytes = await motokoCanister.getFlattenChunkedArray();

      expect(rustBytes).toEqual(motokoBytes);
    });

    test.each([
      [0, 0],
      [0, 1],
      [1, 0],
      [1, 1],
      [2, 0],
      [2, 1],
    ])(
      "values are equal for zone '%s' : chunk '%s' ",
      async (dataZoneId, chunkId) => {
        let rustValue;
        let motokoValue;
        rustValue =
          await rustCanister.get_data_chunk_from_addressed_chunk_array(
            BigInt(dataZoneId),
            BigInt(chunkId)
          );
        motokoValue = await motokoCanister.getDataChunkFromAddressedChunkArray(
          BigInt(dataZoneId),
          BigInt(chunkId)
        );
        console.log(
          "Test values: rust ['%s'] : motoko ['%s']",
          rustValue,
          motokoValue
        );
        expect(rustValue).toEqual(motokoValue);
      }
    );
  });

  describe("DataZone", () => {
    test("sizes are not equal", async () => {
      const rustSize = await rustCanister.get_data_zone_size();
      const motokoSize = await motokoCanister.getDataZoneSize();
      expect(rustSize).toEqual(BigInt(387));
      expect(motokoSize).toEqual(BigInt(1));
    });

    test("byte conversions are equal", async () => {
      const rustBytes = await rustCanister.to_bytes_buffer();
      const motokoBytes = await motokoCanister.toBytesBuffer();
      expect(rustBytes).toEqual(motokoBytes);
    });

    test("conversion from buffer to array of Candy bytes is equal", async () => {
      const rustBytes = await rustCanister.from_buffer();
      const motokoBytes = await motokoCanister.fromBuffer();

      expect(rustBytes).toEqual(motokoBytes);
    });
  });

  describe("Workspace", () => {
    let array: AddressedChunkArray;

    beforeAll(async () => {
      array = await rustCanister.test_chunk_array();
    });

    test("chunk counts are equal", async () => {
      const rustCount = await rustCanister.count_chunks(array);
      const motokoCount = await motokoCanister.countChunks(array);

      expect(rustCount).toEqual(motokoCount);
    });

    test("conversion to workspace and back to addressed chunk array is equal", async () => {
      const array: AddressedChunkArray = await rustCanister.test_chunk_array();
      const rustArray = await rustCanister.to_workspace_and_back(array);
      const motokoArray = await motokoCanister.toWorkspaceAndBack(array);

      expect(rustArray).toEqual(motokoArray);
    });

    test.each([
      [10, 10001, 7],
      [100, 1, 7],
      [200, 1, 4],
      [300, 1, 4],
      [400, 1, 3],
    ])(
      "chunk size are equal for max size (%s)",
      async (maxChunkSize, motokoResult, rustResult) => {
        const rustSize = await rustCanister.get_ws_chunk_size(
          array,
          BigInt(maxChunkSize)
        );
        const motokoSize = await motokoCanister.getWorkspaceChunkSize(
          array,
          BigInt(maxChunkSize)
        );
        expect(rustSize).toEqual(BigInt(rustResult));
        expect(motokoSize).toEqual(BigInt(motokoResult));
      }
    );

    //Disable for now
    test.skip.each([
      [0, 500],
      [0, 600],
      [0, 800],
      [0, 900],
      [0, 200],
    ])(
      "getting NOT equal values from workspace chunk %id and max size [%s] for rust and motoko",
      async (id, maxSize) => {
        const rustValue = await rustCanister.get_workspace_chunk(
          array,
          BigInt(id),
          BigInt(maxSize)
        );
        const motokoValue = await motokoCanister.getWorkspaceChunk(
          array,
          BigInt(id),
          BigInt(maxSize)
        );

        console.log("Values: ", rustValue, motokoValue);
        expect(rustValue).toEqual(motokoValue);
      }
    );
  });
});
