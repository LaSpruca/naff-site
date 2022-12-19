use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Scope,
};
use serde::Deserialize;

use crate::{auth::User, db::Db, Error};

pub fn service() -> Scope {
    Scope::new("/team")
        .service(get_team)
        .service(join_team)
        .service(create_team)
        .service(get_members)
        .service(leave_team)
}

#[derive(Deserialize)]
struct TeamParams {
    id: String,
}

#[derive(Deserialize)]
struct CreateTeamParams {
    name: String,
}

#[get("/")]
async fn get_team(db: web::Data<Db>, user: User) -> Result<HttpResponse, Error> {
    db.get_team(user).await.map(|x| HttpResponse::Ok().json(x))
}

#[post("/join")]
async fn join_team(
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
async fn create_team(
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
async fn leave_team(db: Data<Db>, user: User) -> Result<HttpResponse, Error> {
    db.leave_team(user)
        .await
        .map(|x| HttpResponse::Ok().json(x))
}

#[get("/{id}/members")]
async fn get_members(
    db: Data<Db>,
    user: User,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    db.get_team_members(user, id.into_inner())
        .await
        .map(|x| HttpResponse::Ok().json(x))
}
