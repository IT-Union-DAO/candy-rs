[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![crates.io](https://img.shields.io/crates/v/ic_candy)

## [candy-rs](https://crates.io/crates/ic_candy)

`candy-rs` is a Rust library for working with ic-cdk data types. It also serves as a tool for
communicating with motoko canisters
which use [motoko candy library](https://github.com/icdevs/candy_library/tree/0.2.0)

## Example

The `example` directory contains a DFX set up project that includes two canisters, one written in
Motoko and another in Rust, both of which use the Candy libraries. These canisters showcase the type
compatibility and functionality of the Candy libraries. The tests in the "jest" directory
demonstrate typings and the differences in functionality due to
the diffetent byte size of values in Motoko and Rust.

## Testing

From the root directory of the project, execute the following command:

```bash
 cargo test
```

## Contributing

Contributions to Candy are welcome! If you find a bug or have a feature request, please file an
issue on the GitHub repository. If you would like to contribute code, please fork the repository and
submit a pull request. All contributions must be licensed under the MIT License.

## License

Candy is licensed under the MIT License.

## Donations

To toss a coin for a coffee (thank you so much), please, use the following requisites:

* Bitcoin - bc1qxjphkd5g57vqul5d4y8lwt8z6qd84wlhmumug4
* ICP - 50ad6ca3ac2249d8b7f711252e1ac7ecccdd01883ecc9e5bd1e50336248d0f34
* TON - UQAXO2hMFrN8kJwf-nN04jexs5BkPsrrlJyzT3RpXYUSvuyt