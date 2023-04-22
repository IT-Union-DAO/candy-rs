# Example Project

This is an example project that demonstrates the usage of the Candy library with Motoko and Rust
canisters. The project consists of several canisters - `candyFunctions-motoko`, `worskpace-motoko`
and `rust` -
that use the Candy library to perform various operations. Basically they contain the same set of
update and query endpoints. jest tests are set up to check inter canister calls, type compatability
and candy library functions

## Installation

To install the project, run the `reinstall.sh` script in the root directory:

```
dfx start --background
./reinstall.sh
```

The script will install the necessary dependencies for the Motoko and Rust canisters.

## Usage

To run the Jest tests for the Motoko canister, navigate to the `candy-example-motoko` directory and
run the following commands:

```
npm install
npm test
```

To run the Rust tests, navigate to the `candy-example-rust` directory and run the following command:

```
cargo test
```

Rust tests produce `.did` for rust canister
