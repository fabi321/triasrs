use std::sync::Arc;

use tokio::sync::Mutex;

pub struct MutField<D> {
    storage: Arc<Mutex<D>>,
}

impl<D> MutField<D> {
    pub fn new(data: D) -> MutField<D> {
        let storage = Arc::new(Mutex::new(data));
        MutField { storage }
    }
}