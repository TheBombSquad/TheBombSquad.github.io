use crate::elements::navbar::NavigationBar;
use crate::elements::post::Post;
use askama::Template;
use std::borrow::Cow;
use std::rc::Rc;

#[derive(Template)]
#[template(path = "home.html", escape = "none")]
pub struct HomePage {
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub recent_posts: Vec<Rc<Post>>,
    pub navbar: NavigationBar,
}
