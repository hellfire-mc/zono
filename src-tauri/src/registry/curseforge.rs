use super::{Mod, ModRegistry, ModRegistryError};

pub struct CurseForgeRegistry;

#[async_trait::async_trait]
impl ModRegistry for CurseForgeRegistry {
    async fn get_mod(&self, id: &str) -> Result<Mod, ModRegistryError> {
        todo!()
    }

    async fn search_mods(&self, query: &str) -> Result<Vec<Mod>, ModRegistryError> {
        todo!()
    }
}
