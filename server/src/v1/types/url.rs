/// A URL to a resource, generally an image hosted in S3. URLs should be
/// 400 characters or less.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Url(String);

impl Url {
    /// Constructs a new URL
    pub fn new(url: impl ToString) -> Self {
        Self(url.to_string())
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Url> for String {
    fn from(u: Url) -> Self {
        u.0
    }
}

impl From<String> for Url {
    fn from(s: String) -> Self {
        Url(s)
    }
}

impl From<&str> for Url {
    fn from(s: &str) -> Self {
        Url(s.to_string())
    }
}
