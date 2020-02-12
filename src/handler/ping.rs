#[get("/")]
pub fn ping() -> String {
    "pong".to_owned()
}
