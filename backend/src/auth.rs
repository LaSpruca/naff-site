use crate::{data::Auth0Config, error::*};
use actix_web::{web::Data, FromRequest};
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(alias = "sub")]
    pub id: String,
    pub name: String,
    pub email: String,
}

impl FromRequest for User {
    type Error = Error;

    type Future = LocalBoxFuture<'static, Result<Self, Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let Auth0Config { domain, .. } = req.app_data::<Data<Auth0Config>>().unwrap().as_ref();

            let token = req
                .cookie("access_token")
                .map(|x| Some(x.value().to_owned()))
                .unwrap_or_else(|| {
                    req.headers()
                        .get("Authorization")
                        .and_then(|x| x.to_str().ok().map(|x| x.to_owned()))
                })
                .ok_or(Error::Unauthorized)?;

            let user_request = reqwest::Client::default()
                .get(format!("https://{domain}/userinfo"))
                .bearer_auth(token)
                .send()
                .await
                .map_err(|_| Error::InternalError)?;

            if user_request.status() != 200 {
                return Err(Error::Unauthorized);
            }

            user_request.json().await.map_err(|_| Error::InternalError)
        })
    }
}
