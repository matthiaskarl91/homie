use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::schema::source;


#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = source)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Source {
    pub source_id: i32,
    pub name: String,
    pub unit: String,
}

#[derive(Insertable)]
#[diesel(table_name = source)]
pub struct NewSource<'a> {
    pub name: &'a String,
    pub unit: &'a String,
}
