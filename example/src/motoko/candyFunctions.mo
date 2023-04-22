import Blob "mo:base/Blob";
import Nat8 "mo:base/Nat8";
import Prelude "mo:base/Prelude";
import Principal "mo:base/Principal";
import Conversion "mo:candy/conversion";
import Json "mo:candy/json";
import CandyTypes "mo:candy/types";
import Workspace "mo:candy/workspace";
import Map "mo:map/Map";
import Set "mo:map/Set";

actor MotokoCanisterCandyTypes {

    //Nat
    public query func getNat() : async CandyTypes.CandyShared {
        #Nat(15);
    };

    public query func getNat8() : async CandyTypes.CandyShared {
        #Nat8(15);
    };

    public query func getNat16() : async CandyTypes.CandyShared {
        #Nat16(15);
    };

    public query func getNat32() : async CandyTypes.CandyShared {
        #Nat32(15);
    };

    public query func getNat64() : async CandyTypes.CandyShared {
        #Nat64(15);
    };

    //Int
    public query func getInt() : async CandyTypes.CandyShared {
        #Int(15);
    };

    public query func getInt8() : async CandyTypes.CandyShared {
        #Int8(15);
    };

    public query func getInt16() : async CandyTypes.CandyShared {
        #Int16(15);
    };

    public query func getInt32() : async CandyTypes.CandyShared {
        #Int32(15);
    };

    public query func getInt64() : async CandyTypes.CandyShared {
        #Int64(15);
    };

    //Float
    public query func getFloat() : async CandyTypes.CandyShared {
        #Float(15.0);
    };

    //Principal
    public query func getPrincipal() : async CandyTypes.CandyShared {
        #Principal(Principal.fromText("aaaaa-aa"));
    };

    //Text
    public query func getText() : async CandyTypes.CandyShared {
        #Text("Hello, world!");
    };

    //Blob
    public query func getBlob() : async CandyTypes.CandyShared {
        let principal = Principal.fromText("un4fu-tqaaa-aaaab-qadjq-cai");
        let blob : Blob = Principal.toBlob(principal);
        #Blob(blob);
    };

    //Bool
    public query func getBool() : async CandyTypes.CandyShared {
        #Bool(true);
    };

    //Bytes
    public query func getBytes() : async CandyTypes.CandyShared {
        let principal = Principal.fromText("aaaaa-aa");
        let blob = Principal.toBlob(principal);
        #Bytes(Blob.toArray(blob));
    };

    //Array
    public query func getArray() : async CandyTypes.CandyShared {
        #Array([#Nat(15), #Int(-15)]);
    };

    public query func getClass() : async CandyTypes.CandyShared {
        #Class([{ immutable = true; name = "test"; value = #Nat(15) }, { immutable = true; name = "test2"; value = #Int(-15) }]);
    };

    public query func getFloats() : async CandyTypes.CandyShared {
        #Floats(([1.10, 19.49]));
    };

    //Nats
    public query func getNats() : async CandyTypes.CandyShared {
        #Nats(([1, 2, 3, 4]));
    };

    //Ints
    public query func getInts() : async CandyTypes.CandyShared {
        #Ints(([1, 2, 3, 4]));
    };

    //Option
    public query func getNullOption() : async CandyTypes.CandyShared {
        #Option(null);
    };

    public query func getOptSome() : async CandyTypes.CandyShared {
        #Option(?#Nat(15));
    };

    public query func getCandyMap() : async CandyTypes.CandyShared {
        let map = Map.new<CandyTypes.Candy, CandyTypes.Candy>();
        let map2 = Map.put(map, CandyTypes.candyMapHashTool, #Nat(15), #Nat(15));
        return CandyTypes.shareCandy(#Map(map));
    };

    public query func getCandySet() : async CandyTypes.CandyShared {
        let set = Set.new<CandyTypes.Candy>();
        let set2 = Set.put(set, CandyTypes.candyMapHashTool, #Nat(15));
        return CandyTypes.shareCandy(#Set(set));
    };

    /*
       Conversion to Blob
    */
    public query func candyBlobToBlob() : async Blob {
        Conversion.candySharedToBlob(#Blob(Blob.fromArray([0, 1, 2, 3, 4, 5, 6, 7])));
    };

    public query func candyBytesToBlob() : async Blob {
        Conversion.candySharedToBlob(#Bytes([0, 1, 2, 3, 4, 5, 6, 7]));
    };

    public query func candyTextToBlob() : async Blob {
        Conversion.candySharedToBlob(#Text("Hello, world!"));
    };

    public query func candyNatToBlob() : async Blob {
        Conversion.candySharedToBlob(#Nat(1_234_567_890));
    };

    public query func candyNat8ToBlob() : async Blob {
        Conversion.candySharedToBlob(#Nat8(255));
    };

    public query func candyNat16ToBlob() : async Blob {
        Conversion.candySharedToBlob(#Nat16(65535));
    };

    public query func candyNat32ToBlob() : async Blob {
        Conversion.candySharedToBlob(#Nat32(123_456_789));
    };

    public query func candyNat64ToBlob() : async Blob {
        Conversion.candySharedToBlob(#Nat64(9_223_372_036_854_775_807));
    };

    //Int
    public query func candyIntToBlob() : async Blob {
        Conversion.candySharedToBlob(#Int(-123_456_789_000));
    };

    //Principal
    public query func candyPrincipalToBlob() : async Blob {
        Conversion.candySharedToBlob(#Principal(Principal.fromText("aaaaa-aa")));
    };

    //Float
    public query func candyFloatToBlob() : async Blob {
        Conversion.candySharedToBlob(#Float(1.234));
    };

    /*
        Conversions to JSON
    */
    public query func candyNatToJson() : async Text {
        Json.value_to_json(#Nat(9_223_372_036_854_775_807));
    };
    //Nat8
    public query func candyNat8ToJson() : async Text {
        Json.value_to_json(#Nat8(255));
    };
    //Nat16
    public query func candyNat16ToJson() : async Text {
        Json.value_to_json(#Nat16(65535));
    };
    //Nat32
    public query func candyNat32ToJson() : async Text {
        Json.value_to_json(#Nat32(123_456_789));
    };
    //Nat64
    public query func candyNat64ToJson() : async Text {
        Json.value_to_json(#Nat64(9_223_372_036_854_775_807));
    };

    //Int
    public query func candyIntToJson() : async Text {
        Json.value_to_json(#Int(-123_456_789_000));
    };

    //Int8
    public query func candyInt8ToJson() : async Text {
        Json.value_to_json(#Int8(-128));
    };

    //Int16
    public query func candyInt16ToJson() : async Text {
        Json.value_to_json(#Int16(-32768));
    };

    //Int32
    public query func candyInt32ToJson() : async Text {
        Json.value_to_json(#Int32(-2147483648));
    };

    //Int64
    public query func candyInt64ToJson() : async Text {
        Json.value_to_json(#Int64(-9223372036854775808));
    };

    //Text
    public query func candyTextToJson() : async Text {
        Json.value_to_json(#Text("Hello, world!"));
    };

    //Class
    public query func candyClassToJson() : async Text {
        Json.value_to_json(#Class([{ immutable = true; name = "test"; value = #Nat(15) }, { immutable = true; name = "test2"; value = #Int(-15) }]));
    };

    //Array
    public query func candyArrayToJson() : async Text {
        Json.value_to_json(#Array([#Nat(15), #Int(-15)]));
    };

    //Option
    public query func candyOptionToJson() : async Text {
        Json.value_to_json(#Option(null));
    };

    public query func candySomeToJson() : async Text {
        Json.value_to_json(#Option(?#Nat(15)));
    };

    //Nats
    public query func candyNatsToJson() : async Text {
        Json.value_to_json(#Nats(([1, 2, 3, 4])));
    };

    //Ints
    public query func candyIntsToJson() : async Text {
        Json.value_to_json(#Ints(([1, 2, 3, 4])));
    };

    //Floats
    public query func candyFloatsToJson() : async Text {
        Json.value_to_json(#Floats(([1.0, 2.0, 3.5, 4.123])));
    };

    //Bytes
    public query func candyBytesToJson() : async Text {
        Json.value_to_json(#Bytes([0, 1, 2, 3, 4, 5, 6, 7]));
    };

    //Blob
    public query func candyBlobToJson() : async Text {
        Json.value_to_json(#Blob(Blob.fromArray([0, 1, 2, 3, 4, 5, 6, 7])));
    };

    //Principal
    public query func candyPrincipalToJson() : async Text {
        Json.value_to_json(#Principal(Principal.fromText("qjdve-lqaaa-aaaaa-aaaeq-cai")));
    };

    //Float
    public query func candyFloatToJson() : async Text {
        Json.value_to_json(#Float(1.234));
    };

    //Bool
    public query func candyBoolToJson() : async Text {
        Json.value_to_json(#Bool(true));
    };

    /**
        Get Candy Shared Size
    **/

    //Int
    public query func sizeOfCandyInt() : async Nat {
        Workspace.getCandySharedSize(#Int(-123_456_789_000));
    };

    //Int8
    public query func sizeOfCandyInt8() : async Nat {
        Workspace.getCandySharedSize(#Int8(-128));
    };

    //Int16
    public query func sizeOfCandyInt16() : async Nat {
        Workspace.getCandySharedSize(#Int16(-32768));
    };

    //Int32
    public query func sizeOfCandyInt32() : async Nat {
        Workspace.getCandySharedSize(#Int32(-2147483648));
    };

    //Int64
    public query func sizeOfCandyInt64() : async Nat {
        Workspace.getCandySharedSize(#Int64(-9223372036854775808));
    };

    //Nat
    public query func sizeOfCandyNat() : async Nat {
        Workspace.getCandySharedSize(#Nat(1_234_567_890));
    };

    //Nat8
    public query func sizeOfCandyNat8() : async Nat {
        Workspace.getCandySharedSize(#Nat8(255));
    };

    //Nat16
    public query func sizeOfCandyNat16() : async Nat {
        Workspace.getCandySharedSize(#Nat16(65535));
    };

    //Nat32
    public query func sizeOfCandyNat32() : async Nat {
        Workspace.getCandySharedSize(#Nat32(123_456_789));
    };

    //Nat64
    public query func sizeOfCandyNat64() : async Nat {
        Workspace.getCandySharedSize(#Nat64(9_223_372_036_854_775_807));
    };

    //Float
    public query func sizeOfCandyFloat() : async Nat {
        Workspace.getCandySharedSize(#Float(1.234));
    };

    //Text
    public query func sizeOfCandyText() : async Nat {
        Workspace.getCandySharedSize(#Text("Hello, world!"));
    };

    //Bool
    public query func sizeOfCandyBool() : async Nat {
        Workspace.getCandySharedSize(#Bool(true));
    };

    //Blob
    public query func sizeOfCandyBlob() : async Nat {
        Workspace.getCandySharedSize(#Blob(Blob.fromArray([0, 1, 2, 3, 4, 5, 6, 7])));
    };

    //Class
    public query func sizeOfCandyClass() : async Nat {
        Workspace.getCandySharedSize(#Class([{ immutable = true; name = "test"; value = #Nat(15) }, { immutable = true; name = "test2"; value = #Int(-15) }]));
    };

    //Principal
    public query func sizeOfCandyPrincipal() : async Nat {
        Workspace.getCandySharedSize(#Principal(Principal.fromText("qjdve-lqaaa-aaaaa-aaaeq-cai")));
    };

    //Option
    public query func sizeOfCandyOption() : async Nat {
        Workspace.getCandySharedSize(#Option(?#Nat(15)));
    };

    //Option None
    public query func sizeOfCandyOptionNone() : async Nat {
        Workspace.getCandySharedSize(#Option(null));
    };

    //Array
    public query func sizeOfCandyArray() : async Nat {
        Workspace.getCandySharedSize(#Array([#Nat(15), #Int(-15)]));
    };

    //Bytes
    public query func sizeOfCandyBytes() : async Nat {
        Workspace.getCandySharedSize(#Bytes([0, 1, 2, 3, 4, 5, 6, 7]));
    };

    //Floats
    public query func sizeOfCandyFloats() : async Nat {
        Workspace.getCandySharedSize(#Floats(([1.234, 2.345, 3.456, 4.567, 5.678, 6.789, 7.890])));
    };

    //Nats
    public query func sizeOfCandyNats() : async Nat {
        Workspace.getCandySharedSize(#Nats(([1, 2, 3, 4])));
    };

    //Ints
    public query func sizeOfCandyInts() : async Nat {
        Workspace.getCandySharedSize(#Ints(([-1, -2, -3, -4])));
    };

    //Map
    public query func sizeOfCandyMap() : async Nat {
        let map = Map.new<CandyTypes.Candy, CandyTypes.Candy>();
        let map2 = Map.put(map, CandyTypes.candyMapHashTool, #Nat(15), #Nat(15));
        return Workspace.getCandySharedSize(CandyTypes.shareCandy(#Map(map)));
    };

    //Set
    public query func sizeOfCandySet() : async Nat {
        let set = Set.new<CandyTypes.Candy>();
        let _ = Set.put(set, CandyTypes.candyMapHashTool, #Nat(15));
        let _ = Set.put(set, CandyTypes.candyMapHashTool, #Nat(15));
        return Workspace.getCandySharedSize(CandyTypes.shareCandy(#Set(set)));
    };

};
