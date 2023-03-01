import CandyTypes "mo:candy/types"

actor MotokoCanister {
    public query func getNat() : async CandyTypes.CandyValue {
        #Nat(15);
    };

    // public shared query func getNatUnstable() : async CandyTypes.CandyValueUnstable {
    //     #Nat(15);
    // };
};
