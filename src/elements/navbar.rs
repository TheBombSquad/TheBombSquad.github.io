use std::borrow::Cow;
use askama::Template;
pub struct NavBarLink {
    pub name: Cow::<'static, str>,
    pub path: Cow::<'static, str>,
}

#[derive(Template, Default)]
#[template(path = "navbar.html", escape = "none")]
pub struct NavigationBar {
    pub elements: Vec<NavBarLink>,
}
