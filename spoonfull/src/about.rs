use rocket_dyn_templates::{Template, context};

#[get("/about")]
pub fn about() -> Template {
    Template::render("about", context! { parent: "layout" })
}
