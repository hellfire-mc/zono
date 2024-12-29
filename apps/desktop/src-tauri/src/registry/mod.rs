//! This module defines traits and structs for accessing the various modding
//! registries, such as CurseForge and Modrinth.

use async_trait::async_trait;
use thiserror::Error;
use time::OffsetDateTime;

mod curseforge;
mod modrinth;

#[async_trait]
pub trait ModRegistry {
    async fn get_mod(&self, id: &str) -> Result<Mod, ModRegistryError>;
    async fn search_mods(&self, query: &str) -> Result<Vec<Mod>, ModRegistryError>;
}

/// A mod from a registry.
pub struct Mod {
    // The mod's ID.
    pub id: String,
    /// The mod's name.
    pub name: String,
    /// A description of the mod.
    pub description: String,
    /// A list of authors of the mod.
    pub authors: Vec<String>,
    /// The list of authors of the mod.
    pub versions: Vec<ModVersion>,
}

impl Mod {
    fn get_latest_version(&self) -> &ModVersion {
        self.versions.first().unwrap()
    }
}

#[derive(Error, Debug)]
pub enum ModRegistryError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("JSON parse error: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("unknown data store error")]
    Unknown,
}

pub struct ModVersion {
    pub id: String,
    pub name: String,
    pub description: String,
    pub downloads: u64,
    pub date_published: OffsetDateTime,
    pub files: Vec<ModFile>,
}

pub struct ModFile {
    pub id: String,
    pub name: String,
    pub download_url: String,
    pub date_published: String,
    pub game_versions: Vec<String>,
    pub file_type: ModFileType,
}

pub enum ModFileType {
    Mod,
    Modpack,
    ResourcePack,
    World,
}
