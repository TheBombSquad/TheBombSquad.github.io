use crate::elements::common::*;
use crate::elements::home::build_home_page;
use crate::elements::post::{Post, PostPage};
use crate::elements::post_listing::{
    build_full_post_listing, build_project_listing, build_tag_listing_pages, TAGS_DIR,
};
use const_format::concatcp;
use std::rc::Rc;
use tracing_subscriber::FmtSubscriber;

mod elements;

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
            if path.extension().is_some() && path.extension().unwrap().eq_ignore_ascii_case("md") {
                let post_parse = Post::new(path);
                match post_parse {
                    Ok(post) => {
                        tracing::info!("Parsed markdown file {:?}", post.path);

                        let post_rc = Rc::new(post);

                        posts.push(post_rc);
                    }
                    Err(err) => {
                        tracing::warn!("Failed to parse markdown file {:?}", err);
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_path_wrap() {
        let a = PathWrap::from("images/dragon.png");
        let b = PathWrap::from("posts/post.md");
        let c = PathWrap::from(PathBuf::from("posts/post.md"));

        // Local path, for file output
        assert_eq!(a.to_local_file_path(), "docs/images/dragon.png");
        assert_eq!(b.to_local_file_path(), "docs/posts/post.md");
        assert_eq!(c.to_local_file_path(), "docs/posts/post.html");

        // Root-relative path, for linking
        assert_eq!(a.to_static_file_path(), "/images/dragon.png");
        assert_eq!(b.to_static_file_path(), "/posts/post.md");
        assert_eq!(c.to_static_file_path(), "/posts/post.html");

        // URL, for opengraph
        assert_eq!(a.to_url_string(), "https://bombsquad.dev/images/dragon.png");
        assert_eq!(b.to_url_string(), "https://bombsquad.dev/posts/post.md");
        assert_eq!(c.to_url_string(), "https://bombsquad.dev/posts/post.html");
    }
}
