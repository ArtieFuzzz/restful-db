#![feature(proc_macro_hygiene, decl_macro)]
#![feature(const_fmt_arguments_new)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

mod catchers;
mod config;
mod db;
mod guards;
mod routes;
mod utils;

use catchers::*;
use routes::rest;
use routes::root;
use utils::generate_admin_token;

use rocket::{config::Ident, Config};

use std::env;

lazy_static! {
    static ref BASE_DIR: String = format!(
        "{}\\{}",
        env::current_dir().unwrap().to_str().unwrap().to_string(),
        "rdb"
    );
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
        .mount("/rest/", routes![rest::create, rest::read, rest::delete])
}
