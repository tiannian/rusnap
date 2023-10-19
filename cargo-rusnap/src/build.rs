use std::{
    fs::{self, File},
    io::Write,
};

use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use clap::Args;
use digest::Digest;
use sha2::Sha256;
use wasm_pack::{
    command::{
        build::{BuildOptions, Target},
        run_wasm_pack, Command,
    },
    install::InstallMode,
};

use crate::utils;

#[derive(Args, Debug)]
pub struct BuildArg {
    #[clap(short, long, group = "target")]
    pub dev: bool,

    #[clap(short, long, group = "target")]
    pub release: bool,

    #[clap(short, long, group = "target")]
    pub profiling: bool,
}

impl BuildArg {
    pub fn build_wasm(&self) -> Result<()> {
        let target_path = utils::get_rusnap_path()?;

        if !target_path.exists() {
            fs::create_dir_all(&target_path)?;
        }

        let build_command = BuildOptions {
            path: None,
            scope: None,
            mode: InstallMode::Normal,
            disable_dts: true,
            weak_refs: false,
            reference_types: false,
            target: Target::NoModules,
            debug: false,
            dev: self.dev,
            release: self.release,
            profiling: self.profiling,
            out_dir: target_path
                .join("pkg")
                .to_str()
                .expect("Failed into str")
                .into(),
            out_name: Some("__rusnap".into()),
            no_pack: true,
            extra_options: vec![],
        };

        run_wasm_pack(Command::Build(build_command))?;

        Ok(())
    }

    pub fn build_js(&self) -> Result<()> {
        let target_path = utils::get_rusnap_path()?;

        let pkg = target_path.join("pkg");
        let dist = target_path.join("dist");
        let bundle = dist.join("bundle.js");

        fs::create_dir_all(dist)?;

        fs::copy(pkg.join("__rusnap.js"), &bundle)?;

        let mut bf = File::options().append(true).open(&bundle)?;

        bf.write_all(b"\n")?;
        bf.write_all(b"const __wasm_module = \"")?;

        let s = std::fs::read(pkg.join("__rusnap_bg.wasm"))?;
        bf.write_all(general_purpose::STANDARD_NO_PAD.encode(s).as_bytes())?;
        bf.write_all(b"\";")?;
        bf.write_all(b"\n")?;
        bf.write_all(include_bytes!("../assets/entry.js"))?;

        Ok(())
    }

    pub fn build_manifest(&self) -> Result<()> {
        let base = utils::get_rusnap_path()?;

        let minifest = fs::read_to_string(base.join("snap.manifest.json")).unwrap();
        let mut jsonv: serde_json::Value = serde_json::from_str(&minifest).unwrap();
        let jv = &mut jsonv["source"].as_object_mut().unwrap();
        jv.remove("shasum");
        let manifest = serde_json::to_string(&jsonv).unwrap();
        let hm = Sha256::digest(manifest);

        let code = fs::read(base.join("dist/bundle.js")).unwrap();
        let hc = Sha256::digest(code);

        let icon = fs::read(base.join("icon.svg")).unwrap();

        Ok(())
    }

    pub fn execute(self) -> Result<()> {
        self.build_wasm()?;

        self.build_js()?;

        self.build_manifest()?;

        Ok(())
    }
}
