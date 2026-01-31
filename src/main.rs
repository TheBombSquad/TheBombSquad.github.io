use crate::elements::home::HomePage;
use crate::elements::navbar::{NavBarLink, NavigationBar};
use crate::elements::post::{Post, PostPage};
use anyhow::{Context, Result};
use askama::Template;
use chrono::NaiveDate;
use gray_matter::engine::YAML;
use gray_matter::{Matter, ParsedEntity};
use std::borrow::Cow;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use tracing_subscriber::FmtSubscriber;

mod elements;

fn create_navbar() -> NavigationBar {
    let navbar_elements = vec![
        NavBarLink {
            name: Cow::Borrowed("Home"),
            path: Cow::Borrowed("/"),
        },
        NavBarLink {
            name: Cow::Borrowed("Posts"),
            path: Cow::Borrowed("/posts/posts.html"),
        },
        NavBarLink {
            name: Cow::Borrowed("Projects"),
            path: Cow::Borrowed("/posts/projects.html"),
        },
        NavBarLink {
            name: Cow::Borrowed("About"),
            path: Cow::Borrowed("/posts/about.html"),
        },
    ];

    NavigationBar {
        elements: navbar_elements,
    }
}

fn parse_markdown_post(path: &Path) -> Result<Post> {
    let matter = Matter::<YAML>::new();

    let post_content_raw = &std::fs::read_to_string(path)?;
    let parsed_result: ParsedEntity = matter.parse(post_content_raw)?;

    let post_content = parsed_result.content;
    let post_matter = parsed_result
        .data
        .context("Failed to parse post matter")?
        .as_hashmap()?;

    let post_title = post_matter["title"].as_string()?;
    let post_description = post_matter["description"].as_string()?;

    let post_creation_date = if post_matter.contains_key("date") {
        let post_creation_date = post_matter["date"].as_string()?;
        Some(NaiveDate::parse_from_str(&post_creation_date, "%Y-%m-%d")?)
    } else {
        None
    };

    let mut post_tags = Vec::new();
    if let Ok(post_tags_raw) = post_matter["tags"].as_vec() {
        for tag in post_tags_raw {
            post_tags.push(Cow::Owned(tag.as_string()?));
        }
    }

    let post_content_body =
        markdown::to_html_with_options(&post_content, &markdown::Options::gfm()).unwrap();

    let post_path = match post_creation_date {
        Some(date) => format!("posts/{}.html", date.format("%Y-%m-%d")),
        None => format!("posts/{}.html", post_title.to_ascii_lowercase()),
    };

    let post = Post {
        title: Cow::Owned(post_title),
        description: Cow::Owned(post_description),
        path: Cow::Owned(post_path),
        body: Cow::Owned(post_content_body),
        date: post_creation_date,
        tags: post_tags,
    };

    Ok(post)
}

fn new_page_from_post(post: &Rc<Post>) -> Result<PostPage> {
    let base = PostPage {
        title: post.title.clone(),
        description: post.description.clone(),
        path: Cow::Owned(format!("out/{}", post.path)),
        navbar: create_navbar(),
        post: Rc::clone(post),
    };

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&*base.path)?;
    file.write_all(base.render()?.as_bytes())?;
    file.flush()?;

    Ok(base)
}

fn main() {
    // Logging
    let tracing_subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(tracing_subscriber)
        .expect("setting tracing default failed");

    // Collate Markdown posts
    let mut posts: Vec<Rc<Post>> = Vec::new();

    if let Ok(post_files) = std::fs::read_dir("posts") {
        for entry in post_files {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension() == Some("md".as_ref()) {
                let post_parse = parse_markdown_post(&path);
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

    let recent_posts = posts
        .iter()
        .filter(|x| !x.has_tag("_no-index"))
        .take(5)
        .cloned()
        .collect::<Vec<Rc<Post>>>();

    // Home page
    let home_page = HomePage {
        title: Cow::Borrowed("Home"),
        description: Cow::Borrowed("bombsqud.dev"),
        navbar: create_navbar(),
        recent_posts,
    };

    let mut home_page_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("out/index.html")
        .unwrap();
    home_page_file
        .write_all(home_page.render().unwrap().as_bytes())
        .unwrap();
    home_page_file.flush().unwrap();

    // Actually create the pages
    for post in posts {
        let page_creation = new_page_from_post(&post);
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
