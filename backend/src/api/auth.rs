use std::collections::HashMap;

use crate::data::UrlConfig;
use crate::data::{Auth0Config, States};
use actix_web::cookie::time::Duration;
use actix_web::cookie::{CookieBuilder, SameSite};
use actix_web::web::Query;
use actix_web::Scope;
use actix_web::{get, http::header, web::Data, HttpResponse, Responder};
use serde::Deserialize;
use tokio::sync::Mutex;
use tracing::debug;

#[derive(Deserialize, Clone)]
pub struct OAuthTokenResponse {
    access_token: String,
    expires_in: i32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Auth0Url {
    code: String,
    state: String,
}

pub fn auth() -> actix_web::Scope {
    Scope::new("/auth")
        .service(login)
        .service(login_callback)
        .service(logout)
}

#[get("/login")]
async fn login(
    states: Data<Mutex<States>>,
    auth0: Data<Auth0Config>,
    urls: Data<UrlConfig>,
) -> HttpResponse {
    let UrlConfig { backend, .. } = urls.as_ref();
    let Auth0Config {
        client_id, domain, ..
    } = auth0.as_ref();

    let random_string = states.lock().await.add();

    let callback_url = format!("{backend}/auth/callback?");
    let callback_url = urlencoding::encode(&callback_url).to_string();

    HttpResponse::Found()
        .append_header((
            header::LOCATION,
            format!(
                "https://{domain}/authorize?\
client_id={client_id}&\
redirect_uri={callback_url}&\
response_type=code&\
state={random_string}&\
scope=openid profile email user_id"
            ),
        ))
        .finish()
}

#[get("/callback")]
async fn login_callback(
    req: Query<Auth0Url>,
    states: Data<Mutex<States>>,
    config: Data<Auth0Config>,
    url_config: Data<UrlConfig>,
) -> impl Responder {
    let UrlConfig { frontend, backend } = url_config.as_ref();

    let Auth0Config {
        client_id,
        domain,
        client_secret,
    } = config.get_ref();

    let Auth0Url { code, state } = req.0;

    if states.lock().await.check(&state) {
        // if true {
        let callback_url = format!("{backend}/auth/callback?");

        let params = HashMap::from([
            ("grant_type", "authorization_code"),
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("code", code.as_str()),
            ("redirect_uri", callback_url.as_str()),
        ]);

        let request = reqwest::Client::new()
            .post(format!("https://{domain}/oauth/token"))
            .form(&params)
            .send()
            .await;

        if let Ok(res) = request {
            if res.status() != 200 {
                debug!("Failed to get token: {}", res.text().await.unwrap());
                return HttpResponse::InternalServerError().body("Couldn't get token");
            }

            let text = res.text().await.unwrap();

            debug!("{text}");

            let OAuthTokenResponse {
                access_token,
                expires_in,
            } = serde_json::from_str(&text).unwrap();

            HttpResponse::TemporaryRedirect()
                .append_header((header::LOCATION, format!("{frontend}/participate")))
                .append_header((
                    header::SET_COOKIE,
                    CookieBuilder::new("access_token", access_token)
                        .max_age(Duration::seconds(expires_in as i64))
                        .http_only(true)
                        .path("/")
                        .same_site(SameSite::None)
                        .secure(true)
                        .domain(
                            frontend
                                .strip_prefix("https://")
                                .or_else(|| {
                                    frontend
                                        .strip_prefix("http://")
                                        .unwrap()
                                        .strip_suffix(":5173")
                                })
                                .unwrap(),
                        )
                        .finish()
                        .to_string(),
                ))
                .finish()
        } else {
            eprintln!("{}", request.unwrap_err());
            HttpResponse::InternalServerError().body("Could not get new token")
        }
    } else {
        HttpResponse::BadRequest().body("Invalid state")
    }
}
#[get("/logout")]
async fn logout(public_config: Data<UrlConfig>) -> HttpResponse {
    HttpResponse::TemporaryRedirect()
        .append_header((
            header::SET_COOKIE,
            CookieBuilder::new("access_token", "")
                .max_age(Duration::seconds(-1))
                .path("/")
                .http_only(true)
                .finish()
                .to_string(),
        ))
        .append_header((header::LOCATION, format!("{}", public_config.frontend)))
        .finish()
}
