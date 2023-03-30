use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JdkConfigManifest {
    /// The default JDK to use when creating instances.
    default: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ZonoConfig {
    /// The version of the config manifest.
    version: u8,
    /// The JDK configuration.
    jdks: JdkConfigManifest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InstanceReference {
    /// The path to the instance.
    path: PathBuf,
    /// The path to the icon of the instance.
    icon: PathBuf,
}
