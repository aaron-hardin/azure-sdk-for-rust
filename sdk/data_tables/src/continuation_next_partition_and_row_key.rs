use azure_core::{
    headers::{HeaderName, Headers},
    AppendToUrlQuery,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinuationNextPartitionAndRowKey(String, Option<String>);

impl ContinuationNextPartitionAndRowKey {
    pub fn new(
        continuation_next_partition_key: String,
        continuation_next_row_key: Option<String>,
    ) -> Self {
        Self(continuation_next_partition_key, continuation_next_row_key)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_header_optional(headers: &Headers) -> azure_core::Result<Option<Self>> {
        let partition_header_as_str = headers.get_optional_str(&HeaderName::from_static(
            "x-ms-continuation-NextPartitionKey",
        ));

        let row_header_as_str =
            headers.get_optional_str(&HeaderName::from_static("x-ms-continuation-NextRowKey"));

        Ok(partition_header_as_str.filter(|h| !h.is_empty()).map(|h| {
            ContinuationNextPartitionAndRowKey::new(
                h.to_owned(),
                row_header_as_str.map(|h| h.to_owned()),
            )
        }))
    }
}

impl AppendToUrlQuery for ContinuationNextPartitionAndRowKey {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("NextPartitionKey", &self.0);

        if let Some(row_key) = &self.1 {
            url.query_pairs_mut().append_pair("NextRowKey", row_key);
        }
    }
}
