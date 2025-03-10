//! Authorize using the OAuth 2.0 client credentials flow
//!
//! For example:
//!
//! ```no_run
//! use azure_identity::client_credentials_flow;
//! use oauth2::{ClientId, ClientSecret};
//! use url::Url;
//!
//! use std::env;
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let client_id =
//!         ClientId::new(env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable."));
//!     let client_secret = ClientSecret::new(
//!         env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable."),
//!     );
//!     let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
//!     let subscription_id =
//!         env::var("SUBSCRIPTION_ID").expect("Missing SUBSCRIPTION_ID environment variable.");
//!
//!     let http_client = azure_core::new_http_client();
//!     // This will give you the final token to use in authorization.
//!     let token = client_credentials_flow::perform(
//!         http_client.clone(),
//!         &client_id,
//!         &client_secret,
//!         &["https://management.azure.com/"],
//!         &tenant_id,
//!     )
//!     .await?;
//!     Ok(())
//! }
//! ```
//!
//! You can learn more about this authorization flow [here](https://docs.microsoft.com/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow).

mod login_response;

use azure_core::Method;
use azure_core::{
    content_type,
    error::{ErrorKind, ResultExt},
    headers, HttpClient, Request,
};
use login_response::LoginResponse;
use std::sync::Arc;
use url::{form_urlencoded, Url};

/// Perform the client credentials flow
#[allow(clippy::manual_async_fn)]
#[fix_hidden_lifetime_bug::fix_hidden_lifetime_bug]
pub async fn perform(
    http_client: Arc<dyn HttpClient>,
    client_id: &oauth2::ClientId,
    client_secret: &oauth2::ClientSecret,
    scopes: &[&str],
    tenant_id: &str,
) -> azure_core::Result<LoginResponse> {
    let encoded: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", client_id.as_str())
        .append_pair("scope", &scopes.join(" "))
        .append_pair("client_secret", client_secret.secret())
        .append_pair("grant_type", "client_credentials")
        .finish();

    let url = Url::parse(&format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        tenant_id
    ))
    .with_context(ErrorKind::DataConversion, || {
        format!("The supplied tenant id could not be url encoded: {tenant_id}")
    })?;

    let mut req = Request::new(url, Method::Post);
    req.insert_header(
        headers::CONTENT_TYPE,
        content_type::APPLICATION_X_WWW_FORM_URLENCODED,
    );
    req.set_body(encoded);
    let rsp = http_client.execute_request(&req).await?;
    let rsp_status = rsp.status();
    let rsp_body = rsp.into_body().await;
    if !rsp_status.is_success() {
        return Err(ErrorKind::http_response_from_body(rsp_status as u16, &rsp_body).into_error());
    }
    serde_json::from_slice(&rsp_body).map_kind(ErrorKind::DataConversion)
}
