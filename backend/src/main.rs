extern crate jsonwebtoken as jwt;

mod api;
mod auth;
mod data;
mod db;
mod error;
mod jwt_helpers;

use crate::{api::auth::auth, data::get_config, db::Db};
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use db::create_connection;
use error::AsCreateError;
pub use error::Error;
use sqlx::migrate::Migrator;
use std::{io, process::ExitCode};
use tracing::{error, info};
use tracing_actix_web::TracingLogger;

static MIGRATOR: Migrator = sqlx::migrate!();

async fn start() -> Result<(), Error> {
    info!("Starting application");

    let pool = create_connection().await?;

    MIGRATOR.run(&pool).await.to_crate()?;

    let (auth0, public) = get_config()?;

    let jwk = jwt_helpers::get_kwks(&auth0).await.ok_or_else(|| {
        error::Error::ServerStartError(io::Error::new(io::ErrorKind::Other, "Could not fetch JWKs"))
    })?;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_headers(["Cookie", "Authorization"])
                    .allow_any_origin()
                    .allowed_methods(["GET", "POST"])
                    .supports_credentials(),
            )
            .wrap(TracingLogger::default())
            .app_data(Data::new(Db::new(pool.clone())))
            // .app_data(Data::new(Mutex::new(States::new())))
            .app_data(Data::new(auth0.clone()))
            .app_data(Data::new(public.clone()))
            .app_data(Data::new(jwk.clone()))
            .service(api::api())
            .service(auth())
    })
    .bind("0.0.0.0:8080")
    .map_err(|ex| Error::ServerStartError(ex))?
    .run()
    .await
    .map_err(|ex| Error::ServerStartError(ex))?;

    Ok(())
}

fn main() -> ExitCode {
    #[allow(unused_must_use)]
    {
        dotenv::dotenv();
    }
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Err(ex) => {
                error!("Could not start the runtime {ex}");
                ExitCode::from(1)
            }
            Ok(executor) => executor
                .block_on(start())
                .err()
                .map(|val| {
                    error!("{val}");
                    val.to_code()
                })
                .unwrap_or(ExitCode::SUCCESS),
        };
    }
}
