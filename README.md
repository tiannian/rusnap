# RuSnap

Build Metamask Snap use Rust.

## Crates

| Name   | crate.io                                                                                | docs.rs |
| ------ | --------------------------------------------------------------------------------------- | ------- |
| rusnap | [![Crates.io](https://img.shields.io/crates/v/rusnap)](https://crates.io/crates/rusnap) |         |

## Documents

Visit API reference on [docs.rs](https://docs.rs/rusnap/latest/rusnap/).

And the documents for detail usage in [book](book)

### Quickstart

#### Install and Usage

Install `cargo-rusnap`.

```bash
cargo install rusnap
```

#### Create Snap

Creating a Snap is very easy, only need run:

```bash
cargo rusnap new <pkg-name>
```

#### Start Snap

Enter you Snap folder:

```bash
cd <pkg-name>
```

Then start a dev server.

```bash
cargo rusnap start
```

This command will recompile your snap when you change file.

### Example

https://github.com/tiannian/rusnap-example

## Features

- Manage keys (bip32, bip44, entropy)
- Dialog and Notification
- [Random](#random)
- Store Data
- Network access (See issue)
- Ethereum provider (work with ethers)
- Handle RPC call based on method
- Cronjon support
- Transaction insight support
- Without npm or node requirement
- Using Rust project structure

## Useful crates

### Random

Use `OsRng` in `rand_core` or other random crate based on `getrandom`.

Please add `getrandom` with `js` feature.

```toml
getrandom = { version = "0.2.10", features = ["js"] }
```

### Web3 SDK

Use `rusnap-ethers` and `ethers`.

> Note: `rusnap-ethers` don't re-export in `rusnap` crate, please add this crate independently.
