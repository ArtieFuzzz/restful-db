#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod catchers;
mod guards;
mod routes;
mod utils;

use catchers::*;
use routes::root;
use utils::generate_admin_token;

use clap::Parser;
use rocket::{config::Ident, Config};

#[derive(Parser, Debug)]
#[clap(author = "ArtieFuzzz", about = "A database that uses a RESTFUL API for communication", long_about = None)]
struct Args {
    #[clap(short, long)]
    admin_name: String,
    #[clap(short, long)]
    key: String,
}

#[launch]
fn rocket() -> _ {
    let args = Args::parse();

    let admin = generate_admin_token(args.admin_name, args.key);

    println!("{:?}", admin);

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
}
