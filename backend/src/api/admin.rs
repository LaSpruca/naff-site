use actix_web::{
    get,
    web::{self},
    HttpResponse, Scope,
};

use crate::{auth::AdminUser, db::Db, Error};

pub fn service() -> Scope {
    Scope::new("/admin").service(get_teams).service(hello_world)
}

#[get("/teams")]
async fn get_teams(db: web::Data<Db>, _: AdminUser) -> Result<HttpResponse, Error> {
    db.get_teams().await.map(|x| HttpResponse::Ok().json(x))
}

#[get("/")]
async fn hello_world() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

// #[post("/team/:id/project")]
// async fn get_team_info() -> Result<HttpResponse, Error> {
//     Ok(HttpResponse::Ok().finish())
// }
