#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

use crate::endpoints::*;

mod about;
mod endpoints;
mod home;
mod favicon;
mod responder;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().attach(Template::fairing()).mount(
        "/",
        routes![
            index,
            favicon::favicon,
            home::home,
            about::about,
            temperature,
            humidity,
            led_status
        ],
    )
}
