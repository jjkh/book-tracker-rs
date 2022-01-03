DROP TABLE IF EXISTS [books];
CREATE TABLE [books] 
(
    [id]              INTEGER PRIMARY KEY AUTOINCREMENT
  , [title]           TEXT
  , [author]          TEXT
  , [book_details_id] INTEGER
);

DROP TABLE IF EXISTS [book_details];
CREATE TABLE [book_details] 
(
    [id]              INTEGER PRIMARY KEY AUTOINCREMENT
  , [open_library_id] INTEGER NOT NULL UNIQUE
  , [isbn]            INTEGER
  , [title]           TEXT
  , [author]          TEXT
  , [author_key]      TEXT
  , [publish_date]    DATETIME
  , [last_updated]    DATETIME
  , [page_count]      INTEGER
);