use chrono::NaiveDateTime;
use diesel::{SqliteConnection, RunQueryDsl, SelectableHelper, QueryDsl};

use crate::power::model::{NewPower, Power};

pub fn insert_new_power(
    conn: &mut SqliteConnection,
    value: &i32,
    timestamp: &NaiveDateTime,
    source_id: &i32
) -> diesel::QueryResult<usize> {
    use crate::schema::time_series;

    let new_power = NewPower {
        value,
        source_id,
        timestamp
    };

    let result = diesel::insert_into(time_series::table).values(&new_power).execute(conn).expect("bla");
    Ok(result)
}

pub fn get_power(conn: &mut SqliteConnection) -> diesel::QueryResult<Vec<Power>> {
    use crate::schema::time_series;

   let results = time_series::table.select(Power::as_select()).load(conn).expect("Error loading power data");

   Ok(results)
}
