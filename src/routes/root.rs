#[get("/")]
pub fn home() -> &'static str {
    return "Welcome to the wall.";
}
