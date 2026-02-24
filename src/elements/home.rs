use crate::elements::common::*;
use crate::elements::navbar::NavigationBar;
use crate::elements::post::Post;
use askama::Template;
use std::borrow::Cow;
use std::fs::OpenOptions;
use std::io::Write;
use std::rc::Rc;

const HOMEPAGE_PATH: &str = "index.html";
const HOMEPAGE_DESCRIPTION: &str = "Home page, blog, and portfolio of 'The BombSquad' - software engineer and electronics enthusiast.";
#[derive(Template)]
#[template(path = "home.html", escape = "none")]
pub struct HomePage {
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub path: PathWrap,
    pub recent_posts: Vec<Rc<Post>>,
    pub navbar: NavigationBar,
    pub show_inline_description: bool,
    pub og_type: OgType,
    pub og_image: PathWrap,
}

pub fn build_home_page(blog_posts: &[Rc<Post>]) {
    let recent_blog_posts = blog_posts
        .iter()
        .filter(|x| !x.has_tag("_no-index"))
        .take(5)
        .cloned()
        .collect::<Vec<Rc<Post>>>();
    let home_page = HomePage {
        title: Cow::Borrowed("Home"),
        description: Cow::Borrowed(HOMEPAGE_DESCRIPTION),
        path: PathWrap::from(HOMEPAGE_PATH),
        navbar: NavigationBar::new(),
        recent_posts: recent_blog_posts,
        show_inline_description: false,
        og_type: OgType::Website,
        og_image: PathWrap::from(DEFAULT_IMG_PATH),
    };

    let mut home_page_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(home_page.path.to_local_file_path())
        .unwrap();
    home_page_file
        .write_all(home_page.render().unwrap().as_bytes())
        .unwrap();
    home_page_file.flush().unwrap();
}
