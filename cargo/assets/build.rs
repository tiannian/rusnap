fn main() {
    rusnap_build::build();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Snap.toml");
}
