# sputnik

# Database Setup

1.  This project assumes that you have `postgres` installed locally. At this point, you will need to create a `starwars` database.

    ```psql -h localhost -U postgres -w -c "create database starwars;"```


2.  Create a `.env` file in the project root directory and add the following line:

    ```DATABASE_URL="postgres://postgres:<YOUR_PWD>@localhost:5432/starwars"```

    This environment variable is used as a configuration input to the database connection pool to indicate where to find the database

# Seeding your database with migrations

    Migration tables have been created to seed the `starwars` schema in your database.

1.  To use the migration scripts, install the `golang-migrate` CLI.

    Go [here](https://github.com/golang-migrate/migrate) and follow instructions to install the CLI.

2.  After installation, set this up for `golang-migrate`:

    ```export POSTGRESQL_URL='postgres://postgres:<YOUR_PWD>@localhost:5432/starwars?sslmode=disable'```

3.  When you are ready to run the migration, run this from the project's root directory:

    ```migrate -source file://./src/db/migrations -database postgres://postgres:<YOUR_PWD>@localhost:5432/starwars?sslmode=disable up```

    *Note: If you run into migration quirks, read the documentation at [golang-migrate](https://github.com/golang-migrate/migrate)*

## Extending the database

    This section can be skipped if you are not interested in extending the database.

1.  If you want to extend the tables, you can create new migration scripts,

```migrate create -ext sql -dir src/db/migrations -seq <MIGRATION_FILE_NAME>```

2.  If you want to update the scripts only, you will have to do a `down`, followed by an `up`

    ```migrate -source file://./src/db/migrations -database postgres://postgres:<YOUR_PWD>@localhost:5432/starwars?sslmode=disable down```

    ```migrate -source file://./src/db/migrations -database postgres://postgres:<YOUR_PWD>@localhost:5432/starwars?sslmode=disable up```

    *Note: If you encounter errors in your scripts, you will need to truncate the `scheme_migrations` table, and drop the `starwars` schema.*

