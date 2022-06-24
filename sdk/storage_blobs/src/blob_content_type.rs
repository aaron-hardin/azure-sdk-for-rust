use azure_core::headers::{self, Header};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobContentType<'a>(&'a str);

impl<'a> BlobContentType<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}

impl<'a, S> From<S> for BlobContentType<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl<'a> Header for BlobContentType<'a> {
    fn name(&self) -> headers::HeaderName {
        "x-ms-blob-content-type".into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
