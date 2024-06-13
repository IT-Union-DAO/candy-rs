import Array "mo:base/Array";
import Buffer "mo:base/Buffer";
import Debug "mo:base/Debug";
import Nat8 "mo:base/Nat8";
import Principal "mo:base/Principal";
import Conversion "mo:candy/conversion";
import Types "mo:candy/types";
import Workspace "mo:candy/workspace";
import StableBuffer "mo:stablebuffer/StableBuffer";

actor WorkspacesFunctions {

    public func testChunkArray() : async Types.AddressedChunkArray {
        let ws = Workspace.emptyWorkspace();

        let dz_1 = StableBuffer.init<Types.DataChunk>();

        StableBuffer.add(dz_1, #Nat(16));
        StableBuffer.add(dz_1, #Principal(Principal.fromText("aaaaa-aa")));

        let dz_2 = StableBuffer.init<Types.DataChunk>();
        StableBuffer.add<Types.Candy>(dz_2, #Nat(16));
        StableBuffer.add<Types.Candy>(dz_2, #Principal(Principal.fromText("aaaaa-aa")));

        let dz_3 = StableBuffer.initPresized<Types.DataChunk>(2);
        StableBuffer.add<Types.DataChunk>(dz_3, #Int(-123_456_789_000));
        StableBuffer.add<Types.DataChunk>(dz_3, #Text("Hello, world!"));

        StableBuffer.add<Types.DataZone>(ws, dz_1);
        StableBuffer.add<Types.DataZone>(ws, dz_2);
        StableBuffer.add<Types.DataZone>(ws, dz_3);

        return Workspace.workspaceToAddressedChunkArray(ws);
    };

    // Addressed chunk array
    public func getChunkArraySize() : async Nat {
        let arr = await testChunkArray();
        return Workspace.getAddressedChunkArraySize(arr);
    };

    public func getFlattenChunkedArray() : async [Nat8] {
        let arr = await testChunkArray();
        return Workspace.flattenAddressedChunkArray(arr);
    };

    public func getDataChunkFromAddressedChunkArray(dataZone : Nat, dataChunk : Nat) : async Types.CandyShared {
        let arr = await testChunkArray();
        return Workspace.getDataChunkFromAddressedChunkArray(arr, dataZone, dataChunk);
    };

    //DataZone
    public func getDataZoneSize() : async Nat {
        let dz_2 = StableBuffer.initPresized<Types.DataChunk>(2);
        StableBuffer.add<Types.DataChunk>(dz_2, #Nat(16));
        StableBuffer.add<Types.DataChunk>(dz_2, #Principal(Principal.fromText("aaaaa-aa")));
        let size = Workspace.getDataZoneSize(dz_2);
        Debug.print("motoko data zone size ");
        Debug.print(debug_show (Workspace.getCandySize(#Nat(16))));
        Debug.print(debug_show (Workspace.getCandySize(#Principal(Principal.fromText("aaaaa-aa")))));
        return size;
    };

    public func toBytesBuffer() : async [[Nat8]] {
        let dz = StableBuffer.initPresized<Types.DataChunk>(2);
        StableBuffer.add<Types.DataChunk>(dz, #Nat(16));
        StableBuffer.add<Types.DataChunk>(dz, #Principal(Principal.fromText("aaaaa-aa")));

        let buffer = Workspace.byteBufferDataZoneToBuffer(dz);
        let array_of_buf = Buffer.toArray(buffer);
        return Array.map<Buffer.Buffer<Nat8>, [Nat8]>(
            array_of_buf,
            func(x) {
                Buffer.toArray(x);
            },
        );
    };

    public func fromBuffer() : async [Types.CandyShared] {
        let dz = StableBuffer.initPresized<Types.DataChunk>(2);
        StableBuffer.add<Types.DataChunk>(dz, #Nat(16));
        StableBuffer.add<Types.DataChunk>(dz, #Principal(Principal.fromText("aaaaa-aa")));

        let buffer = Workspace.byteBufferDataZoneToBuffer(dz);

        let array_of_buf = Buffer.toArray(buffer);
        return Array.map<Buffer.Buffer<Nat8>, Types.CandyShared>(
            array_of_buf,
            func(x : Buffer.Buffer<Nat8>) : Types.CandyShared {
                let arr : [Nat8] = Buffer.toArray(x);
                return #Bytes(arr);
            },
        );

    };

    //Workspace

    public func countChunks(chunks : Types.AddressedChunkArray) : async Nat {
        let ws = Workspace.fromAddressedChunks(chunks);
        return Workspace.countAddressedChunksInWorkspace(ws);
    };

    public func toWorkspaceAndBack(chunks : Types.AddressedChunkArray) : async Types.AddressedChunkArray {
        let ws = Workspace.fromAddressedChunks(chunks);
        return Workspace.workspaceToAddressedChunkArray(ws);
    };

    public func getWorkspaceChunkSize(chunks : Types.AddressedChunkArray, maxChunkSize : Nat) : async Nat {
        let ws = Workspace.fromAddressedChunks(chunks);
        return Workspace.getWorkspaceChunkSize(ws, maxChunkSize);
    };

    public func getWorkspaceChunk(chunks : Types.AddressedChunkArray, chunkID : Nat, maxChunkSize : Nat) : async ({ #eof; #chunk }, Types.AddressedChunkArray) {
        let ws = Workspace.fromAddressedChunks(chunks);
        let unshared : ({ #eof; #chunk }, Types.AddressedChunkBuffer) = Workspace.getWorkspaceChunk(ws, chunkID, maxChunkSize);

        return (unshared.0, StableBuffer.toArray(unshared.1));
    };

};
