use std::borrow::Cow;
use askama::Template;
use chrono::NaiveDate;

#[derive(Template, Default)]
#[template(path = "post-body.html", escape = "none")]
pub struct Post {
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub content: Cow<'static, str>,
    pub date: NaiveDate,
    pub tags: Vec<String>,
}
