use chrono::Utc;
use diesel::{SqliteConnection, RunQueryDsl, SelectableHelper, QueryResult, QueryDsl};

use crate::power::model::{NewPower, Power};

pub fn insert_new_power(
    conn: &mut SqliteConnection,
    value: i32
) -> usize {
    use crate::schema::time_series;

    let new_power = NewPower {
        value1: &value,
        source_id: &0,
        timestamp: &Utc::now().naive_utc()
    };

    //diesel::insert_into(time_series::table).values(&new_power).returning(Power::as_select())
    //    .get_result(conn).expect("Error inserting power")
    diesel::insert_into(time_series::table).values(&new_power).execute(conn).expect("bla")

    //let power = time_series::table.first::<Power>(conn).expect("hi");

    //Ok(power)
}

pub fn get_power(conn: &mut SqliteConnection) -> diesel::QueryResult<Vec<Power>> {
    use crate::schema::time_series;

   let results = time_series::table.select(Power::as_select()).load(conn).expect("Error loading power data");

   Ok(results)
}
