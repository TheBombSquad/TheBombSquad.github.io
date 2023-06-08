mod elements;

use askama::Template;
use std::fs::{File, OpenOptions};
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
    let test_element = NavBarLink{name: "Title", path: "title.html"};
    let test_navbar = NavigationBar{elements: vec![test_element, test_element]};
    let test_template = HelloTemplate { title: "title!" , description: "description!", navbar: test_navbar };
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open("out.html").unwrap();
    file.write_all(test_template.render().unwrap().as_bytes()).unwrap();
    file.flush().unwrap();
}
