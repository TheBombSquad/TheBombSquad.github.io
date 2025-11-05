use askama::Template;
use chrono::NaiveDate;

#[derive(Template, Default)]
#[template(path = "post-body.html", escape = "none")]
pub struct Post<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub date: NaiveDate,
}

