use actix_web::{get, web::Data, HttpResponse};

use diesel::result::Error as DieselError;

use crate::{
    common::{get_conn, DbPool},
    sync,
    users::get_all,
};

#[get("/users")]
async fn get_users(pool: Data<DbPool>) -> HttpResponse {
    match sync!(move || {
        let mut conn = get_conn(&pool)?;
        get_all(&mut conn)
    }) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(DieselError::NotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
