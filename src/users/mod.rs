use diesel::{result::Error as DieselError, Queryable, RunQueryDsl};
use serde::Serialize;
use uuid::Uuid;

use crate::{common::Connection, schema::users};

mod get;
mod post;

pub use get::get_users;
pub use post::create_user;

use diesel::prelude::*;

#[derive(Queryable, Selectable, Serialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[serde(default)]
    pub id: Option<Uuid>,
    pub username: String,
    pub password_hash: String,
}

fn get_all(conn: &mut Connection) -> Result<Vec<User>, DieselError> {
    users::table.load::<User>(conn)
}
