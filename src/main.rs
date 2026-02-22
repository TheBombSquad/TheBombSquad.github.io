use crate::elements::home::HomePage;
use crate::elements::navbar::NavigationBar;
use crate::elements::post::{Post, PostListingPage, PostPage};
use askama::Template;
use const_format::concatcp;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;
use tracing_subscriber::FmtSubscriber;
use crate::elements::common::OgType;

mod elements;

const OUT_DIR: &str = "docs/";
const HOMEPAGE_PATH: &str = concatcp!(OUT_DIR, "index.html");
const POST_LISTING_PATH: &str = concatcp!(OUT_DIR, "posts.html");
const PROJECT_LISTING_PATH: &str = concatcp!(OUT_DIR, "posts/projects.html");
const TAGS_DIR: &str = concatcp!(OUT_DIR, "tags");


const POSTS_DESCRIPTION: &str = "A listing of all of the posts on this site, sorted by date.";
const PROJECTS_DESCRIPTION: &str = "A listing of some of the projects I've worked on - in no particular order. This list isn't comprehensive!";

const SITE_URL: &str = "https://bombsquad.dev";

pub fn convert_path_to_url(path: &str) -> String {
    format!("{}/{}", SITE_URL, path.strip_prefix(OUT_DIR).unwrap_or(path))
}

fn clean_output_dir(path: &str) {
    tracing::info!("Cleaning up: {}", path);
    if let Ok(dirs) = std::fs::read_dir(path) {
        for entry in dirs.flatten() {
            let path = entry.path();
            if path.is_file() {
                tracing::info!("Removed page {:?}", path);
                std::fs::remove_file(&path).unwrap();
            }
        }
    }
}

fn collect_markdown_posts(path_prefix: &str) -> Vec<Rc<Post>> {
    let mut posts: Vec<Rc<Post>> = Vec::new();

    if let Ok(post_files) = std::fs::read_dir(path_prefix) {
        for entry in post_files {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().is_some() && path.extension().unwrap().eq_ignore_ascii_case("md")  {
                let post_parse = Post::new(&path);
                match post_parse {
                    Ok(post) => {
                        tracing::info!("Parsed markdown file {:?}", path);

                        let post_rc = Rc::new(post);

                        posts.push(post_rc);
                    }
                    Err(err) => {
                        tracing::warn!("Failed to parse markdown file {:?}: {:?}", path, err);
                    }
                }
            } else if path.is_file() {
                tracing::warn!("Skipping non-markdown file {:?}", path);
            }
        }
    }

    // Sort posts from newest to oldest
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    posts
}

fn build_home_page(blog_posts: &[Rc<Post>]) {
    let recent_blog_posts = blog_posts
        .iter()
        .filter(|x| !x.has_tag("_no-index"))
        .take(5)
        .cloned()
        .collect::<Vec<Rc<Post>>>();
    let home_page = HomePage {
        title: Cow::Borrowed("Home"),
        description: Cow::Borrowed("bombsquad.dev"),
        path: PathBuf::from(OUT_DIR),
        navbar: NavigationBar::new(),
        recent_posts: recent_blog_posts,
        show_inline_description: false,
        og_type: OgType::Website,
        og_url: SITE_URL.to_string()
    };

    let mut home_page_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(HOMEPAGE_PATH)
        .unwrap();
    home_page_file
        .write_all(home_page.render().unwrap().as_bytes())
        .unwrap();
    home_page_file.flush().unwrap();
}

fn build_full_post_listing(blog_posts: &[Rc<Post>]) {
    let path = PathBuf::from(OUT_DIR).join("posts.html");
    let og_path = convert_path_to_url(&path.to_string_lossy());

    let posts_page = PostListingPage {
        title: Cow::Borrowed("Posts"),
        description: Cow::Borrowed(POSTS_DESCRIPTION),
        path,
        navbar: NavigationBar::new(),
        show_inline_description: false,
        posts: blog_posts
            .iter()
            .filter(|x| !x.has_tag("_no-index"))
            .cloned()
            .collect(),
        og_type: OgType::Website,
        og_url: og_path
    };

    let mut posts_page_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(POST_LISTING_PATH)
        .unwrap();
    posts_page_file
        .write_all(posts_page.render().unwrap().as_bytes())
        .unwrap();
    posts_page_file.flush().unwrap();
}

fn build_project_listing(projects: &[Rc<Post>]) {
    let path = PathBuf::from(OUT_DIR).join("posts/projects.html");
    let og_path = convert_path_to_url(&path.to_string_lossy());

    let projects_page = PostListingPage {
        title: Cow::Borrowed("Projects"),
        description: Cow::Borrowed(PROJECTS_DESCRIPTION),
        path,
        navbar: NavigationBar::new(),
        show_inline_description: true,
        posts: projects.to_vec(),
        og_type: OgType::Website,
        og_url: og_path,
    };

    let mut project_page_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(PROJECT_LISTING_PATH)
        .unwrap();
    project_page_file
        .write_all(projects_page.render().unwrap().as_bytes())
        .unwrap();
    project_page_file.flush().unwrap();
}

fn build_tag_listing_pages(blog_posts: &[Rc<Post>]) {
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
        let path = PathBuf::from(TAGS_DIR).join(tag).with_extension("html");
        let og_path = convert_path_to_url(&path.to_string_lossy());

        let tag_page = PostListingPage {
            title: Cow::Owned(format!("Posts tagged {tag}")),
            description: Cow::Owned(format!("Listing of all posts on this website that have been tagged \"{tag}\".")),
            path: path.clone(),
            navbar: NavigationBar::new(),
            show_inline_description: false,
            posts,
            og_type: OgType::Website,
            og_url: og_path
        };


        if let Ok(_) = std::fs::exists(&std::path::Path::new(TAGS_DIR)) {
            std::fs::create_dir_all(TAGS_DIR).unwrap();
        }

        let mut tag_page_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .unwrap();
        tag_page_file
            .write_all(tag_page.render().unwrap().as_bytes())
            .unwrap();
        tag_page_file.flush().unwrap();

        tracing::info!("Created page {:?}", path);
    }
}

fn main() {
    // Logging
    let tracing_subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(tracing_subscriber)
        .expect("setting tracing default failed");

    // Clean up old/removed posts
    clean_output_dir(concatcp!(OUT_DIR, "posts"));
    clean_output_dir(concatcp!(OUT_DIR, "posts/projects"));
    clean_output_dir(TAGS_DIR);

    // Blog posts
    let blog_posts = collect_markdown_posts("posts");

    // Home page
    build_home_page(&blog_posts);

    // Posts listing page
    build_full_post_listing(&blog_posts);

    // Projects
    let projects = collect_markdown_posts("posts/projects");
    build_project_listing(&projects);

    // Tag->post listing page
    build_tag_listing_pages(&blog_posts);

    // Render the rest of the pages
    for post in blog_posts.iter().chain(projects.iter()) {
        let page_creation = PostPage::new(post);
        match page_creation {
            Ok(page) => {
                tracing::info!("Created page {:?}", page.path);
            }
            Err(err) => {
                tracing::error!("Failed to create page for post {:?}: {:?}", post.title, err);
            }
        }
    }
}
