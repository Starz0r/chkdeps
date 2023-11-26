use std::{collections::HashMap, path::Path};

use {anyhow::Result, either::Either, serde::Deserialize, toml};

use crate::fs_utils;

pub fn crates_from_cargo_file<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<Cargo> {
    return Ok(toml::from_str::<Cargo>(&fs_utils::file_contents(path)?)?);
}

type CrateVersion = String;

#[derive(Deserialize)]
pub struct CrateInfo {
    pub version: CrateVersion,

    pub features: Vec<String>,
}

// NOTE: Unfortunate bludgeon to get Either deserializable
#[derive(Deserialize)]
#[serde(transparent)]
pub struct CrateVersionOrInfo {
    #[serde(with = "either::serde_untagged")]
    pub ver_or_info: Either<CrateVersion, CrateInfo>,
}

#[derive(Deserialize)]
pub struct Cargo {
    pub dependencies: HashMap<String, CrateVersionOrInfo>,
}
