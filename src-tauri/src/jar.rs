use std::{convert::Infallible, fs, io, path::Path};

use serde::Deserialize;
use thiserror::Error;

/// An array of identifiers to mod files.
static IDENTIFIERS: [&str; 3] = ["fabric.mod.json", "quilt.mod.json", "mods.toml"];

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
    #[error("failed to read manifest file")]
    ReadError(#[from] io::Error),
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
    type Error = ManifestError;

    fn try_into(self) -> Result<ModMetadata, Self::Error> {
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
    use std::error::Error;

    use pretty_assertions::assert_eq;

    use crate::jar::{ForgeManifest, Manifest, ModMetadata, FabricManifest, QuiltManifest};
	
	#[test]
	pub fn test_deserialize_fabric_metadata() -> Result<(), Box<dyn Error>> {
		let metadata: ModMetadata =
			FabricManifest::from_str(include_str!("../tests/assets/fabric.mod.json"))?
				.try_into()?;
		assert_eq!(metadata.id, "fabric-api-base");
		assert_eq!(metadata.name, "Fabric API Base".to_string());
		assert_eq!(metadata.version, "${version}".to_string());

		Ok(())
	}

    #[test]
    pub fn test_deserialize_forge_metadata() -> Result<(), Box<dyn Error>> {
        let metadata: ModMetadata =
            ForgeManifest::from_str(include_str!("../tests/assets/mods.toml"))?.try_into()?;

        assert_eq!(metadata.id, "cloth_config");
        assert_eq!(metadata.description, Some("An API for config screens.\n".into()));

        Ok(())
    }


	#[test]
	pub fn test_deserialize_quilt_metadata() -> Result<(), Box<dyn Error>> {
		let metadata: ModMetadata =
            QuiltManifest::from_str(include_str!("../tests/assets/quilt.mod.json"))?.try_into()?;

        assert_eq!(metadata.id, "ok_zoomer");
        assert_eq!(metadata.name, "Ok Zoomer");

        Ok(())
	}
}
