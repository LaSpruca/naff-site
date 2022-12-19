use crate::{auth::User as AuthUser, error::*};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{borrow::Cow, time::Duration};
use tracing::{debug, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub id: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    id: String,
    name: String,
    film_name: String,
    film_description: String,
    has_file: bool,
}

pub struct Db {
    connection: PgPool,
}

impl Db {
    pub fn new(connection: PgPool) -> Self {
        Self { connection }
    }

    pub async fn get_user(&self, user: AuthUser) -> Result<User, Error> {
        let res = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user.id)
            .fetch_optional(&self.connection)
            .await
            .map_err(|ex| {
                error!("Error fetching user {ex}");
                Error::InternalError
            })?;

        Ok(if res.is_none() {
            sqlx::query_as!(
                User,
                "INSERT INTO users (id, \"name\", email) VALUES ($1, $2, $3) RETURNING *",
                user.id,
                user.name,
                user.email
            )
            .fetch_one(&self.connection)
            .await
            .map_err(|ex| {
                error!("Error inserting user {ex}");
                Error::InternalError
            })?
        } else {
            debug!("Trolling complete, return to hq");
            res.unwrap()
        })
    }

    pub async fn get_team(&self, user: AuthUser) -> Result<Option<Team>, Error> {
        sqlx::query_as!(
            Team,
            r#"SELECT t.* FROM user_connection join teams t on t.id = user_connection.team where user_connection."user" = $1;"#,
            user.id,
        ).fetch_optional(&self.connection).await.map_err(|ex| {error!("Error fetching team: {ex}"); Error::InternalError})
    }

    pub async fn in_team(&self, user: AuthUser) -> Result<(), Error> {
        let in_team = sqlx::query!("SELECT has_team($1)", user.id)
            .fetch_one(&self.connection)
            .await
            .map_err(|x| {
                error!("Error checking for team: {x}");
                Error::InternalError
            })?
            .has_team
            // This is safe to unwrap because I know that it will always have a value, unless there was an error which will be reflected in the result
            .unwrap();

        if in_team {
            return Err(Error::InTeam);
        }

        Ok(())
    }

    pub async fn join_team(&self, user: AuthUser, team_id: String) -> Result<Team, Error> {
        self.in_team(user.clone()).await?;

        let team = sqlx::query_as!(Team, "SELECT * FROM teams WHERE id = $1", team_id)
            .fetch_one(&self.connection)
            .await
            .map_err(|x| match x {
                sqlx::Error::RowNotFound => Error::NoSuchTeam(team_id),
                _ => {
                    error!("{x}");
                    Error::InternalError
                }
            })?;

        sqlx::query!(
            "INSERT INTO user_connection (team, \"user\") VALUES ($1, $2)",
            team.id,
            user.id
        )
        .execute(&self.connection)
        .await
        .map_err(|x| {
            error!("{x}");
            Error::InternalError
        })?;

        Ok(team)
    }

    pub async fn create_team(&self, user: AuthUser, team_name: String) -> Result<Team, Error> {
        self.in_team(user.clone()).await?;

        let team = sqlx::query_as!(
            Team,
            "INSERT INTO teams (\"name\") VALUES ($1) RETURNING *",
            team_name
        )
        .fetch_one(&self.connection)
        .await
        .map_err(|x| match x {
            sqlx::Error::Database(ex) => {
                if ex.code() == Some(Cow::from("23505")) {
                    Error::TeamNameTaken(team_name)
                } else {
                    error!("Error inserting new team: {ex}");
                    Error::InternalError
                }
            }
            _ => {
                error!("{x}");
                Error::InternalError
            }
        })?;

        sqlx::query!(
            "INSERT INTO user_connection (\"user\", team) VALUES ($2, $1)",
            team.id,
            user.id
        )
        .execute(&self.connection)
        .await
        .map_err(|x| {
            error!("Couldn't add user to team after creation {x}");
            Error::InternalError
        })?;

        Ok(team)
    }

    pub async fn in_specific_team(&self, user: AuthUser, team_code: String) -> Result<(), Error> {
        if self
            .get_user(user.clone())
            .await
            .map(|x| x.is_admin)
            .unwrap_or(false)
            || sqlx::query!(
                "SELECT exists(SELECT 1 FROM user_connection WHERE \"user\" = $1 AND team = $2)",
                user.id,
                team_code
            )
            .fetch_one(&self.connection)
            .await
            .map_err(|x| {
                error!("Error checking if in team {x}");
                Error::InternalError
            })?
            .exists
            .unwrap()
        {
            Ok(())
        } else {
            Err(Error::TeamAccessDenied(team_code))
        }
    }

    pub async fn get_team_members(
        &self,
        user: AuthUser,
        team_code: String,
    ) -> Result<Vec<User>, Error> {
        self.in_specific_team(user, team_code.clone()).await?;

        sqlx::query_as!(User, "SELECT u.* FROM user_connection JOIN users u on user_connection.\"user\" = u.id WHERE team = $1;", team_code).fetch_all(&self.connection).await.or_else(|x| {
            match x {
                sqlx::Error::RowNotFound => Result::<Vec<User>, Error>::Ok(vec![]),
                _ => Err(Error::InternalError)
            }
        })
    }

    pub async fn leave_team(&self, user: AuthUser) -> Result<(), Error> {
        self.in_team(user.clone())
            .await
            .or_else(|x| match x {
                Error::InTeam => Ok(()),
                _ => Err(Error::InternalError),
            })
            .and(Err(Error::NotInTeam))?;

        sqlx::query!("DELETE FROM user_connection WHERE \"user\" = $1", user.id)
            .execute(&self.connection)
            .await
            .map_err(|x| {
                error!("Error removing user connection {x}");
                Error::InternalError
            })?;

        Ok(())
    }
}

pub async fn create_connection() -> Result<PgPool, Error> {
    let url = std::env::var("DATABASE_URL").to_crate("DATABASE_URL")?;

    PgPoolOptions::default()
        .acquire_timeout(Duration::from_secs(5))
        .max_connections(10)
        .connect(&url)
        .await
        .to_crate_conn()
}
