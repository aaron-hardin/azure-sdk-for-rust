use azure_core::{
    error::{Error, ErrorKind},
    CollectedResponse, Etag, StatusCode,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::{TryFrom, TryInto};
use url::Url;

#[derive(Debug, Clone)]
pub struct OperationResponse {
    pub status_code: StatusCode,
    pub location: Option<Url>,
    pub data_service_id: Option<String>,
    pub etag: Option<Etag>,
}

impl Default for OperationResponse {
    fn default() -> Self {
        Self {
            status_code: StatusCode::Ok,
            location: None,
            data_service_id: None,
            etag: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubmitTransactionResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub operation_responses: Vec<OperationResponse>,
}

impl TryFrom<CollectedResponse> for SubmitTransactionResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        let body = std::str::from_utf8(response.body())?;

        let mut operation_responses = Vec::new();

        for change_set_response in body
            .split("\n--changesetresponse_")
            .into_iter()
            .filter(|change_set_response| change_set_response.contains("HTTP/1.1"))
        {
            trace!("changeset --> {}", change_set_response);

            let mut operation_response = OperationResponse::default();

            for line in change_set_response.lines() {
                if line.starts_with("HTTP/1.1") {
                    let status_code = line.split_whitespace().nth(1).ok_or_else(|| {
                        Error::message(ErrorKind::Other, "missing HTTP status code")
                    })?;
                    let status_code = status_code.parse::<u16>().map_err(|_| {
                        Error::with_message(ErrorKind::DataConversion, || {
                            format!("invalid HTTP status code `{status_code}`")
                        })
                    })?;
                    operation_response.status_code =
                        StatusCode::try_from(status_code).map_err(|_| {
                            Error::with_message(ErrorKind::DataConversion, || {
                                format!("invalid status code {status_code}")
                            })
                        })?;
                } else if line.starts_with("Location:") {
                    operation_response.location = Some(
                        line.split_whitespace()
                            .nth(1)
                            .ok_or_else(|| {
                                Error::message(ErrorKind::Other, "invalid Location header")
                            })?
                            .parse()?,
                    );
                } else if line.starts_with("DataServiceId:") {
                    operation_response.data_service_id = Some(
                        line.split_whitespace()
                            .nth(1)
                            .ok_or_else(|| {
                                {
                                    {
                                        Error::message(
                                            ErrorKind::Other,
                                            "invalid DataServiceId header",
                                        )
                                    }
                                }
                            })?
                            .to_owned(),
                    );
                } else if line.starts_with("ETag:") {
                    operation_response.etag = Some(
                        line.split_whitespace()
                            .nth(1)
                            .ok_or_else(|| Error::message(ErrorKind::Other, "invalid ETag header"))?
                            .into(),
                    );
                }
            }

            operation_responses.push(operation_response);
        }

        Ok(SubmitTransactionResponse {
            common_storage_response_headers: response.headers().try_into()?,
            operation_responses,
        })
    }
}
