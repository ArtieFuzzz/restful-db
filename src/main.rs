#![feature(proc_macro_hygiene, decl_macro)]
#![feature(const_fmt_arguments_new)]
#![feature(option_result_contains)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

extern crate base64;

mod catchers;
mod config;
mod db;
mod guards;
mod routes;

use catchers::*;
use routes::rest;
use routes::root;

use rocket::{config::Ident, Config};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::env;

lazy_static! {
    static ref BASE_DIR: String = format!(
        "{}\\{}",
        env::current_dir().unwrap().to_str().unwrap().to_string(),
        "rdb"
    );
}

fn generate_admin_token(username: String, string_key: String) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(string_key.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();

    claims.insert("admin", "yes");
    claims.insert("sub", &username);

    let signed_token = claims.sign_with_key(&key).unwrap();

    return signed_token;
}

#[launch]
fn rocket() -> _ {
    let config = config::read_config().unwrap();

    let token = generate_admin_token(config.admin_name, config.key);

    println!("{}", token);

    std::fs::create_dir_all(BASE_DIR.to_string()).unwrap();

    let config = Config {
        port: 7892,
        keep_alive: 60,
        ident: Ident::none(),
        ..Config::debug_default()
    };

    rocket::build()
        .configure(config)
        .register(
            "/",
            catchers![
                not_found,
                backend_error,
                unauthorized,
                forbidden,
                broken_request
            ],
        )
        .mount("/", routes![root::home])
        .mount(
            "/rest/",
            routes![rest::create, rest::read, rest::delete, rest::put],
        )
}
