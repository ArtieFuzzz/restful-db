use serde::{Deserialize, Serialize};
use serde_yaml;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub key: String,
    pub admin_name: String,
}

pub fn read_config() -> Result<Config, Box<dyn Error>> {
    let base = format!(
        "{}\\{}",
        env::current_dir().unwrap().to_str().unwrap().to_string(),
        "config.yml"
    );
    let path = Path::new(&base);

    if !path.exists() {
        println!("{}", "Config does not exist... Creating...");
        File::create(path).expect("Could not write new config file");

        let w = OpenOptions::new()
            .write(true)
            .open(path)
            .expect("Could not write new config file");

        let conf = Config {
            key: "".to_owned(),
            admin_name: "admin".to_owned(),
        };

        serde_yaml::to_writer(w, &conf).expect("Failed to write config file");
    }

    let r = File::open(path).expect("Cannot open config");

    let config: Config = serde_yaml::from_reader(r).expect("Failed to read config");

    return Ok(config);
}
