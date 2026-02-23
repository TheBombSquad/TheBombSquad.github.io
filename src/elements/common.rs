use chrono::NaiveDate;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

pub const OUT_DIR: &str = "docs/";
pub const SITE_URL: &str = "https://bombsquad.dev";

pub const DEFAULT_IMG_PATH: &str = "images/dragon.png";

/// Enum for opengraph types - article, etc
pub enum OgType {
    Article(Option<NaiveDate>, Vec<String>),
    Website,
}

impl Display for OgType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OgType::Article(_, _) => write!(f, "article"),
            OgType::Website => write!(f, "website"),
        }
    }
}

// Type for dealing with various path demands: file paths, relative to root, and URL.
// Just as a note: if we pass a `.md` file into this, it will get converted into a path to a `.html.`
// ONLY if the file is passed in as a PathBuf, however.
#[derive(Clone, Debug)]
pub struct PathWrap {
    path: PathBuf,
}

impl PathWrap {
    // For URL paths.
    pub fn to_url_string(&self) -> String {
        let path = self.path.to_string_lossy();
        format!(
            "{}/{}",
            SITE_URL,
            path.strip_prefix(OUT_DIR).unwrap_or(&path)
        )
    }

    // For paths that need to be relative to /.
    pub fn to_static_file_path(&self) -> String {
        let root_path = "/";
        root_path.to_owned()
            + &self
                .path
                .strip_prefix(OUT_DIR)
                .unwrap_or(&self.path)
                .to_string_lossy()
                .to_ascii_lowercase()
    }

    // For paths that need to be relative to /doc/.
    pub fn to_local_file_path(&self) -> String {
        self.path.to_string_lossy().to_ascii_lowercase()
    }

    // For when we need specific path operations.
    pub fn to_path(&self) -> &Path {
        &self.path
    }
}

impl From<PathBuf> for PathWrap {
    fn from(mut path: PathBuf) -> Self {
        // Fix up MD->HTML translation

        path.as_mut_os_str().make_ascii_lowercase();
        let html_path = match path.extension() {
            Some(ext) => {
                if ext == "md" {
                    path.with_extension("html")
                } else {
                    path
                }
            }
            None => path,
        };

        Self {
            path: PathBuf::from(OUT_DIR).join(html_path),
        }
    }
}

impl From<&str> for PathWrap {
    fn from(path: &str) -> Self {
        Self {
            path: PathBuf::from(OUT_DIR).join(path),
        }
    }
}

impl From<String> for PathWrap {
    fn from(path: String) -> Self {
        Self {
            path: PathBuf::from(OUT_DIR).join(path),
        }
    }
}

// Display implementation here
impl Display for PathWrap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.to_string_lossy())
    }
}
