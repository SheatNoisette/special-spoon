#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::{database, diesel};

use crate::endpoints::*;

mod about;
mod db_model;
mod endpoints;
mod favicon;
mod home;
mod responder;

#[database("sqlite_values")]
pub struct ValueDbConnection(diesel::SqliteConnection);

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(ValueDbConnection::fairing())
        .attach(Template::fairing())
        .mount(
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
