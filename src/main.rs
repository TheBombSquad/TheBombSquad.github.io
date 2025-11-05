use std::fs::OpenOptions;
use std::io::Write;

use askama::Template;

use crate::elements::navbar::{NavBarLink, NavigationBar};
use crate::elements::post::Post;

mod elements;

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate<'a> {
    title: &'a str,
    description: &'a str,
    navbar: NavigationBar<'a>,
    post: Post<'a>,
}

fn create_navbar<'a>() -> NavigationBar<'a> {
    let navbar_elements = vec![
        NavBarLink {
            name: "Home",
            path: "",
        },
        NavBarLink {
            name: "Projects",
            path: "/about.html",
        },
        NavBarLink {
            name: "About",
            path: "/about.html",
        },
    ];

    NavigationBar {
        elements: navbar_elements,
    }
}

fn main() {
    let navbar = create_navbar();

    let post_content = markdown::to_html_with_options(&std::fs::read_to_string("posts/20251103.md").unwrap(),
    &markdown::Options::gfm()).unwrap();

    let base = BaseTemplate {
        title: "Home",
        description: "Home page description goes here!",
        navbar,
        post: Post { title: "Test", content: &post_content },
    };

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("out/index.html")
        .unwrap();
    file.write_all(base.render().unwrap().as_bytes()).unwrap();
    file.flush().unwrap();
}
