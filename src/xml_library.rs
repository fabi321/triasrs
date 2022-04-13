use std::{sync::Arc, collections::HashMap};
use tokio::sync::Mutex;
use crate::{xml_document::XMLDocument, API};


pub struct XMLLibrary {
    storage: Arc<Mutex<HashMap<String, Arc<Mutex<XMLDocument>>>>>
}

impl XMLLibrary {
    pub fn new() -> XMLLibrary {
        let storage = Arc::new(Mutex::new(HashMap::new()));
        XMLLibrary {storage}
    }

    pub async fn get_document(&self, name: String, api: &API) -> Result<Arc<String>, crate::Error> {
        let document = {
            self
                .storage
                .lock()
                .await
                .entry(name.clone())
                .or_insert_with(|| Arc::new(Mutex::new(XMLDocument::new(name))))
                .clone()
        };
        let mut unlocked_document = document.lock().await;
        unlocked_document.get_content(api).await
    }
}
