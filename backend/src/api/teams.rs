use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse,
};
use serde::Deserialize;

use crate::{auth::User, db::Db, Error};

#[derive(Deserialize)]
pub struct TeamParams {
    id: String,
}

#[derive(Deserialize)]
pub struct CreateTeamParams {
    name: String,
}

#[post("/join")]
pub async fn join_team(
    db: Data<Db>,
    user: User,
    params: web::Query<TeamParams>,
) -> Result<HttpResponse, Error> {
    let id = params.id.clone();
    db.join_team(user, id.to_owned())
        .await
        .map(|x| HttpResponse::Ok().json(x))
}

#[post("/new")]
pub async fn create_team(
    db: Data<Db>,
    user: User,
    params: web::Query<CreateTeamParams>,
) -> Result<HttpResponse, Error> {
    let name = params.name.clone();
    db.create_team(user, name.to_owned())
        .await
        .map(|x| HttpResponse::Ok().json(x))
}

#[post("/leave")]
pub async fn leave_team(db: Data<Db>, user: User) -> Result<HttpResponse, Error> {
    db.leave_team(user)
        .await
        .map(|x| HttpResponse::Ok().json(x))
}

#[get("/{id}/members")]
pub async fn get_members(
    db: Data<Db>,
    user: User,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    db.get_team_members(user, id.into_inner())
        .await
        .map(|x| HttpResponse::Ok().json(x))
}
