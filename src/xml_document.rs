use std::sync::Arc;

use chrono::{DateTime, Utc};
use reqwest;

struct Document {
    document: Arc<String>,
    last_updated: DateTime<Utc>,
}

pub struct XMLDocument {
    name: String,
    document: Option<Document>,
}

impl XMLDocument {
    pub fn new(name: String) -> XMLDocument {
        XMLDocument {name, document:None}
    }

    pub async fn get_content(&mut self, api: &crate::API) -> Result<Arc<String>, crate::Error> {
        if let Some(document) = &self.document {
            if Utc::now() - document.last_updated > api.refresh_interval {
                self.document = None;
            }
        }
        if let None = self.document {
            let res = reqwest::get(&self.name).await?;
            let document = Arc::new(res.text().await?);
            self.document = Some(Document {
                document,
                last_updated: Utc::now()
            });
        }
        if let Some(ref document) = self.document {
            return Ok(document.document.clone());
        } else {
            unreachable!("It should not be possible")
        }
    }
}