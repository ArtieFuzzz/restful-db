use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

type Cache = HashMap<String, String>;
type CacheRecords = Arc<RwLock<Cache>>;

pub struct ReadCache {
    pub cache: CacheRecords,
}

impl ReadCache {
    pub fn new() -> ReadCache {
        return ReadCache {
            cache: Arc::new(RwLock::new(Cache::new())),
        };
    }

    pub fn add(&mut self, key: String, value: String) {
        if self.exists(key.clone()) {
            return;
        } else {
            let _ = &self
                .cache
                .write()
                .ok()
                .and_then(|mut g| g.insert(key, value));
        }

        return;
    }

    pub fn get(&self, key: String) -> String {
        if self.exists(key.clone()) {
            let v = self
                .cache
                .read()
                .ok()
                .and_then(|g| g.get::<String>(&key.into()).map(|val| val.clone()))
                .unwrap();

            return String::from(v.as_str());
        } else {
            return String::from("Does not exist");
        }
    }

    pub fn exists<S: Into<String>>(&self, key: S) -> bool {
        if self
            .cache
            .read()
            .ok()
            .and_then(|g| Some(g.contains_key(&key.into())))
            .unwrap()
        {
            return true;
        } else {
            return false;
        }
    }
}
