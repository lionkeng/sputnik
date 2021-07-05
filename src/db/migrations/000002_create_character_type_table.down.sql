BEGIN;

ALTER TABLE starwars.characters DROP COLUMN kind;
DROP TYPE starwars.enum_character_kind;

COMMIT;