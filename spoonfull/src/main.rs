#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket::log::PaintExt;
use rocket::yansi::Paint;
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;

use librumqttd::Config;
use std::path::PathBuf;
use std::thread;

use mqtt::publisher;

use crate::endpoints::*;

mod about;
mod endpoints;
mod favicon;
mod home;
mod responder;

mod mqtt;

mod db_model;
mod db_schema;

#[database("sqlite_values")]
pub struct ValueDbConnection(diesel::SqliteConnection);

#[launch]
fn rocket() -> Rocket<Build> {
    let build = rocket::build()
        .attach(ValueDbConnection::fairing())
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                index,
                favicon::favicon,
                home::home,
                about::about,
                temperature::set,
                humidity::set,
                led::get,
                led::set,
                get_data
            ],
        );

    info!("{}{}:", Paint::emoji("📶"), Paint::yellow(" MQTT"));
    info_!("{}: {}", "publisher", Paint::default("starting"));
    info_!("{}: {}", "subscriber", Paint::default("starting"));
    let config: Config = confy::load_path(PathBuf::from("config/rumqttd.conf")).unwrap();
    // Spawn mqtt server
    thread::spawn(|| {

        publisher::start(config);

    });


    build
}
