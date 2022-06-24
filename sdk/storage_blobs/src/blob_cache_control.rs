use azure_core::headers::{self, Header};

#[derive(Debug, Clone, Copy)]
pub struct BlobCacheControl<'a>(&'a str);

impl<'a, S> From<S> for BlobCacheControl<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl<'a> Header for BlobCacheControl<'a> {
    fn name(&self) -> headers::HeaderName {
        azure_core::headers::BLOB_CACHE_CONTROL.into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
