// @generated automatically by Diesel CLI.

diesel::table! {
    source (source_id) {
        source_id -> Nullable<Integer>,
        name -> Nullable<Text>,
        unit -> Nullable<Text>,
    }
}

diesel::table! {
    time_series (series_id) {
        series_id -> Integer,
        timestamp -> Timestamp,
        value1 -> Integer,
        source_id -> Integer,
    }
}

diesel::joinable!(time_series -> source (source_id));

diesel::allow_tables_to_appear_in_same_query!(
    source,
    time_series,
);
