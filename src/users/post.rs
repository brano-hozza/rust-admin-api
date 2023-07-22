use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel::RunQueryDsl;
use serde::Deserialize;
use serde_valid::Validate;

use crate::{
    common::{get_conn, DbPool},
    schema::users,
    sync,
    users::User,
};

#[derive(Deserialize, Validate)]
struct NewUserRequest {
    username: String,
    password: String,
}

#[post("/user")]
async fn create_user(user: Json<NewUserRequest>, pool: Data<DbPool>) -> HttpResponse {
    match sync!(move || {
        let new_user = User {
            id: None,
            password_hash: user.password.clone(),
            username: user.username.clone(),
        };
        let mut conn = get_conn(&pool)?;
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(&mut conn)
    }) {
        Ok(user) => HttpResponse::Created().json(user),
        Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            HttpResponse::Conflict().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
