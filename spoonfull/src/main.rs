#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use rocket::yansi::Paint;
use rocket::log::PaintExt;

use librumqttd::{Broker, Config};
use std::path::PathBuf;
use std::thread;

use crate::endpoints::*;

mod about;
mod endpoints;
mod favicon;
mod home;
mod responder;

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

    // Spawn mqtt server
    thread::spawn(|| {
        info!("{}{}:", Paint::emoji("📶"), Paint::yellow(" MQTT"));
        info_!("{}: {}", "start", Paint::default("created thread worker"));

        let config: Config = confy::load_path(PathBuf::from("config/rumqttd.conf")).unwrap();
        info_!("{}: {}", "config", Paint::default("loading configuration"));
        let mut broker = Broker::new(config);
        let mut tx = broker.link("localclient").unwrap();
        let output = broker.start();
        info!("{}{}:", Paint::emoji("🛑"), Paint::yellow(" MQTT"));
        info_!("{}: {}", "broker stopped", Paint::default(format!("{:?}", output)));

        let mut rx = tx.connect(200).unwrap();
        tx.subscribe("#").unwrap();

        // subscribe and publish in a separate thread
        thread::spawn(move || {
            for _ in 0..10 {
            for i in 0..200 {
                let topic = format!("hello/{}/world", i);
                tx.publish(topic, false, vec![0; 1024]).unwrap();
            }
            }
        });

        let mut count = 0;
        loop {
            if let Some(message) = rx.recv().unwrap() {
            // println!("T = {}, P = {:?}", message.topic, message.payload.len());
            count += message.payload.len();
            println!("{}", count);
            }
        }
        });

        build
}
