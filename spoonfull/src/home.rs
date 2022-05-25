use rocket_dyn_templates::{Template, context};

#[get("/home")]
pub fn home() -> Template {
    Template::render(
        "index",
        context! {
            ip_address: "None",
            led_state: "off".to_owned(),
            temperature: 0.0,
            humidity: 0.0,
        },
    )
}
