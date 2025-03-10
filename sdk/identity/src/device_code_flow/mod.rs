//! Authorize using the device authorization grant flow
//!
//! This flow allows users to sign in to input-constrained devices such as a smart TV, IoT device, or printer.
//!
//! You can learn more about this authorization flow [here](https://docs.microsoft.com/azure/active-directory/develop/v2-oauth2-device-code).
mod device_code_responses;

use async_timer::timer::new_timer;
use azure_core::Method;
use azure_core::{
    content_type,
    error::{Error, ErrorKind},
    headers, HttpClient, Request, Response,
};
pub use device_code_responses::*;
use futures::stream::unfold;
use oauth2::ClientId;
use serde::Deserialize;
use std::{borrow::Cow, sync::Arc, time::Duration};
use url::{form_urlencoded, Url};

/// Start the device authorization grant flow.
/// The user has only 15 minutes to sign in (the usual value for expires_in).
pub async fn start<'a, 'b, T>(
    http_client: Arc<dyn HttpClient>,
    tenant_id: T,
    client_id: &'a ClientId,
    scopes: &'b [&'b str],
) -> azure_core::Result<DeviceCodePhaseOneResponse<'a>>
where
    T: Into<Cow<'a, str>>,
{
    let tenant_id = tenant_id.into();
    let url = &format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/devicecode",
        tenant_id
    );

    let mut encoded = form_urlencoded::Serializer::new(String::new());
    let encoded = encoded.append_pair("client_id", client_id.as_str());
    let encoded = encoded.append_pair("scope", &scopes.join(" "));
    let encoded = encoded.finish();

    let rsp = post_form(http_client.clone(), url, encoded).await?;
    let rsp_status = rsp.status();
    let rsp_body = rsp.into_body().await;
    if !rsp_status.is_success() {
        return Err(ErrorKind::http_response_from_body(rsp_status as u16, &rsp_body).into_error());
    }
    let device_code_response: DeviceCodePhaseOneResponse = serde_json::from_slice(&rsp_body)?;

    // we need to capture some variables that will be useful in
    // the second phase (the client, the tenant_id and the client_id)
    Ok(DeviceCodePhaseOneResponse {
        device_code: device_code_response.device_code,
        user_code: device_code_response.user_code,
        verification_uri: device_code_response.verification_uri,
        expires_in: device_code_response.expires_in,
        interval: device_code_response.interval,
        message: device_code_response.message,
        http_client: Some(http_client),
        tenant_id,
        client_id: client_id.as_str().to_string(),
    })
}

/// Contains the required information to allow a user to sign in.
#[derive(Debug, Clone, Deserialize)]
pub struct DeviceCodePhaseOneResponse<'a> {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
    message: String,
    // The skipped fields below do not come from the Azure answer.
    // They will be added manually after deserialization
    #[serde(skip)]
    http_client: Option<Arc<dyn HttpClient>>,
    #[serde(skip)]
    tenant_id: Cow<'a, str>,
    // We store the ClientId as string instead of the original type, because it
    // does not implement Default, and it's in another crate
    #[serde(skip)]
    client_id: String,
}

impl<'a> DeviceCodePhaseOneResponse<'a> {
    /// The message containing human readable instructions for the user.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Polls the token endpoint while the user signs in.
    /// This will continue until either success or error is returned.
    pub fn stream(
        &self,
    ) -> impl futures::Stream<Item = azure_core::Result<DeviceCodeAuthorization>> + '_ {
        #[derive(Debug, Clone, PartialEq)]
        enum NextState {
            Continue,
            Finish,
        }

        unfold(NextState::Continue, move |state: NextState| async move {
            match state {
                NextState::Continue => {
                    let url = &format!(
                        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                        self.tenant_id,
                    );

                    // Throttle down as specified by Azure. This could be
                    // smarter: we could calculate the elapsed time since the
                    // last poll and wait only the delta.
                    new_timer(Duration::from_secs(self.interval)).await;

                    let mut encoded = form_urlencoded::Serializer::new(String::new());
                    let encoded = encoded
                        .append_pair("grant_type", "urn:ietf:params:oauth:grant-type:device_code");
                    let encoded = encoded.append_pair("client_id", self.client_id.as_str());
                    let encoded = encoded.append_pair("device_code", &self.device_code);
                    let encoded = encoded.finish();

                    let http_client = self.http_client.clone().unwrap();

                    match post_form(http_client.clone(), url, encoded).await {
                        Ok(rsp) => {
                            let rsp_status = rsp.status();
                            let rsp_body = rsp.into_body().await;
                            if rsp_status.is_success() {
                                match serde_json::from_slice::<DeviceCodeAuthorization>(&rsp_body) {
                                    Ok(authorization) => {
                                        Some((Ok(authorization), NextState::Finish))
                                    }
                                    Err(error) => {
                                        Some((Err(Error::from(error)), NextState::Finish))
                                    }
                                }
                            } else {
                                match serde_json::from_slice::<DeviceCodeErrorResponse>(&rsp_body) {
                                    Ok(error_rsp) => {
                                        let next_state =
                                            if error_rsp.error == "authorization_pending" {
                                                NextState::Continue
                                            } else {
                                                NextState::Finish
                                            };
                                        Some((
                                            Err(Error::new(ErrorKind::Credential, error_rsp)),
                                            next_state,
                                        ))
                                    }
                                    Err(error) => {
                                        Some((Err(Error::from(error)), NextState::Finish))
                                    }
                                }
                            }
                        }
                        Err(error) => Some((Err(error), NextState::Finish)),
                    }
                }
                NextState::Finish => None,
            }
        })
    }
}

async fn post_form(
    http_client: Arc<dyn HttpClient>,
    url: &str,
    form_body: String,
) -> azure_core::Result<Response> {
    let url = Url::parse(url)?;
    let mut req = Request::new(url, Method::Post);
    req.insert_header(
        headers::CONTENT_TYPE,
        content_type::APPLICATION_X_WWW_FORM_URLENCODED,
    );
    req.set_body(form_body);
    http_client.execute_request(&req).await
}
