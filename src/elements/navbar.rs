use askama::Template;

#[derive(Copy, Clone)]
pub struct NavBarLink<'a> {
    pub name: &'a str,
    pub path: &'a str,
}

#[derive(Template, Default)]
#[template(path = "navbar.html", escape="none")]
pub struct NavigationBar<'a> {
    pub elements: Vec<NavBarLink<'a>>,
}
