use crate::elements::navbar::NavigationBar;
use askama::Template;
use chrono::NaiveDate;
use std::borrow::Cow;
use std::rc::Rc;
pub struct Post {
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub body: Cow<'static, str>,
    pub date: Option<NaiveDate>,
    pub tags: Vec<Cow<'static, str>>,
}

impl Post {
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&Cow::Borrowed(tag))
    }
}

#[derive(Template)]
#[template(path = "post.html", escape = "none")]
pub struct PostPage {
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub path: Cow<'static, str>,
    pub post: Rc<Post>,
    pub navbar: NavigationBar,
}

