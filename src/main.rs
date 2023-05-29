use askama::Template;
use std::fs::File;
use std::io::Write;

#[derive(Template)]
#[template(path = "base.html")]
struct HelloTemplate<'a> {
    title: &'a str,
    description: &'a str,
}

fn main() {
    let test_template = HelloTemplate { title: "title!" , description: "description!" };
    let mut file = File::create("out.html").unwrap();
    file.write_all(test_template.render().unwrap().as_bytes()).unwrap();
}
