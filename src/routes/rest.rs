use crate::db::operations;
use crate::guards::auth::Token;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub data: String,
}

macro_rules! response {
    ($status:expr, $message:literal) => {
        ($status, $message)
    };
}

#[post("/<key>", data = "<data>", format = "json")]
pub fn write(_tk: Token, data: Json<Data>, key: String) -> (Status, &'static str) {
    let d = data.into_inner();

    let success = operations::write(key, d.data, false).unwrap();

    if !success {
        return response!(Status::Conflict, "file already exists");
    }

    return response!(Status::Created, "");
}

#[put("/<key>", data = "<data>", format = "json")]
pub fn overwrite(_tk: Token, data: Json<Data>, key: String) -> (Status, &'static str) {
    let d = data.into_inner();

    operations::write(key, d.data, true).unwrap();

    return response!(Status::Ok, "");
}

#[get("/<key>")]
pub fn read(_tk: Token, key: String) -> String {
    return operations::read(key).unwrap();
}

#[delete("/<key>")]
pub fn delete(_tk: Token, key: String) -> &'static str {
    return operations::delete(key).unwrap();
}
