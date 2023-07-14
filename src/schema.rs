// @generated automatically by Diesel CLI.

diesel::table! {
    source (source_id) {
        source_id -> Integer,
        name -> Text,
        unit -> Text,
    }
}

diesel::table! {
    time_series (series_id) {
        series_id -> Integer,
        timestamp -> Timestamp,
        value -> Integer,
        source_id -> Integer,
    }
}

diesel::joinable!(time_series -> source (source_id));

diesel::allow_tables_to_appear_in_same_query!(
    source,
    time_series,
);
