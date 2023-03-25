mod admin;
pub mod auth;
mod teams;

use crate::{auth::User, db::Db, Error};
use actix_web::{get, web, HttpResponse, Scope};

pub fn api() -> Scope {
    Scope::new("/api")
        .service(admin::service())
        .service(get_user)
        .service(teams::service())
}

#[get("/user")]
async fn get_user(db: web::Data<Db>, user: User) -> Result<HttpResponse, Error> {
    db.get_user(user).await.map(|x| HttpResponse::Ok().json(x))
}
