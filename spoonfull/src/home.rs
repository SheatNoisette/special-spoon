use crate::responder::HTMLResponder;
use rocket_dyn_templates::{context, Template};

#[get("/home")]
pub fn home() -> HTMLResponder {
    Template::render(
        "index",
        context! {
            ip_address: "None",
            led_state: "off".to_owned(),
            temperature: 0.0_f32,
            humidity: 0.0_f32,
        },
    )
    .into()
}
