use std::{collections::HashMap, future::Future, hash::Hash, sync::Arc};

use tokio::sync::{Mutex, MutexGuard};

use crate::util::default_id::DefaultId;

pub struct MutHashMap<Id, D> where
    Id: Send + Sync + Clone + Eq + Hash,
    D: DefaultId<Id=Id>
{
    outer: Arc<Mutex<HashMap<Id, Arc<Mutex<D>>>>>,
}

impl<Id, D> MutHashMap<Id, D> where
    Id: Send + Sync + Clone + Eq + Hash,
    D: DefaultId<Id=Id>
{
    pub fn new() -> MutHashMap<Id, D> {
        let outer = Arc::new(Mutex::new(HashMap::new()));
        MutHashMap { outer }
    }

    pub async fn get_copy(&self, key: Id) -> Arc<Mutex<D>> {
        self
            .outer
            .lock()
            .await
            .entry(key.clone())
            .or_insert_with(|| Arc::new(Mutex::new(D::default(key))))
            .clone()
    }

    pub async fn execute_on_copy<O, Fut: Future<Output=O>, F: FnOnce(MutexGuard<D>) -> Fut>(&self, key: Id, closure: F) -> O {
        let copy = self.get_copy(key).await;
        let unlocked: MutexGuard<D> = copy.lock().await;
        closure(unlocked).await
    }
}