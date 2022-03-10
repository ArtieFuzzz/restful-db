use crate::config;
use crate::db::cache;
use crate::db::utils::{de, en};
use crate::rest::Data;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    static ref CONFIG: config::Config = config::read_config().unwrap();
    static ref BASE_DIR: String = format!("{}", CONFIG.path);
    static ref CACHE: Mutex<cache::ReadCache> = Mutex::new({
        #[allow(unused_mut)]
        let mut c = cache::ReadCache::new();

        c
    });
}

pub fn write(key: String, data: String, overwrite: bool) -> Result<bool, Box<dyn Error>> {
    let file_name = format!("{}\\{}", BASE_DIR.to_string(), key);
    let file_path = Path::new(&file_name);

    if file_path.exists() && !overwrite {
        return Ok(false);
    }

    fs::File::create(file_path)?;

    let mut file = OpenOptions::new()
        .write(overwrite)
        .append(true)
        .open(file_path)?;

    file.write_all(&(en(Data { data }))?.to_owned())?;

    if overwrite {
        CACHE.lock()?.delete(key);
    }

    return Ok(true);
}

pub fn delete(key: String) -> Result<&'static str, Box<dyn Error>> {
    let file_name = format!("{}\\{}", BASE_DIR.to_string(), key);
    let file_path = Path::new(&file_name);

    if !file_path.exists() {
        return Ok("file does not exist");
    }

    fs::remove_file(file_path)?;

    return Ok("removed file");
}

pub fn read(key: String) -> Result<String, Box<dyn Error>> {
    if CACHE.lock()?.exists(key.clone()) {
        return Ok(CACHE.lock()?.get(key));
    }

    let file_name = format!("{}\\{}", BASE_DIR.to_string(), key);
    let file_path = Path::new(&file_name);

    if !file_path.exists() {
        return Ok("file does not exist".to_string());
    }

    let d = de(fs::read_to_string(file_path)?.into_bytes())?;

    CACHE.lock()?.add(key, d.data.clone());

    Ok(d.data)
}
