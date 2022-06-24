use crate::{
    blob::{responses::SetBlobPropertiesResponse, BlobProperties},
    prelude::*,
};
use azure_core::prelude::*;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct SetBlobPropertiesBuilder<'a> {
    blob_client: &'a BlobClient,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    cache_control: Option<BlobCacheControl<'a>>,
    content_type: Option<BlobContentType<'a>>,
    content_encoding: Option<BlobContentEncoding<'a>>,
    content_language: Option<BlobContentLanguage<'a>>,
    content_disposition: Option<BlobContentDisposition<'a>>,
    content_md5: Option<BlobContentMD5>,
}

impl<'a> SetBlobPropertiesBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient) -> Self {
        Self {
            blob_client,
            lease_id: None,
            client_request_id: None,
            timeout: None,
            cache_control: None,
            content_type: None,
            content_encoding: None,
            content_language: None,
            content_disposition: None,
            content_md5: None,
        }
    }

    pub fn set_from_blob_properties(self, blob_properties: &'a BlobProperties) -> Self {
        let mut s = self;

        if let Some(cc) = &blob_properties.cache_control {
            s = s.cache_control(&cc[..]);
        }
        if !blob_properties.content_type.is_empty() {
            s = s.content_type(&blob_properties.content_type[..]);
        }
        if let Some(ce) = &blob_properties.content_encoding {
            s = s.content_encoding(&ce[..]);
        }
        if let Some(cl) = &blob_properties.content_language {
            s = s.content_language(&cl[..]);
        }
        if let Some(cd) = &blob_properties.content_disposition {
            s = s.content_disposition(&cd[..]);
        }
        if let Some(cmd5) = &blob_properties.content_md5 {
            s = s.content_md5(cmd5);
        }
        s
    }

    setters! {
        lease_id: &'a LeaseId => Some(lease_id),
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
        cache_control: BlobCacheControl<'a> => Some(cache_control),
        content_type: BlobContentType<'a> => Some(content_type),
        content_encoding: BlobContentEncoding<'a> => Some(content_encoding),
        content_language: BlobContentLanguage<'a> => Some(content_language),
        content_disposition: BlobContentDisposition<'a> => Some(content_disposition),
        content_md5: BlobContentMD5 => Some(content_md5),
    }

    pub async fn execute(self) -> azure_core::Result<SetBlobPropertiesResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("comp", "properties");
        self.timeout.append_to_url_query(&mut url);

        let mut request =
            self.blob_client
                .prepare_request(url.as_str(), http::Method::PUT, None)?;
        request.add_optional_header(&self.client_request_id);
        request.add_optional_header_ref(&self.lease_id);
        request.add_optional_header(&self.cache_control);
        request.add_optional_header(&self.content_type);
        request.add_optional_header(&self.content_encoding);
        request.add_optional_header(&self.content_language);
        request.add_optional_header(&self.content_disposition);
        request.add_optional_header(&self.content_md5);

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        response.headers().try_into()
    }
}
