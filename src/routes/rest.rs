use crate::db::fs;
use crate::guards::auth::Token;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Data {
    data: String,
}

#[post("/<table>/<name>", data = "<data>", format = "json")]
pub fn create(_tk: Token, data: Json<Data>, table: &str, name: &str) {
    let d = data.into_inner();

    return fs::write(table.to_string(), name.to_string(), d.data).unwrap();
}

#[get("/<table>/<name>")]
pub fn read(_tk: Token, table: &str, name: &str) -> String {
    return fs::read(table.to_string(), name.to_string()).unwrap();
}

#[delete("/<table>/<name>")]
pub fn delete(_tk: Token, table: &str, name: &str) -> String {
    return fs::delete(table.to_string(), name.to_string()).unwrap();
}
