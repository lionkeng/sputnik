BEGIN;
CREATE SCHEMA starwars;
CREATE TABLE IF NOT EXISTS starwars.characters(
   id serial PRIMARY KEY,
   name VARCHAR (50) UNIQUE NOT NULL
);
COMMIT;

