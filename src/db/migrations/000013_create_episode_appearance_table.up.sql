BEGIN;
CREATE TABLE IF NOT EXISTS starwars.episode_appearance(
   character_id INTEGER,
   episode starwars.enum_episode,
   PRIMARY KEY (character_id, episode)
);
COMMIT;

