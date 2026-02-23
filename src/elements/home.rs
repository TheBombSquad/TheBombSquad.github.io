use crate::elements::navbar::NavigationBar;
use crate::elements::post::Post;
use askama::Template;
use std::borrow::Cow;
use std::path::PathBuf;
use std::rc::Rc;
use crate::elements::common::*;

#[derive(Template)]
#[template(path = "home.html", escape = "none")]
pub struct HomePage {
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub path: PathWrap,
    pub recent_posts: Vec<Rc<Post>>,
    pub navbar: NavigationBar,
    pub show_inline_description: bool,
    pub og_type: OgType,
    pub og_image: PathWrap,
}
