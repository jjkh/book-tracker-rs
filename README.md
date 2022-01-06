# book-tracker-rs
GraphQL backend for a personal read tracking webapp.

## Usage notes
When making changes to the SQL schema, make sure to run 

```bash
    # requires cargo install sqlx-cli
    sqlx migrate --source ./src/db/migrations run
```

This will update the dev SQLite instance to ensure the sqlx SQL checks at compile time will be run against the current schema.
