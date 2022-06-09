use crate::responder::HTMLResponder;
use rocket_dyn_templates::{context, Template};

#[get("/about")]
pub fn about() -> HTMLResponder {
    Template::render("about", context! { parent: "layout" }).into()
}
