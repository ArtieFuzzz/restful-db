use crate::db::cache;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    static ref BASE_DIR: String = format!(
        "{}\\{}",
        env::current_dir().unwrap().to_str().unwrap().to_string(),
        "rdb"
    );
    static ref CACHE: Mutex<cache::ReadCache> = Mutex::new({
        let mut c = cache::ReadCache::new();

        c
    });
}

pub fn create(key: String, data: String) -> Result<(), Box<dyn Error>> {
    let file_name = format!("{}\\{}", BASE_DIR.to_string(), key);
    let file_path = Path::new(&file_name);

    if !file_path.exists() {
        fs::File::create(file_path)?;
    }

    let mut file = OpenOptions::new()
        .write(false)
        .append(true)
        .open(file_path)?;

    file.write_all(data.as_bytes()).unwrap();

    Ok(())
}

pub fn delete(key: String) -> Result<String, Box<dyn Error>> {
    let file_name = format!("{}\\{}", BASE_DIR.to_string(), key);
    let file_path = Path::new(&file_name);

    if !file_path.exists() {
        return Ok("file does not exist".to_string());
    }

    fs::remove_file(file_path)?;

    return Ok("removed".to_string());
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

    let content = fs::read_to_string(file_path)?;

    CACHE.lock()?.add(key, content.clone());

    Ok(content)
}
