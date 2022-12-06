use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Instance {
    /// The name of this instance.
    pub name: String,
    /// The location of this instance.
    pub location: PathBuf,
    /// The icon of this instance.
    pub icon: InstanceIcon,
    /// A list of versions this instance is using.
    pub versions: Vec<VersionEntry>,
    /// A list of mods this instance is using.
    pub mods: Vec<ModEntry>,
}

/// An enumeration of icon types.
#[derive(Debug, Serialize, Deserialize)]
pub enum InstanceIcon {
    /// The default icon type.
    Default,
    /// A custom icon type, with the path to the icon file.
    Custom(PathBuf),
}

/// A version entry for a given instance.
#[derive(Debug, Serialize, Deserialize)]
struct VersionEntry {
    /// The name of the version.
    pub name: String,
    /// The version of the version.
    pub version: String,
}

/// A mod this instance has loaded.
#[derive(Debug, Serialize, Deserialize)]
struct ModEntry {
    /// Whether this mod is enabled.
    pub enabled: bool,
}
