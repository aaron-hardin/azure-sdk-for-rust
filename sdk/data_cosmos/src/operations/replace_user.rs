use crate::{prelude::*, resources::user::UserResponse};
use azure_core::Context;

#[derive(Debug, Clone)]
pub struct ReplaceUserBuilder {
    client: UserClient,
    user_name: String,
    consistency_level: Option<ConsistencyLevel>,
    context: Context,
}

impl ReplaceUserBuilder {
    pub(crate) fn new(client: UserClient, user_name: String) -> Self {
        Self {
            client,
            user_name,
            consistency_level: None,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        context: Context => context,
    }

    pub fn into_future(self) -> ReplaceUser {
        Box::pin(async move {
            let mut request = self.client.user_request(azure_core::Method::Put);

            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }
            let body = ReplaceUserBody {
                id: &self.user_name,
            };
            request.set_body(serde_json::to_vec(&body)?);
            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Users),
                    &mut request,
                )
                .await?;

            UserResponse::try_from(response).await
        })
    }
}

#[derive(Serialize)]
struct ReplaceUserBody<'a> {
    id: &'a str,
}

/// The future returned by calling `into_future` on the builder.
pub type ReplaceUser = futures::future::BoxFuture<'static, azure_core::Result<UserResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for ReplaceUserBuilder {
    type IntoFuture = ReplaceUser;
    type Output = <ReplaceUser as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
