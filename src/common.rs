use chrono::NaiveDateTime;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    result::{DatabaseErrorKind, Error as DieselError},
    PgConnection,
};

use serde::{Deserialize, Deserializer, Serializer};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn deserialize_omittable<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

#[macro_export]
macro_rules! sync {
    ($e:expr) => {
        match actix_web::web::block($e).await {
            Ok(r) => r,
            Err(_) => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    };
}

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_conn(pool: &DbPool) -> Result<Connection, DieselError> {
    pool.get().map_err(|e| {
        DieselError::DatabaseError(DatabaseErrorKind::Unknown, Box::new(e.to_string()))
    })
}

pub fn serialize_datetime<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    serializer.serialize_str(&s)
}
