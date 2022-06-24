use azure_core::headers::{self, Header};

#[derive(Debug, Clone, Copy)]
pub struct BlobContentEncoding<'a>(&'a str);

impl<'a, S> From<S> for BlobContentEncoding<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl<'a> Header for BlobContentEncoding<'a> {
    fn name(&self) -> headers::HeaderName {
        "x-ms-blob-content-encoding".into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
