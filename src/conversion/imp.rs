use candid::Nat;

trait UnboxCandyValue {
    fn to_nat(self) -> Nat;
    fn to_nat8(self) -> u8;

}