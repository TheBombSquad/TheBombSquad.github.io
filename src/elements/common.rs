use chrono::NaiveDate;

/// Enum for opengraph types - article, etc
pub enum OgType {
    Article(Option<NaiveDate>, Vec<String>),
    Website,
}

impl std::fmt::Display for OgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OgType::Article(_, _) => write!(f, "article"),
            OgType::Website => write!(f, "website"),
        }
    }
}