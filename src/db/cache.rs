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

    pub fn add(&mut self, key: String, value: String) -> bool {
        if self.exists(key.clone()) {
            return false;
        } else {
            let _ = &self
                .cache
                .write()
                .ok()
                .and_then(|mut g| g.insert(key, value));
        }

        return true;
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
            .and_then(|g| {
                let gs = &key.into();
                Some(g.contains_key(gs))
            })
            .unwrap()
        {
            return true;
        } else {
            return false;
        }
    }
}
