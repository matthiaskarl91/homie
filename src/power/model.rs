use chrono::NaiveDateTime;
use diesel::{prelude::*, sql_types::Timestamp};
use crate::schema::time_series;
use serde::{Deserialize,Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = time_series)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Power {
    pub series_id: i32,
    pub value: i32, 
    pub timestamp: NaiveDateTime,
    pub source_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = time_series)]
pub struct NewPower<'a> {
    pub value: &'a i32,
    pub timestamp: &'a NaiveDateTime,
    pub source_id: &'a i32,
}
