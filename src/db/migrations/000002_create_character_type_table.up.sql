BEGIN;

CREATE TYPE starwars.enum_character_kind AS ENUM (
  'Droid',
  'Human',
  'Wookie'
);

ALTER TABLE starwars.characters ADD COLUMN kind starwars.enum_character_kind;

COMMIT;