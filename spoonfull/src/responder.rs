use rocket::http::ContentType;
use rocket_dyn_templates::Template;

#[derive(Responder)]
#[response(status = 500, content_type = "html")]
pub struct HTMLResponder {
    inner: Template,
    header: ContentType,
}

impl From<Template> for HTMLResponder {
    fn from(inner: Template) -> Self {
        HTMLResponder {
            inner,
            header: ContentType::HTML,
        }
    }
}
