use std::{env::VarError, io, process::ExitCode};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde_json::json;
use sqlx::PgPool;

pub trait AsCreateError<T> {
    fn to_crate(self) -> Result<T, Error>;
}

impl<T> AsCreateError<T> for Result<T, sqlx::Error> {
    fn to_crate(self) -> Result<T, Error> {
        match self {
            Ok(val) => Ok(val),
            Err(ex) => Err(Error::DbQueryError(ex)),
        }
    }
}

impl<T> AsCreateError<T> for Result<T, sqlx::migrate::MigrateError> {
    fn to_crate(self) -> Result<T, Error> {
        self.map_err(|ex| Error::DbMigrationError(ex))
    }
}

impl<T> AsCreateError<T> for Result<T, envy::Error> {
    fn to_crate(self) -> Result<T, Error> {
        self.map_err(|x| Error::EnvyError(x))
    }
}

pub trait DbConnectionError {
    fn to_crate_conn(self) -> Result<PgPool, Error>;
}

impl DbConnectionError for Result<PgPool, sqlx::Error> {
    fn to_crate_conn(self) -> Result<PgPool, Error> {
        match self {
            Ok(val) => Ok(val),
            Err(ex) => Err(Error::DbConnectError(ex)),
        }
    }
}

pub trait EnvErrorExt {
    fn to_crate(self, var: impl ToString) -> Result<String, Error>;
}

impl EnvErrorExt for Result<String, VarError> {
    fn to_crate(self, var: impl ToString) -> Result<String, Error> {
        match self {
            Err(_) => Err(Error::EnvVarMissing(var.to_string())),
            Ok(val) => Ok(val),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Missing environment variable: {0}")]
    EnvVarMissing(String),

    #[error("Couldn't connect to Database: {0}")]
    DbConnectError(sqlx::Error),

    #[error("Couldn't execute query: {0}")]
    DbQueryError(sqlx::Error),

    #[error("Error running migrations: {0}")]
    DbMigrationError(sqlx::migrate::MigrateError),

    #[error("Could not start server")]
    ServerStartError(io::Error),

    #[error("Not implemented")]
    NotImplemented,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Missing environment variable")]
    EnvyError(envy::Error),

    #[error("There was an internal server error, please try again later")]
    InternalError,

    #[error("You cannot join/found a team because you are currently in one. Please leave the current team before retrying")]
    InTeam,

    #[error("No team with the code {0} exists")]
    NoSuchTeam(String),

    #[error("Sorry, the team name {0} was taken, choose another one")]
    TeamNameTaken(String),

    #[error("You do not have permission to access the team {0}")]
    TeamAccessDenied(String),

    #[error("You have to be in a team before you can leave it")]
    NotInTeam,
}

impl Error {
    pub fn as_number(&self) -> u8 {
        match self {
            Error::EnvVarMissing(_) | Error::EnvyError(_) => 1,
            Error::ServerStartError(_) => 2,
            Error::DbConnectError(_) => 3,
            Error::DbQueryError(_) | Error::DbMigrationError(_) => 4,

            Error::NotInTeam => 238,
            Error::TeamAccessDenied(_) => 239,
            Error::TeamNameTaken(_) => 240,
            Error::InTeam => 241,
            Error::NoSuchTeam(_) => 242,
            Error::Unauthorized => 243,
            Error::NotImplemented => 254,
            Error::InternalError => 255,
        }
    }

    pub fn to_code(self) -> ExitCode {
        ExitCode::from(self.as_number())
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::InTeam | Error::NoSuchTeam(_) | Error::NotInTeam => StatusCode::BAD_REQUEST,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::TeamAccessDenied(_) => StatusCode::FORBIDDEN,
            Error::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({
            "code": self.as_number(),
            "error": format!("{self}")
        }))
    }
}
