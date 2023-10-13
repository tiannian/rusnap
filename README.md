# RuSnap

Build Metamask Snap use Rust.

## Features

- Snap basic function.
  - Manage keys
  - Dialog and Notification
  - Random
  - Store Data
  - Network Access (WIP)
  - Ethereum Provider (WIP)
- RPC Handle
- Keyring API (TODO)

## Example

https://github.com/tiannian/rusnap-example

## Install and Usage

Install `cargo-rusnap`.

```bash
cargo install rusnap
```

### Dependencies

Please install these dependencies.

- wasm-pack
- npm (yarn or pnpm)

### Create Snap

```bash
cargo rusnap new <pkg-name>
```

### Build Snap

```bash
# Build dev mode in default
cargo rusnap build

# Build release mode
cargo rusnap build --release
```

### Start Snap

```bash
cargo rusnap start
```

## Snap

### Install Snap in Metamask

Then you can load snap in metamask.

Execute these js statement in broswer devtool:

```js
window.ethereum.request({
  method: "wallet_requestSnaps",
  params: { "local:http://localhost:8080": {} },
});
```

Or use Metamask Snap Debug Tool.

### Call Snap

Execute these js statement in broswer devtool:

```
window.ethereum.request({
  method: "wallet_invokeSnap",
  params: {
    snapId: "local:http://localhost:8080",
    request: {
      method: "hello",
    },
  },
});
```

## Publish Snap

> TODO

## Reference

### Basic Function

Basic function of Snap: ![docs.rs](https://img.shields.io/docsrs/rusnap)

### Random

Use `OsRng` in `rand_core` or other random crate based on `getrandom`.

Please add `getrandom` with `js` feature.

```toml
getrandom = { version = "0.2.10", features = ["js"] }
```

### Web3 SDK

Use `rusnap-ethers` and `ethers`.

> Note: `rusnap-ethers` don't re-export into `rusnap` crate, please add this crate independently.
