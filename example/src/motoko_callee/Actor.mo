import Blob "mo:base/Blob";
import Prelude "mo:base/Prelude";
import Principal "mo:base/Principal";
import CandyTypes "mo:candy/types";
actor MotokoCanister {

    //Nat
    public query func getNat() : async CandyTypes.CandyValue {
        #Nat(15);
    };

    public query func getNat8() : async CandyTypes.CandyValue {
        #Nat8(15);
    };

    public query func getNat16() : async CandyTypes.CandyValue {
        #Nat16(15);
    };

    public query func getNat32() : async CandyTypes.CandyValue {
        #Nat32(15);
    };

    public query func getNat64() : async CandyTypes.CandyValue {
        #Nat64(15);
    };

    //Int
    public query func getInt() : async CandyTypes.CandyValue {
        #Int(15);
    };

    public query func getInt8() : async CandyTypes.CandyValue {
        #Int8(15);
    };

    public query func getInt16() : async CandyTypes.CandyValue {
        #Int16(15);
    };

    public query func getInt32() : async CandyTypes.CandyValue {
        #Int32(15);
    };

    public query func getInt64() : async CandyTypes.CandyValue {
        #Int64(15);
    };

    //Float
    public query func getFloat() : async CandyTypes.CandyValue {
        #Float(15.0);
    };

    //Principal
    public query func getPrincipal() : async CandyTypes.CandyValue {
        #Principal(Principal.fromText("aaaa-aa"));
    };

    //Text
    public query func getText() : async CandyTypes.CandyValue {
        #Text("Hello, world!");
    };

    //Blob
    public query func getBlob() : async CandyTypes.CandyValue {
        let principal = Principal.fromText("aaaa-aa");
        let blob = Principal.toBlob(principal);
        #Blob(blob);
    };

    //Bool
    public query func getBool() : async CandyTypes.CandyValue {
        #Bool(true);
    };

    //Bytes
    public query func getFrozenBytes() : async CandyTypes.CandyValue {
        let principal = Principal.fromText("aaaa-aa");
        let blob = Principal.toBlob(principal);
        #Bytes(#frozen(Blob.toArray(blob)));
    };

    public query func getThawedBytes() : async CandyTypes.CandyValue {
        let principal = Principal.fromText("aaaa-aa");
        let blob = Principal.toBlob(principal);
        #Bytes(#thawed(Blob.toArray(blob)));
    };

    //Array
    public query func getFrozenArray() : async CandyTypes.CandyValue {
        #Array(#frozen([#Nat(15), #Int(-15), #Empty]));
    };

    public query func getThawedArray() : async CandyTypes.CandyValue {
        #Array(#thawed([#Nat(15), #Int(-15), #Empty]));
    };

    public query func getClass() : async CandyTypes.CandyValue {
        #Class([{ immutable = true; name = "test"; value = #Nat(15) }, { immutable = true; name = "test2"; value = #Int(-15) }]);
    };

    //Empty
    public query func getEmpty() : async CandyTypes.CandyValue {
        #Empty;
    };

    //Floats
    public query func getFrozenFloats() : async CandyTypes.CandyValue {
        #Floats(#frozen([26.07, 19.53]));
    };

    public query func getThawedFloats() : async CandyTypes.CandyValue {
        #Floats(#thawed([1.10, 19.49]));
    };

    //Nats
    public query func getFrozenNats() : async CandyTypes.CandyValue {
        #Nats(#frozen([1, 2, 3, 4]));
    };

    public query func getThawedNats() : async CandyTypes.CandyValue {
        #Nats(#frozen([5, 6, 7, 8]));
    };

    //Option
    public query func getNullOption() : async CandyTypes.CandyValue {
        #Option(null);
    };

    public query func getOptSome() : async CandyTypes.CandyValue {
        #Option(?#Nat(15));
    };

    //Text
    public query func getFrozenText() : async CandyTypes.CandyValue {
        #Text("Hello, world!");
    };

};
