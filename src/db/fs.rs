use std::env;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

lazy_static! {
    static ref BASE_DIR: String = format!(
        "{}\\{}",
        env::current_dir().unwrap().to_str().unwrap().to_string(),
        "rdb"
    );
}

pub fn write(table: String, name: String, data: String) -> Result<(), Box<dyn Error>> {
    let file_name = format!("{}\\{}_{}", BASE_DIR.to_string(), table, name);
    let file_path = Path::new(&file_name);

    if !file_path.exists() {
        fs::File::create(file_path)?;
    }

    let mut file = OpenOptions::new()
        .write(false)
        .append(true)
        .open(file_path)?;

    file.write_all(format!("{:?}", data).as_bytes()).unwrap();

    Ok(())
}

pub fn delete(table: String, name: String) -> Result<String, Box<dyn Error>> {
    let file_name = format!("{}\\{}_{}", BASE_DIR.to_string(), table, name);
    let file_path = Path::new(&file_name);

    if !file_path.exists() {
        return Ok("file does not exist".to_string());
    }

    fs::remove_file(file_path)?;

    return Ok("removed".to_string());
}

pub fn read(table: String, name: String) -> Result<String, Box<dyn Error>> {
    let file_name = format!("{}\\{}_{}", BASE_DIR.to_string(), table, name);
    let file_path = Path::new(&file_name);

    if !file_path.exists() {
        return Ok("file does not exist".to_string());
    }

    let content = fs::read_to_string(file_path)?;

    Ok(content)
}
