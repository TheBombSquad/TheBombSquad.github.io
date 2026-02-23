use crate::elements::common::{OgType, PathWrap};
use crate::elements::navbar::NavigationBar;
use crate::DEFAULT_IMG_PATH;
use anyhow::{Context, Result};
use askama::Template;
use chrono::NaiveDate;
use gray_matter::engine::YAML;
use gray_matter::{Matter, ParsedEntity};
use markdown::{CompileOptions, ParseOptions};
use std::borrow::Cow;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

pub struct ReadStats {
    pub num_words: usize,
    pub estimated_reading_time: f64,
}
pub struct Post {
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub body: Cow<'static, str>,
    pub path: PathWrap,
    pub date: Option<NaiveDate>,
    pub tags: Vec<String>,
    pub preview: Cow<'static, str>,
    pub header_image_path: PathWrap,
    pub read_stats: ReadStats,
}

impl Post {
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    pub fn has_visible_tags(&self) -> bool {
        self.tags.iter().any(|t| !t.starts_with('_'))
    }

    pub fn get_tag_page_path(&self, tag: &str) -> String {
        format!("/tags/{tag}.html")
    }

    // Thanks CraftedCart for letting me borrow this
    fn get_reading_stats(text_parts: &str) -> ReadStats {
        const WORDS_PER_MIN: f64 = 200.0;

        let num_words = text_parts
            .split_whitespace()
            .filter(|part| !part.trim().is_empty())
            .count();
        let mins = num_words as f64 / WORDS_PER_MIN;
        let estimated_reading_time = mins.ceil();

        ReadStats {
            num_words,
            estimated_reading_time,
        }
    }

    pub fn new(path: PathBuf) -> Result<Self> {
        // Parse front matter
        let matter = Matter::<YAML>::new();

        let post_content_raw = &std::fs::read_to_string(&path)?;
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
                post_tags.push(tag.as_string()?);
            }
        }

        let post_header_image = if post_matter.contains_key("header_image") {
            PathWrap::from(post_matter["header_image"].as_string()?)
        } else {
            PathWrap::from(DEFAULT_IMG_PATH)
        };

        // Preview is just before the first paragraph break
        let first_line_break = post_content.find("\n\n");

        let truncated = match first_line_break {
            Some(idx) => post_content[..idx].to_string(),
            None => post_content.clone(),
        };

        let post_content_preview =
            markdown::to_html_with_options(&truncated, &markdown::Options::gfm()).unwrap();

        // Parse the actual post content
        let post_content_body = markdown::to_html_with_options(
            &post_content,
            &markdown::Options {
                parse: ParseOptions::gfm(),
                compile: CompileOptions {
                    gfm_footnote_label_tag_name: Some("h3".to_string()),
                    ..CompileOptions::gfm()
                },
            },
        )
        .unwrap()
        .replace(
            "<table>",
            "<table class=\"table table-sm table-striped table-bordered\">",
        ); // Hack to make tables look nice

        // Resulting post file name should be lowercase for consistency
        let post_path = PathWrap::from(path);

        let post_reading_stats = Post::get_reading_stats(&post_content_body);

        let post = Post {
            title: Cow::Owned(post_title),
            description: Cow::Owned(post_description),
            path: post_path,
            body: Cow::Owned(post_content_body),
            preview: Cow::Owned(post_content_preview),
            date: post_creation_date,
            tags: post_tags,
            header_image_path: post_header_image,
            read_stats: post_reading_stats,
        };

        Ok(post)
    }
}

#[derive(Template)]
#[template(path = "post.html", escape = "none")]
pub struct PostPage {
    pub title: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub path: PathWrap,
    pub post: Rc<Post>,
    pub navbar: NavigationBar,
    pub show_inline_description: bool,
    pub og_type: OgType,
    pub og_image: PathWrap,
}

impl PostPage {
    pub fn new(post: &Rc<Post>) -> Result<PostPage> {
        let base = PostPage {
            title: post.title.clone(),
            description: post.description.clone(),
            path: post.path.clone(),
            navbar: NavigationBar::new(),
            post: Rc::clone(post),
            show_inline_description: false,
            og_type: OgType::Article(post.date, post.tags.clone()),
            og_image: post.header_image_path.clone(),
        };

        if let Some(parent) = base.path.to_path().parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(base.path.to_local_file_path())?;
        file.write_all(base.render()?.as_bytes())?;
        file.flush()?;

        Ok(base)
    }
}
