#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate dotenv;

use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

use crate::endpoints::*;

mod about;
mod endpoints;
mod home;
mod favicon;
mod responder;
mod db_model;
mod db;

#[launch]
fn rocket() -> Rocket<Build> {

    let conn = db::establish_connection();


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
