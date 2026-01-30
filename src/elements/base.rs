use crate::elements::navbar::NavigationBar;
use askama::Template;
use std::borrow::Cow;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate {
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub path: Cow<'static, str>,
    pub navbar: NavigationBar,
}
