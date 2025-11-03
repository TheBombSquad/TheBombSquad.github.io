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
    let base = BaseTemplate {
        title: "Home",
        description: "Home page description goes here!",
        navbar,
        post: Post{title: "Test", content: "This is a test of a post's contents. Soon it will be parsed from a markdown file. For now, I'm just typing a rather long sentence to see how text wrapping gets handled. The quick brown fox jumps over the lazy dog."},
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
