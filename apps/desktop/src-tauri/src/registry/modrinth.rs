use lazy_static::lazy_static;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{Mod, ModRegistry, ModRegistryError};

const BASE_URL: &'static str = "https://api.modrinth.com/v2";

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

#[derive(Default)]
pub struct ModrinthRegistry;

#[async_trait::async_trait]
impl ModRegistry for ModrinthRegistry {
    async fn get_mod(&self, id: &str) -> Result<Mod, ModRegistryError> {
        let res: Project =
            reqwest::get(format!("{}/project/{}", BASE_URL, id)).await?.json().await?;

        Ok(res.into())
    }

    async fn search_mods(&self, query: &str) -> Result<Vec<Mod>, ModRegistryError> {
        let req =
            CLIENT.get(format!("{}{}", BASE_URL, "/search")).query(&[("query", query)]).build()?;
        let res = CLIENT.execute(req).await?.json::<SearchResponse>().await?;
        Ok(res.hits.into_iter().map(Into::into).collect())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub hits: Vec<SearchHit>,
    pub offset: i64,
    pub limit: i64,
    #[serde(rename = "total_hits")]
    pub total_hits: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchHit {
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "project_type")]
    pub project_type: String,
    pub slug: String,
    pub author: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    #[serde(rename = "display_categories")]
    pub display_categories: Vec<String>,
    pub versions: Vec<String>,
    pub downloads: i64,
    pub follows: i64,
    #[serde(rename = "icon_url")]
    pub icon_url: String,
    #[serde(rename = "date_created")]
    pub date_created: String,
    #[serde(rename = "date_modified")]
    pub date_modified: String,
    #[serde(rename = "latest_version")]
    pub latest_version: String,
    pub license: String,
    #[serde(rename = "client_side")]
    pub client_side: String,
    #[serde(rename = "server_side")]
    pub server_side: String,
    pub gallery: Vec<String>,
    #[serde(rename = "featured_gallery")]
    pub featured_gallery: Option<String>,
    pub color: Option<i64>,
}

impl From<SearchHit> for Mod {
    fn from(value: SearchHit) -> Self {
        Self {
            authors: vec![value.author],
            description: value.description,
            id: value.project_id,
            name: value.title,
            versions: vec![],
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    #[serde(rename = "client_side")]
    pub client_side: String,
    #[serde(rename = "server_side")]
    pub server_side: String,
    pub body: String,
    #[serde(rename = "additional_categories")]
    pub additional_categories: Vec<String>,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "source_url")]
    pub source_url: String,
    #[serde(rename = "wiki_url")]
    pub wiki_url: String,
    #[serde(rename = "discord_url")]
    pub discord_url: String,
    #[serde(rename = "donation_urls")]
    pub donation_urls: Vec<DonationUrl>,
    #[serde(rename = "project_type")]
    pub project_type: String,
    pub downloads: Option<i64>,
    #[serde(rename = "icon_url")]
    pub icon_url: String,
    pub color: Option<i64>,
    pub id: String,
    pub team: String,
    #[serde(rename = "body_url")]
    pub body_url: Option<String>,
    #[serde(rename = "moderator_message")]
    pub moderator_message: Option<String>,
    pub published: String,
    pub updated: String,
    pub approved: String,
    pub followers: Option<i64>,
    pub status: String,
    pub license: License,
    pub versions: Vec<String>,
    #[serde(rename = "game_versions")]
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub gallery: Vec<Gallery>,
}

impl From<Project> for Mod {
    fn from(value: Project) -> Self {
        Self {
            authors: vec![value.team],
            description: value.description,
            id: value.id,
            name: value.title,
            versions: vec![],
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DonationUrl {
    pub id: String,
    pub platform: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gallery {
    pub url: String,
    pub featured: bool,
    pub title: String,
    pub description: String,
    pub created: String,
    pub ordering: i64,
}

#[cfg(test)]
mod tests {
    use crate::registry::{ModRegistry, ModRegistryError};

    #[tokio::test]
    async fn test_search_mods() -> Result<(), ModRegistryError> {
        let registry = super::ModrinthRegistry::default();
        let mods = registry.search_mods("fabric").await?;
        assert!(mods.len() > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_mod() -> Result<(), ModRegistryError> {
        let registry = super::ModrinthRegistry::default();
        let mod_response = registry.get_mod("fabric-api").await?;

        assert_eq!(mod_response.id, "fabric-api");

        Ok(())
    }
}
