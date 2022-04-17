use std::sync::Arc;

use chrono::{DateTime, Utc};
use sxd_document::{Package, parser};

use crate::util::default_id::DefaultId;

struct Document {
    document: Arc<String>,
    package: Arc<Package>,
    last_updated: DateTime<Utc>,
}

pub struct XMLDocument {
    name: Arc<String>,
    document: Option<Document>,
}

impl XMLDocument {
    pub fn new(name: Arc<String>) -> XMLDocument {
        XMLDocument { name, document: None }
    }

    pub async fn get_content(&mut self, api: &crate::API) -> Result<(Arc<String>, Arc<Package>), crate::Error> {
        if let Some(document) = &self.document {
            if Utc::now() - document.last_updated > api.refresh_interval {
                self.document = None;
            }
        }
        if self.document.is_none() {
            let document = Arc::new(if cfg!(test) {
                unimplemented!("add test mock for XMLDocument::get_content")
            } else {
                let res = api.client.post(&api.base_url)
                    .body(self.name.as_str().to_string())
                    .header("Content-Type", "text/xml")
                    .send()
                    .await?;
                res.text().await?
            });
            let package = Arc::new(parser::parse(document.as_str())?);
            self.document = Some(Document {
                document,
                package,
                last_updated: Utc::now(),
            });
        }
        if let Some(ref document) = self.document {
            Ok((document.document.clone(), document.package.clone()))
        } else {
            unreachable!("It should not be possible")
        }
    }
}

impl DefaultId for XMLDocument {
    type Id = Arc<String>;

    fn default(id: Arc<String>) -> Self {
        XMLDocument::new(id)
    }
}