use crate::elements::home::HomePage;
use crate::elements::navbar::NavigationBar;
use crate::elements::post::{Post, PostListingPage, PostPage};
use askama::Template;
use std::borrow::Cow;
use std::fs::OpenOptions;
use std::io::Write;
use std::rc::Rc;
use const_format::concatcp;
use tracing_subscriber::FmtSubscriber;

mod elements;

const OUT_DIR: &str = "docs/";
const HOMEPAGE_PATH: &str = concatcp!(OUT_DIR, "index.html");
const POST_LISTING_PATH: &str = concatcp!(OUT_DIR, "posts.html");
const PROJECT_LISTING_PATH: &str = concatcp!(OUT_DIR, "posts/projects.html");

fn collect_markdown_posts(path_prefix: &str) -> Vec<Rc<Post>> {
    let mut posts: Vec<Rc<Post>> = Vec::new();

    if let Ok(post_files) = std::fs::read_dir(path_prefix) {
        for entry in post_files {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension() == Some("md".as_ref()) {
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
            } else {
                tracing::warn!("Skipping non-markdown file {:?}", path);
            }
        }
    }

    // Sort posts from newest to oldest
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    posts
}
fn main() {
    // Logging
    let tracing_subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(tracing_subscriber)
        .expect("setting tracing default failed");

    // Blog posts
    let blog_posts = collect_markdown_posts("posts");

    let recent_blog_posts = blog_posts
        .iter()
        .filter(|x| !x.has_tag("_no-index"))
        .take(5)
        .cloned()
        .collect::<Vec<Rc<Post>>>();

    // Home page
    let home_page = HomePage {
        title: Cow::Borrowed("Home"),
        description: Cow::Borrowed("bombsquad.dev"),
        navbar: NavigationBar::new(),
        recent_posts: recent_blog_posts,
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

    // Posts listing page
    let posts_page = PostListingPage {
        title: Cow::Borrowed("Posts"),
        description: Cow::Borrowed("All posts"),
        navbar: NavigationBar::new(),
        posts: blog_posts
            .iter()
            .filter(|x| !x.has_tag("_no-index"))
            .cloned()
            .collect(),
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


    // Projects
    let projects = collect_markdown_posts("posts/projects");

    let projects_page = PostListingPage {
        title: Cow::Borrowed("Projects"),
        description: Cow::Borrowed("All projects"),
        navbar: NavigationBar::new(),
        posts: projects.clone()
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

    // Actually create the pages
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
