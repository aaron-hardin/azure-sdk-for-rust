#![doc = "generated by AutoRust"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
use super::models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = "https://169.254.169.254/metadata";
impl ClientBuilder {
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        self.pipeline.send(&mut context, request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn attested(&self) -> attested::Client {
        attested::Client(self.clone())
    }
    pub fn identity(&self) -> identity::Client {
        identity::Client(self.clone())
    }
    pub fn instances(&self) -> instances::Client {
        instances::Client(self.clone())
    }
}
pub mod instances {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn get_metadata(&self, metadata: impl Into<String>) -> get_metadata::Builder {
            get_metadata::Builder {
                client: self.0.clone(),
                metadata: metadata.into(),
            }
        }
    }
    pub mod get_metadata {
        use super::models;
        type Response = models::Instance;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) metadata: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/instance", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, http::Method::GET);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                        req.insert_header("Metadata", &this.metadata);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::Instance = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code.as_u16(),
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
}
pub mod attested {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn get_document(&self, metadata: impl Into<String>) -> get_document::Builder {
            get_document::Builder {
                client: self.0.clone(),
                metadata: metadata.into(),
                nonce: None,
            }
        }
    }
    pub mod get_document {
        use super::models;
        type Response = models::AttestedData;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) metadata: String,
            pub(crate) nonce: Option<String>,
        }
        impl Builder {
            pub fn nonce(mut self, nonce: impl Into<String>) -> Self {
                self.nonce = Some(nonce.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/attested/document", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, http::Method::GET);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                        if let Some(nonce) = &this.nonce {
                            req.url_mut().query_pairs_mut().append_pair("nonce", nonce);
                        }
                        req.insert_header("Metadata", &this.metadata);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::AttestedData = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code.as_u16(),
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
}
pub mod identity {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn get_token(&self, metadata: impl Into<String>, resource: impl Into<String>) -> get_token::Builder {
            get_token::Builder {
                client: self.0.clone(),
                metadata: metadata.into(),
                resource: resource.into(),
                client_id: None,
                object_id: None,
                msi_res_id: None,
                authority: None,
                bypass_cache: None,
            }
        }
        pub fn get_info(&self, metadata: impl Into<String>) -> get_info::Builder {
            get_info::Builder {
                client: self.0.clone(),
                metadata: metadata.into(),
            }
        }
    }
    pub mod get_token {
        use super::models;
        type Response = models::IdentityTokenResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) metadata: String,
            pub(crate) resource: String,
            pub(crate) client_id: Option<String>,
            pub(crate) object_id: Option<String>,
            pub(crate) msi_res_id: Option<String>,
            pub(crate) authority: Option<String>,
            pub(crate) bypass_cache: Option<String>,
        }
        impl Builder {
            pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
                self.client_id = Some(client_id.into());
                self
            }
            pub fn object_id(mut self, object_id: impl Into<String>) -> Self {
                self.object_id = Some(object_id.into());
                self
            }
            pub fn msi_res_id(mut self, msi_res_id: impl Into<String>) -> Self {
                self.msi_res_id = Some(msi_res_id.into());
                self
            }
            pub fn authority(mut self, authority: impl Into<String>) -> Self {
                self.authority = Some(authority.into());
                self
            }
            pub fn bypass_cache(mut self, bypass_cache: impl Into<String>) -> Self {
                self.bypass_cache = Some(bypass_cache.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/identity/oauth2/token", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, http::Method::GET);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                        req.insert_header("Metadata", &this.metadata);
                        let resource = &this.resource;
                        req.url_mut().query_pairs_mut().append_pair("resource", resource);
                        if let Some(client_id) = &this.client_id {
                            req.url_mut().query_pairs_mut().append_pair("client_id", client_id);
                        }
                        if let Some(object_id) = &this.object_id {
                            req.url_mut().query_pairs_mut().append_pair("object_id", object_id);
                        }
                        if let Some(msi_res_id) = &this.msi_res_id {
                            req.url_mut().query_pairs_mut().append_pair("msi_res_id", msi_res_id);
                        }
                        if let Some(authority) = &this.authority {
                            req.url_mut().query_pairs_mut().append_pair("authority", authority);
                        }
                        if let Some(bypass_cache) = &this.bypass_cache {
                            req.url_mut().query_pairs_mut().append_pair("bypass_cache", bypass_cache);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::IdentityTokenResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code.as_u16(),
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_info {
        use super::models;
        type Response = models::IdentityInfoResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) metadata: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/identity/info", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, http::Method::GET);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-01-01");
                        req.insert_header("Metadata", &this.metadata);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::IdentityInfoResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code.as_u16(),
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
}
