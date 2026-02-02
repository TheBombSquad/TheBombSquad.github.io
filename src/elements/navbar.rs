use askama::Template;
use std::borrow::Cow;

pub struct NavBarLink {
    pub name: Cow<'static, str>,
    pub path: Cow<'static, str>,
}

#[derive(Template, Default)]
#[template(path = "navbar.html", escape = "none")]
pub struct NavigationBar {
    pub elements: Vec<NavBarLink>,
}

impl NavigationBar {
    pub fn new() -> Self {
        let navbar_elements = vec![
            NavBarLink {
                name: Cow::Borrowed("Home"),
                path: Cow::Borrowed("/"),
            },
            NavBarLink {
                name: Cow::Borrowed("Posts"),
                path: Cow::Borrowed("/posts.html"),
            },
            NavBarLink {
                name: Cow::Borrowed("Projects"),
                path: Cow::Borrowed("/posts/projects.html"),
            },
            NavBarLink {
                name: Cow::Borrowed("About"),
                path: Cow::Borrowed("/posts/about.html"),
            },
        ];

        NavigationBar {
            elements: navbar_elements,
        }
    }
}
