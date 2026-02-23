use crate::elements::common::{OgType, PathWrap};
use crate::elements::navbar::NavigationBar;
use crate::elements::post::Post;
use crate::DEFAULT_IMG_PATH;
use askama::Template;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Add;
use std::rc::Rc;

const POST_LISTING_PATH: &str = "posts.html";
const POSTS_DESCRIPTION: &str = "A listing of all of the posts on this site, sorted by date.";

const PROJECT_LISTING_PATH: &str = "posts/projects.html";
const PROJECTS_DESCRIPTION: &str = "A listing of some of the projects I've worked on - in no particular order. This list isn't comprehensive!";

pub const TAGS_DIR: &str = "tags/";

#[derive(Template)]
#[template(path = "post-listing.html", escape = "none")]
pub struct PostListingPage {
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub path: PathWrap,
    pub posts: Vec<Rc<Post>>,
    pub navbar: NavigationBar,
    pub show_inline_description: bool,
    pub og_type: OgType,
    pub og_image: PathWrap,
}

pub fn build_full_post_listing(blog_posts: &[Rc<Post>]) {
    let posts_page = PostListingPage {
        title: Cow::Borrowed("Posts"),
        description: Cow::Borrowed(POSTS_DESCRIPTION),
        path: PathWrap::from(POST_LISTING_PATH),
        navbar: NavigationBar::new(),
        show_inline_description: false,
        posts: blog_posts
            .iter()
            .filter(|x| !x.has_tag("_no-index"))
            .cloned()
            .collect(),
        og_type: OgType::Website,
        og_image: PathWrap::from(DEFAULT_IMG_PATH),
    };

    let mut posts_page_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(posts_page.path.to_local_file_path())
        .unwrap();
    posts_page_file
        .write_all(posts_page.render().unwrap().as_bytes())
        .unwrap();
    posts_page_file.flush().unwrap();
}

pub fn build_project_listing(projects: &[Rc<Post>]) {
    let projects_page = PostListingPage {
        title: Cow::Borrowed("Projects"),
        description: Cow::Borrowed(PROJECTS_DESCRIPTION),
        path: PathWrap::from(PROJECT_LISTING_PATH),
        navbar: NavigationBar::new(),
        show_inline_description: true,
        posts: projects.to_vec(),
        og_type: OgType::Website,
        og_image: PathWrap::from(DEFAULT_IMG_PATH),
    };

    let mut project_page_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(projects_page.path.to_local_file_path())
        .unwrap();
    project_page_file
        .write_all(projects_page.render().unwrap().as_bytes())
        .unwrap();
    project_page_file.flush().unwrap();
}

pub fn build_tag_listing_pages(blog_posts: &[Rc<Post>]) {
    let mut tag_map: HashMap<&str, Vec<Rc<Post>>> = HashMap::new();

    blog_posts
        .iter()
        .filter(|x| !x.has_tag("_no-index"))
        .for_each(|post| {
            post.tags.iter().for_each(|tag| {
                let new_entry = tag_map.entry(tag).or_default();
                new_entry.push(Rc::clone(post));
            });
        });

    for (tag, posts) in tag_map {
        let path = TAGS_DIR.to_string().add(tag).add(".html");

        let tag_page = PostListingPage {
            title: Cow::Owned(format!("Posts tagged {tag}")),
            description: Cow::Owned(format!(
                "Listing of all posts on this website that have been tagged \"{tag}\"."
            )),
            path: PathWrap::from(path),
            navbar: NavigationBar::new(),
            show_inline_description: false,
            posts,
            og_type: OgType::Website,
            og_image: PathWrap::from(DEFAULT_IMG_PATH),
        };

        if std::fs::exists(std::path::Path::new(TAGS_DIR)).is_ok() {
            std::fs::create_dir_all(TAGS_DIR).unwrap();
        }

        let mut tag_page_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(tag_page.path.to_local_file_path())
            .unwrap();
        tag_page_file
            .write_all(tag_page.render().unwrap().as_bytes())
            .unwrap();
        tag_page_file.flush().unwrap();

        tracing::info!("Created page {:?}", tag_page.path);
    }
}
