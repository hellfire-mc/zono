//! Contains definitions for mod manifest files and their deserialization.

use std::{
    convert::Infallible,
    fs::{self, File},
    io::{self, BufReader, Read},
    path::Path,
};

use serde::Deserialize;
use thiserror::Error;
use zip::{result::ZipError, ZipArchive};

/// An array of identifiers to mod files.
static IDENTIFIERS: [&str; 3] = ["quilt.mod.json", "fabric.mod.json", "META-INF/mods.toml"];

/// Read the jar file at the given path.
pub fn read_jar_file<P: AsRef<Path>>(path: P) -> Result<ModMetadata, ManifestError> {
    let file = File::open(path.as_ref()).map_err(ManifestError::IoError)?;
    let mut zip = ZipArchive::new(BufReader::new(file)).map_err(ManifestError::JairUnzipFail)?;

    // for every identifier, check if it exists
    for identifier in IDENTIFIERS {
        let mut entry = match zip.by_name(identifier) {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        // read entry
        let mut entry_contents = String::with_capacity(
            entry.size().try_into().expect("file size too large for current platform"),
        );
        entry.read_to_string(&mut entry_contents).map_err(ManifestError::IoError)?;

        // deserialize the manifest into the generic version
        let file: ModMetadata = match identifier {
            "fabric.mod.json" => FabricManifest::from_str(entry_contents)?
                .try_into()
                .map_err(|_| ManifestError::DeserializeError)?,
            "quilt.mod.json" => QuiltManifest::from_str(entry_contents)?
                .try_into()
                .map_err(|_| ManifestError::DeserializeError)?,
            "META-INF/mods.toml" => ForgeManifest::from_str(entry_contents)?
                .try_into()
                .map_err(|_| ManifestError::DeserializeError)?,
            _ => unreachable!(),
        };

        return Ok(file);
    }

    Err(ManifestError::MissingMetadataFile)
}

/// Metadata for a mod.
pub struct ModMetadata {
    pub kind: ModKind,
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
}

/// An enumeration of mod kinds.
#[derive(Debug, Deserialize)]
pub enum ModKind {
    Fabric,
    Forge,
    Quilt,
}

/// An enumeration of error types that can be raised during metadata reading.
#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("encountered an IO error")]
    IoError(#[from] io::Error),
    #[error("failed to unzip jar file")]
    JairUnzipFail(#[from] ZipError),
    #[error("failed to find a valid metadata file")]
    MissingMetadataFile,
    #[error("failed to deserialize manifest")]
    DeserializeError,
    #[error("empty forge manifest")]
    EmptyForgeManifest,
}

/// Utility trait for reading and deserializing manifests.
pub trait Manifest: Sized + TryInto<ModMetadata> {
    /// Read and return a manifest from a given path.
    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ManifestError> {
        let contents = fs::read_to_string(path)?;
        Self::from_str(contents)
    }

    /// Read and return a manifest from a string.
    fn from_str<S: AsRef<str>>(str: S) -> Result<Self, ManifestError>;
}

#[derive(Debug, Deserialize)]
pub struct FabricManifest {
    /// Defines the mod's identifier.
    pub id: String,
    /// Define's the mod's version.
    pub version: String,
    /// The name of the mod.
    pub name: Option<String>,
    /// A description of the mod.
    pub description: Option<String>,
}

impl TryInto<ModMetadata> for FabricManifest {
    type Error = Infallible;

    fn try_into(self) -> Result<ModMetadata, Infallible> {
        Ok(ModMetadata {
            description: self.description,
            id: self.id.clone(),
            kind: ModKind::Fabric,
            name: self.name.unwrap_or_else(|| self.id.clone()),
            version: self.version,
        })
    }
}

impl Manifest for FabricManifest {
    fn from_str<S: AsRef<str>>(s: S) -> Result<FabricManifest, ManifestError> {
        serde_json::from_str(s.as_ref()).map_err(|_| ManifestError::DeserializeError)
    }
}

/// A manifest file for a Forge mod.
#[derive(Debug, Deserialize)]
pub struct ForgeManifest {
    /// A list of mods contained in this jar.
    pub mods: Vec<ForgeMod>,
}

impl TryInto<ModMetadata> for ForgeManifest {
    type Error = ManifestError;

    fn try_into(self) -> Result<ModMetadata, Self::Error> {
        let mod_data = self.mods.first().ok_or(ManifestError::EmptyForgeManifest)?;
        Ok(ModMetadata {
            description: mod_data.description.clone(),
            id: mod_data.mod_id.clone(),
            kind: ModKind::Forge,
            name: mod_data.display_name.clone().unwrap_or_else(|| mod_data.mod_id.clone()),
            version: mod_data.version.clone(),
        })
    }
}

impl Manifest for ForgeManifest {
    fn from_str<S: AsRef<str>>(s: S) -> Result<ForgeManifest, ManifestError> {
        toml::from_str(s.as_ref()).map_err(|_| ManifestError::DeserializeError)
    }
}

/// A mod specified in the manifest file.
#[derive(Debug, Deserialize)]
pub struct ForgeMod {
    /// The ID of the mod.
    #[serde(rename = "modId")]
    pub mod_id: String,
    /// The version of the mod.
    pub version: String,
    /// The display name of the mod.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// The description of the mod.
    pub description: Option<String>,
}

/// A manifest file for a Quilt mod. See [the manifest RFC](https://github.com/QuiltMC/rfcs/blob/main/specification/0002-quilt.mod.json.md).
#[derive(Debug, Deserialize)]
pub struct QuiltManifest {
    /// Information related to loading the mod.
    quilt_loader: QuiltManifestLoader,
}

impl TryInto<ModMetadata> for QuiltManifest {
    type Error = Infallible;

    fn try_into(self) -> Result<ModMetadata, Infallible> {
        Ok(ModMetadata {
            description: self
                .quilt_loader
                .metadata
                .as_ref()
                .map(|metadata| metadata.description.clone()),
            id: self.quilt_loader.id.clone(),
            kind: ModKind::Quilt,
            name: self
                .quilt_loader
                .metadata
                .map(|metadata| metadata.name)
                .unwrap_or_else(|| self.quilt_loader.id.clone()),
            version: self.quilt_loader.version,
        })
    }
}

impl Manifest for QuiltManifest {
    fn from_str<S: AsRef<str>>(s: S) -> Result<QuiltManifest, ManifestError> {
        serde_json::from_str(s.as_ref()).map_err(|_| ManifestError::DeserializeError)
    }
}

/// Information related to loading the mod.
#[derive(Debug, Deserialize)]

pub struct QuiltManifestLoader {
    /// The ID of the mod.
    pub id: String,
    /// The group of the mod.
    pub group: String,
    /// The version of the mod.
    pub version: String,
    /// Extra information about this mod and/or its authors
    pub metadata: Option<QuiltModMetadata>,
}

/// Extra information about this mod and/or its authors
#[derive(Debug, Deserialize, Default)]
pub struct QuiltModMetadata {
    pub name: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    pub fn test_read_fabric_jar() -> Result<(), ManifestError> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("./tests/assets/jars/fabric-api-0.68.0+1.19.2.jar");
        let metadata = read_jar_file(path)?;

        assert_eq!("fabric-api", metadata.id);
        assert_eq!("Fabric API".to_string(), metadata.name);
        assert_eq!("0.68.0+1.19.2".to_string(), metadata.version);

        Ok(())
    }

    #[test]
    pub fn test_read_forge_jar() -> Result<(), ManifestError> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("./tests/assets/jars/cloth-config-8.2.88-forge.jar");
        let metadata = read_jar_file(path)?;

        assert_eq!("cloth_config", metadata.id);
        assert_eq!(Some("An API for config screens.\n".into()), metadata.description);

        Ok(())
    }

    #[test]
    pub fn test_read_quilt_jar() -> Result<(), ManifestError> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("./tests/assets/jars/ok_zoomer-5.0.0-beta.9+1.19.jar");
        let metadata = read_jar_file(path)?;

        assert_eq!("ok_zoomer", metadata.id);
        assert_eq!("Ok Zoomer", metadata.name);

        Ok(())
    }
}
