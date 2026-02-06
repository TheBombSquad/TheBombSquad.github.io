use crate::elements::navbar::NavigationBar;
use anyhow::{Context, Result};
use askama::Template;
use chrono::NaiveDate;
use gray_matter::engine::YAML;
use gray_matter::{Matter, ParsedEntity};
use std::borrow::Cow;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub struct Post {
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub body: Cow<'static, str>,
    pub path: PathBuf,
    pub date: Option<NaiveDate>,
    pub tags: Vec<Cow<'static, str>>,
    pub preview: Cow<'static, str>,
}

const PREVIEW_CHAR_LIMIT: usize = 300;

impl Post {
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&Cow::Borrowed(tag))
    }

    // Strictly for template use - askama does not like it when we pass in a PathBuf/Path.
    pub fn get_path_str(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    pub fn new(path: &Path) -> Result<Self> {
        // Parse front matter
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

        // Preview is just before the first line break
        let first_line_break = post_content.find('\n');

        let truncated = match (first_line_break) {
            Some(idx) => post_content[..idx].to_string(),
            None => post_content.clone()
        };

        let post_content_preview = markdown::to_html_with_options(&truncated, &markdown::Options::gfm()).unwrap();

        // Parse the actual post content
        let post_content_body =
            markdown::to_html_with_options(&post_content, &markdown::Options::gfm()).unwrap();

        let post_path = path.with_extension("html");

        let post = Post {
            title: Cow::Owned(post_title),
            description: Cow::Owned(post_description),
            path: post_path,
            body: Cow::Owned(post_content_body),
            preview: Cow::Owned(post_content_preview),
            date: post_creation_date,
            tags: post_tags,
        };

        Ok(post)
    }
}

#[derive(Template)]
#[template(path = "post.html", escape = "none")]
pub struct PostPage {
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub path: PathBuf,
    pub post: Rc<Post>,
    pub navbar: NavigationBar,
}

impl PostPage {
    pub fn new(post: &Rc<Post>) -> Result<PostPage> {
        let base = PostPage {
            title: post.title.clone(),
            description: post.description.clone(),
            path: PathBuf::from("docs").join(&post.path), // TODO: use const in main
            navbar: NavigationBar::new(),
            post: Rc::clone(post),
        };

        if let Some(parent) = base.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&base.path)?;
        file.write_all(base.render()?.as_bytes())?;
        file.flush()?;

        Ok(base)
    }
}

#[derive(Template)]
#[template(path = "post-listing.html", escape = "none")]
pub struct PostListingPage {
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub posts: Vec<Rc<Post>>,
    pub navbar: NavigationBar,
}
