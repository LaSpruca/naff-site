use jsonwebtoken::jwk::JwkSet;
use tracing::{debug, error};

use crate::data::Auth0Config;

pub async fn get_kwks(Auth0Config { domain, .. }: &Auth0Config) -> Option<JwkSet> {
    debug!("Fetching JWKs from https://{domain}/.well-known/jwks.json");
    let result: Result<JwkSet, _> =
        reqwest::get(&format!("https://{domain}/.well-known/jwks.json"))
            .await
            .ok()?
            .json()
            .await;

    if let Err(ex) = result {
        error!("{ex}");
        None
    } else {
        debug!("Got JWKs");
        Some(result.unwrap())
    }
}
