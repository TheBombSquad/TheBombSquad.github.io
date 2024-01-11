use std::fs::OpenOptions;
use std::io::Write;

use askama::Template;

use crate::elements::navbar::{NavBarLink, NavigationBar};

mod elements;

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate<'a> {
    title: &'a str,
    description: &'a str,
    navbar: NavigationBar<'a>,
}

fn create_navbar<'a>() -> NavigationBar<'a> {
    let navbar_elements = vec![
        NavBarLink { name: "Home", path: "" },
        NavBarLink { name: "About", path: "/about.html" },
    ];

    NavigationBar { elements: navbar_elements }
}

fn main() {
    let base = BaseTemplate { title: "title!", description: "description!", navbar: create_navbar() };
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open("index.html").unwrap();
    file.write_all(base.render().unwrap().as_bytes()).unwrap();
    file.flush().unwrap();
}
