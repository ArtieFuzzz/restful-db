use crate::db::operations;
use crate::guards::auth::Token;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Data {
    data: String,
}

macro_rules! response {
    ($status:expr, $message:literal) => {
        ($status, $message)
    };
}

#[post("/<key>", data = "<data>", format = "json")]
pub fn create(_tk: Token, data: Json<Data>, key: &str) -> (Status, &'static str) {
    let d = data.into_inner();

    let success = operations::create(key.to_string(), d.data).unwrap();

    if !success {
        return response!(Status::Conflict, "file already exists");
    }

    return response!(Status::Ok, "");
}

#[get("/<key>")]
pub fn read(_tk: Token, key: &str) -> String {
    return operations::read(key.to_string()).unwrap();
}

#[delete("/<key>")]
pub fn delete(_tk: Token, key: &str) -> String {
    return operations::delete(key.to_string()).unwrap();
}
