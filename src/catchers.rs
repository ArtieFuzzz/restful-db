use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("{} Is an invalid route.", req.uri())
}

#[catch(500)]
pub fn backend_error(_req: &Request) -> String {
    format!("{}", "Internal Server Error...")
}

#[catch(401)]
pub fn unauthorized(_req: &Request) -> String {
    format!("{}", "Unauthorized.")
}

#[catch(403)]
pub fn forbidden(_req: &Request) -> String {
    format!("{}", "Forbidden...")
}

#[catch(400)]
pub fn broken_request(_req: &Request) -> String {
    format!(
        "{}",
        "Bad request... Please verify your request and try again"
    )
}
