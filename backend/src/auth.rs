use crate::{data::Auth0Config, db::Db, error::*};
use actix_web::{web::Data, FromRequest};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{
    decode,
    jwk::{AlgorithmParameters, JwkSet},
    Algorithm, DecodingKey, Validation,
};
use serde::{Deserialize, Serialize};

async fn parse_jwt(
    jwks: &JwkSet,
    token: &str,
    Auth0Config {
        domain, client_id, ..
    }: &Auth0Config,
) -> Result<User, Error> {
    let header = jwt::decode_header(&token).map_err(|_| Error::Unauthorized)?;
    let kid = header.kid.ok_or(Error::Unauthorized)?;
    let jwk = jwks.find(&kid).ok_or(Error::Unauthorized)?;

    match jwk.clone().algorithm {
        AlgorithmParameters::RSA(ref rsa) => {
            let mut validation = Validation::new(Algorithm::RS256);
            validation.set_audience(&[client_id]);
            validation.set_issuer(&[format!("https://{domain}/").as_str()]);
            let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                .map_err(|_| Error::Unauthorized)?;
            let token =
                decode::<User>(token, &key, &validation).map_err(|_| Error::Unauthorized)?;

            Ok(token.claims)
        }
        _ => Err(Error::Unauthorized),
    }
}

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
            let auth0_config = req.app_data::<Data<Auth0Config>>().unwrap().as_ref();
            let jwks = req.app_data::<Data<JwkSet>>().unwrap().as_ref();

            let token = req
                .cookie("access_token")
                .map(|x| Some(x.value().to_owned()))
                .unwrap_or_else(|| {
                    req.headers()
                        .get("Authorization")
                        .and_then(|x| x.to_str().ok().map(|x| x.to_owned()))
                })
                .ok_or(Error::Unauthorized)?;

            parse_jwt(jwks, token.as_str(), auth0_config).await
        })
    }
}

pub struct AdminUser(User);

impl FromRequest for AdminUser {
    type Error = Error;

    type Future = LocalBoxFuture<'static, Result<Self, Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let auth0_config = req.app_data::<Data<Auth0Config>>().unwrap().as_ref();
            let jwks = req.app_data::<Data<JwkSet>>().unwrap().as_ref();
            let db = req.app_data::<Data<Db>>().unwrap().as_ref();

            let token = req
                .cookie("access_token")
                .map(|x| Some(x.value().to_owned()))
                .unwrap_or_else(|| {
                    req.headers()
                        .get("Authorization")
                        .and_then(|x| x.to_str().ok().map(|x| x.to_owned()))
                })
                .ok_or(Error::Unauthorized)?;

            let user = parse_jwt(jwks, token.as_str(), auth0_config).await?;

            if db.is_user_admin(&user).await.unwrap_or(false) {
                Ok(AdminUser(user))
            } else {
                Err(Error::Unauthorized)
            }
        })
    }
}
