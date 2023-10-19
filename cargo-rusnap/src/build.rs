use std::{
    fs::{self, File},
    io::Write,
};

use anyhow::{anyhow, Result};
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

        let manifest_path = base.join("snap.manifest.json");

        let minifest = fs::read_to_string(&manifest_path)?;
        let mut jsonv: serde_json::Value = serde_json::from_str(&minifest)?;
        let jv = &mut jsonv["source"]
            .as_object_mut()
            .ok_or(anyhow!("No source"))?;
        jv.remove("shasum");
        let manifest = serde_json::to_string(&jsonv)?;
        let hm = Sha256::digest(manifest);

        let code = fs::read(base.join("dist/bundle.js")).unwrap();
        let hc = Sha256::digest(code);

        let icon = fs::read(base.join("icon")).unwrap();
        let hi = Sha256::digest(icon);

        let mut hasher = Sha256::new();
        hasher.update(hc);
        hasher.update(hi);
        hasher.update(hm);
        let result = hasher.finalize();
        let r = general_purpose::STANDARD.encode(result);

        jsonv["source"]["shasum"] = r.into();
        let manifest = serde_json::to_string_pretty(&jsonv)?;
        fs::write(manifest_path, manifest)?;

        Ok(())
    }

    pub fn execute(self) -> Result<()> {
        self.build_wasm()?;

        log::info!("Generate js code");
        self.build_js()?;

        log::info!("Re-build manifest file");
        self.build_manifest()?;

        Ok(())
    }
}
