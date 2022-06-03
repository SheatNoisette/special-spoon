use std::thread;
use rocket::log::PaintExt;
use rocket::yansi::Paint;
use librumqttd::{Broker, Config};

pub fn start(config: Config) {
    let mut broker = Broker::new(config);
    let mut tx = broker.link("localclient").unwrap();
    thread::spawn(move || {
        let output = broker.start();
        info!("{}{}:", Paint::emoji("🛑"), Paint::yellow(" MQTT"));
        info_!(
            "{}: {}",
            "broker stopped",
            Paint::default(format!("{:?}", output))
        );
    });

    let mut rx = tx.connect(200).unwrap();
    tx.subscribe("#").unwrap();

    // subscribe and publish in a separate thread
    thread::spawn(move || {
        for _ in 0..10 {
            for i in 0..200 {
                let topic = format!("hello/{}/world", i);
                tx.publish(topic, false, vec![0; 1024]).unwrap();
                info!("Published something");
            }
        }
    });

    let mut count = 0;
    loop {
        if let Some(message) = rx.recv().unwrap() {
            info!("T = {}, P = {:?}", message.topic, message.payload.len());
            count += message.payload.len();
            println!("{}", count);
            info!("Received? something");
        }
    }
}
