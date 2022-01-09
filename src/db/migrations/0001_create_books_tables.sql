DROP TABLE IF EXISTS [books];
CREATE TABLE [books] 
(
    [id]              INTEGER PRIMARY KEY AUTOINCREMENT
  , [title]           TEXT
  , [author]          TEXT
  , [book_details_id] INTEGER REFERENCES book_details(id)
);

DROP TABLE IF EXISTS [book_details];
CREATE TABLE [book_details] 
(
    [id]              INTEGER PRIMARY KEY AUTOINCREMENT
  , [open_library_id] TEXT NOT NULL UNIQUE
  , [cover_id]        INTEGER
  , [isbn]            TEXT
  , [title]           TEXT
  , [author]          TEXT
  , [author_key]      TEXT
  , [publish_year]    INTEGER
  , [last_updated]    DATETIME
  , [page_count]      INTEGER
);