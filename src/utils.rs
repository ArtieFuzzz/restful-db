use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn generate_admin_token(username: String, string_key: String) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(string_key.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();

    claims.insert("admin", "yes");
    claims.insert("sub", &username);

    let signed_token = claims.sign_with_key(&key).unwrap();

    return signed_token;
}
