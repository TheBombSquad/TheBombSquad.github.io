use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use askama::Template;
use chrono::NaiveDate;
use crate::elements::navbar::{NavBarLink, NavigationBar};
use crate::elements::post::Post;
use anyhow::{Context, Result};
use gray_matter::engine::YAML;
use gray_matter::{Matter, ParsedEntity};
use tracing_subscriber::FmtSubscriber;

mod elements;

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate<'a> {
    title: &'a str,
    description: &'a str,
    navbar: NavigationBar<'a>,
    post: Post<'a>,
}

fn create_navbar<'a>() -> NavigationBar<'a> {
    let navbar_elements = vec![
        NavBarLink {
            name: "Home",
            path: "",
        },
        NavBarLink {
            name: "Projects",
            path: "/about.html",
        },
        NavBarLink {
            name: "About",
            path: "/about.html",
        },
    ];

    NavigationBar {
        elements: navbar_elements,
    }
}

fn parse_markdown_post(path: &Path) -> Result<()> {
    let matter = Matter::<YAML>::new();

    let post_filename = path.with_extension("html");
    let post_path_name = format!("out/{}", post_filename.to_str().context("Failed to convert filename to string")?);

    let post_content_raw = &std::fs::read_to_string(path)?;
    let parsed_result: ParsedEntity = matter.parse(post_content_raw)?;

    let post_content = parsed_result.content;
    let post_matter = parsed_result.data.context("Failed to parse post matter")?;

    let post_title = post_matter["title"].as_string()?;
    let post_description = post_matter["description"].as_string()?;

    let post_creation_date_str = post_matter["date"].as_string()?;
    let post_creation_date = NaiveDate::parse_from_str(&post_creation_date_str, "%Y-%m-%d")?;

    let post_content_body = markdown::to_html_with_options(&post_content, &markdown::Options::gfm()).unwrap();

    let navbar = create_navbar();
    let base = BaseTemplate {
        title: &post_title,
        description: &post_description,
        navbar,
        post: Post { title: &post_title, content: &post_content_body, date: post_creation_date },
    };

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(post_path_name)?;
    file.write_all(base.render()?.as_bytes())?;
    file.flush()?;
    Ok(())
}

fn main() {
    // Logging
    let tracing_subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(tracing_subscriber)
    .expect("setting tracing default failed");

    // Markdown posts
    if let Ok(posts) = std::fs::read_dir("posts") {
        for entry in posts {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension() == Some("md".as_ref()) {
                if let Ok(()) = parse_markdown_post(&path) {
                    tracing::info!("Parsed markdown file {:?}", path);
                }
                else {
                    tracing::warn!("Failed to parse markdown file {:?}", path);
                }
            }
            else {
                tracing::warn!("Skipping non-markdown file {:?}", path);
            }
        }
    }
}
