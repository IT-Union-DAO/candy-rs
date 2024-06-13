dfx canister stop --all
dfx canister delete --all
rm -rf ./src/declarations
cargo test
dfx canister create --all --no-wallet
dfx generate
dfx deploy --no-wallet