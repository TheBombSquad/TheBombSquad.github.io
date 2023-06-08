mod elements;

use askama::Template;
use std::fs::File;
use std::io::Write;

use crate::elements::navbar::{NavBarLink, NavigationBar};

#[derive(Template)]
#[template(path = "base.html")]
struct HelloTemplate<'a> {
    title: &'a str,
    description: &'a str,
    navbar: NavigationBar<'a>,
}

fn main() {
    let test_template = HelloTemplate { title: "title!" , description: "description!", navbar: NavigationBar::default() };
    let test_element = NavBarLink{name: "Title", path: "title.html"};
    let test_navbar = NavigationBar{elements: vec![test_element, test_element]};
    let mut file = File::create("out.html").unwrap();
    file.write_all(test_template.render().unwrap().as_bytes()).unwrap();
}
