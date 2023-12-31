CREATE TABLE time_series (
  series_id INTEGER PRIMARY KEY ASC NOT NULL
  ,timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
  ,value INTEGER NOT NULL
  ,source_id INTEGER NOT NULL
  ,FOREIGN KEY (source_id) REFERENCES source(source_id)
);
