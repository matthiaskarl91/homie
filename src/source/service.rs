use diesel::{SqliteConnection, SelectableHelper, RunQueryDsl, QueryDsl};

use crate::source::model::NewSource;

use super::model::Source;

pub fn insert_new_source( conn: &mut SqliteConnection, name: &String, unit: &String) -> diesel::QueryResult<usize> {
    use crate::schema::source;

    let source = NewSource {
        name,
        unit
    };

    let result = diesel::insert_into(source::table)
        .values(&source)
        .execute(conn)
        .expect("Error inserting new source");

    Ok(result)
}

pub fn get_sources(conn: &mut SqliteConnection) -> diesel::QueryResult<Vec<Source>> {
    use crate::schema::source;
    let results = source::table.select(Source::as_select()).load(conn).expect("Error loading sources");

    Ok(results)
}
