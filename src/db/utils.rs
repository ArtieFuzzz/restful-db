use base64::{decode, encode};
use std::str;

pub fn en(data: String) -> String {
    return encode(data);
}

pub fn de(data: String) -> String {
    let bytes = &decode(data).unwrap()[..];
    return String::from(str::from_utf8(bytes).unwrap());
}
