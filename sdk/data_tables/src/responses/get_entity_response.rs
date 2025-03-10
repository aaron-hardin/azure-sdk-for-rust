use azure_core::{error::Error, headers::etag_from_headers, CollectedResponse, Etag};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use serde::de::DeserializeOwned;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct GetEntityResponse<E>
where
    E: DeserializeOwned,
{
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub metadata: String,
    pub entity: E,
    pub etag: Etag,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetEntityResponseInternal<E> {
    #[serde(rename = "odata.metadata")]
    pub metadata: String,
    #[serde(flatten)]
    pub value: E,
}

impl<E> TryFrom<CollectedResponse> for GetEntityResponse<E>
where
    E: DeserializeOwned,
{
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        let get_entity_response_internal: GetEntityResponseInternal<E> =
            serde_json::from_slice(response.body())?;

        Ok(GetEntityResponse {
            common_storage_response_headers: response.headers().try_into()?,
            metadata: get_entity_response_internal.metadata,
            entity: get_entity_response_internal.value,
            etag: etag_from_headers(response.headers())?.into(),
        })
    }
}
