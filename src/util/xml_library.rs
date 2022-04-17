use std::sync::Arc;
use sxd_document::Package;

use crate::API;
use crate::util::mut_hash_map::MutHashMap;
use crate::util::xml_document::XMLDocument;

pub struct XMLLibrary {
    storage: MutHashMap<Arc<String>, XMLDocument>,
}

impl XMLLibrary {
    pub fn new() -> XMLLibrary {
        let storage = MutHashMap::new();
        XMLLibrary { storage }
    }

    pub async fn get_document(&self, name: Arc<String>, api: &API) -> Result<(Arc<String>, Arc<Package>), crate::Error> {
        let document = self.storage.get_copy(name).await;
        let mut unlocked_document = document.lock().await;
        unlocked_document.get_content(api).await
    }
}
