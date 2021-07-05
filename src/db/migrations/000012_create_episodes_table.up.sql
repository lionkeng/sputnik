BEGIN;

CREATE TYPE starwars.enum_episode AS ENUM (
  'A New Hope',
  'Empire Strikes Back',
  'Return of the Jedi',
  'The Force Awakens',
  'The Rise of Skywalker'
);

COMMIT;