use std::fs::File;

const FAVICON_PATH: &'static str = "res/favicon.ico";

#[get("/favicon.ico")]
pub fn favicon() -> Option<File> {
    File::open(FAVICON_PATH).ok()
}
