use std::{fs, path::Path};

use digest::Digest;
use serde_json::Value;
use sha2::Sha256;

fn main() {
    let base = Path::new("../rusnap-example/target/rusnap");

    let minifest = fs::read_to_string(base.join("snap.manifest.json")).unwrap();

    let mut jsonv: Value = serde_json::from_str(&minifest).unwrap();

    let jv = &mut jsonv["source"].as_object_mut().unwrap();
    jv.remove("shasum");

    let manifest = serde_json::to_string(&jsonv).unwrap();
    println!("{}", manifest);
    let hm = Sha256::digest(manifest);

    let code = fs::read(base.join("dist/bundle.js")).unwrap();
    let hc = Sha256::digest(code);

    let icon = fs::read(base.join("icon.svg")).unwrap();
    let hi = Sha256::digest(icon);

    let mut hasher = Sha256::new();
    hasher.update(hc);
    hasher.update(hi);
    hasher.update(hm);
    let result = hasher.finalize();
    let r = base64::encode(result);

    println!("{r}");
}
