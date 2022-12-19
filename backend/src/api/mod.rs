pub mod auth;
mod teams;

use crate::{auth::User, db::Db, Error};
use actix_web::{get, web, HttpResponse, Scope};

pub fn api() -> Scope {
    Scope::new("/api")
        .service(get_user)
        .service(get_team)
        .service(
            Scope::new("/team")
                .service(teams::join_team)
                .service(teams::create_team)
                .service(teams::get_members),
        )
}

#[get("/user")]
async fn get_user(db: web::Data<Db>, user: User) -> Result<HttpResponse, Error> {
    db.get_user(user).await.map(|x| HttpResponse::Ok().json(x))
}

#[get("/team")]
async fn get_team(db: web::Data<Db>, user: User) -> Result<HttpResponse, Error> {
    db.get_team(user).await.map(|x| HttpResponse::Ok().json(x))
}
