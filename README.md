# Ethgen

<br>

A toolkit for ethereum compatible development.

<br>

Contains base primitives, utilities and helper macroses to construct RPC calls,
to deserialize RPC responses and extract the data and to sign transactions with
a private key, built with full `no_std` support.

> Currently uses `forbid(unsafe_code)` directive, however this may change as
> soon benchmarking is done and use of unsafe code would show significant gain
> of performance.

<br>
<br>

## Why not just use **ethers-rs** or **anvil**?

The idea behind this project is to pack the minimum set of things required for
an app to interact with ethereum compatible network with a completelly `no_std`
library.

Imagine all the cryptoghraphy happening in a wallet in a isolated wasm module,
or a chain event listener running on a smart watch. This and many more are the
opportunities this create is designed to open for the blockchain dev community.

<br>
<br>

## Development state

> This project is not yet released, if you are looking for a create to use,
> consider checking the [ethers-rs](https://docs.rs/ethers/latest/ethers/).

Currently this project is at the embryo state and is under heavy development,
the following features are planned to be included in the first stable release:

- [] Blockchain primitives;
	- [] Address;
	- [] Block;
	- [] U256;
- [x] JSON serializable / deserializable RPC calls;
- [] Eip1559 Transaction (with RLP serialization);
- [] Secp256k1 signature;

<br>
<br>

## Sources:

- [bytes](https://docs.rs/bytes/latest/bytes/);
- [generic-array](https://docs.rs/generic-array/latest/generic_array/);
- [crypto-bigint](https://docs.rs/crypto-bigint/latest/crypto_bigint/);
- [faster-hex](https://docs.rs/faster-hex/0.8.1/faster_hex/);

### JSON

- [serde_json](https://docs.rs/serde_json/latest/serde_json/);
- [serde_json_core](https://docs.rs/serde-json-core/latest/serde_json_core/);

### RPC

- [ethereum JSON RPC](https://ethereum.org/en/developers/docs/apis/json-rpc/);
- [anvil](https://github.com/foundry-rs/foundry/blob/master/crates/anvil);

### RLP

- [ethereum RLP](https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/);
- [parity-common/rlp](https://docs.rs/rlp/latest/rlp/);

### Crypto

- [sha3](https://docs.rs/sha3/latest/sha3/);
- [k256](https://docs.rs/k256/latest/k256/);

<br>
<br>

## Build documentation

```
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc
```

<br>
<br>
