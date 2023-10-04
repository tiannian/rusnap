# RuSnap

Build Metamask Snap use Rust.

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
cargo rusnap new rusnap-pkg
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

Then you can load snap in metamask.

Execute these js statement in broswer devtool:

```js
window.ethereum.request({
  method: "wallet_requestSnaps",
  params: { "local:http://localhost:8080": {} },
});
```

Or use Metamask Snap Debug Tool.
