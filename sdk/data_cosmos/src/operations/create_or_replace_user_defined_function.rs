use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::UserDefinedFunction;
use crate::ResourceQuota;

use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::prelude::*;
use azure_core::{collect_pinned_stream, Response as HttpResponse};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct CreateOrReplaceUserDefinedFunctionBuilder {
    client: UserDefinedFunctionClient,
    is_create: bool,
    body: String,
    consistency_level: Option<ConsistencyLevel>,
    context: Context,
}

impl CreateOrReplaceUserDefinedFunctionBuilder {
    pub(crate) fn new(client: UserDefinedFunctionClient, is_create: bool, body: String) -> Self {
        Self {
            client,
            is_create,
            body,
            consistency_level: None,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        context: Context => context,
    }

    pub fn into_future(self) -> CreateOrReplaceUserDefinedFunction {
        Box::pin(async move {
            let mut request = match self.is_create {
                true => self.client.udfs_request(azure_core::Method::Post),
                false => self.client.udf_request(azure_core::Method::Put),
            };

            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }

            #[derive(Debug, Serialize)]
            struct Request<'a> {
                body: &'a str,
                id: &'a str,
            }
            let request_body = Request {
                body: &self.body,
                id: self.client.user_defined_function_name(),
            };
            request.set_body(serde_json::to_vec(&request_body)?);
            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Permissions),
                    &mut request,
                )
                .await?;

            CreateOrReplaceUserDefinedFunctionResponse::try_from(response).await
        })
    }
}

/// The future returned by calling `into_future` on the builder.
pub type CreateOrReplaceUserDefinedFunction = futures::future::BoxFuture<
    'static,
    azure_core::Result<CreateOrReplaceUserDefinedFunctionResponse>,
>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for CreateOrReplaceUserDefinedFunctionBuilder {
    type IntoFuture = CreateOrReplaceUserDefinedFunction;
    type Output = <CreateOrReplaceUserDefinedFunction as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateOrReplaceUserDefinedFunctionResponse {
    pub user_defined_function: UserDefinedFunction,
    pub server: String,
    pub last_state_change: DateTime<Utc>,
    pub etag: String,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub schema_version: String,
    pub alt_content_path: String,
    pub content_path: String,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
    pub role: u32,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_quorum_acked_llsn: u64,
    pub session_token: String,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: DateTime<Utc>,
}

impl CreateOrReplaceUserDefinedFunctionResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        Ok(Self {
            user_defined_function: serde_json::from_slice(&body)?,
            server: server_from_headers(&headers)?.to_owned(),
            last_state_change: last_state_change_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(&headers)?.to_owned(),
            content_path: content_path_from_headers(&headers)?.to_owned(),
            quorum_acked_lsn: quorum_acked_lsn_from_headers(&headers)?,
            current_write_quorum: current_write_quorum_from_headers(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(&headers)?,
            role: role_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?.to_owned(),
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
            date: date_from_headers(&headers)?,
        })
    }
}
