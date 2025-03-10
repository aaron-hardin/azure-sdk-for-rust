use crate::{prelude::*, ContinuationNextTableName};
use azure_core::{error::Error, CollectedResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct ListTablesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub metadata: String,
    pub tables: Vec<Table>,
    pub continuation_next_table_name: Option<ContinuationNextTableName>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ListTablesResponseInternal {
    #[serde(rename = "odata.metadata")]
    pub metadata: String,
    #[serde(default = "Vec::new")]
    pub value: Vec<Table>,
}

impl TryFrom<CollectedResponse> for ListTablesResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        let list_tables_response_internal: ListTablesResponseInternal =
            serde_json::from_slice(response.body())?;

        Ok(ListTablesResponse {
            common_storage_response_headers: response.headers().try_into()?,
            metadata: list_tables_response_internal.metadata,
            tables: list_tables_response_internal.value,
            continuation_next_table_name: ContinuationNextTableName::from_header_optional(
                response.headers(),
            )?,
        })
    }
}
