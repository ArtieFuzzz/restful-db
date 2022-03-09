use crate::db::fs;
use crate::guards::auth::Token;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Data {
    data: String,
}

#[post("/<key>", data = "<data>", format = "json")]
pub fn create(_tk: Token, data: Json<Data>, key: &str) -> (Status, &'static str) {
    let d = data.into_inner();

    let success = fs::create(key.to_string(), d.data).unwrap();

    if !success {
        return (Status::Conflict, "file already exists.");
    }

    return (Status::Ok, "file created.");
}

#[get("/<key>")]
pub fn read(_tk: Token, key: &str) -> String {
    return fs::read(key.to_string()).unwrap();
}

#[delete("/<key>")]
pub fn delete(_tk: Token, key: &str) -> String {
    return fs::delete(key.to_string()).unwrap();
}
