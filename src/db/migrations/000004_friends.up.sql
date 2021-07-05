CREATE TABLE IF NOT EXISTS starwars.friends(
   character_id INTEGER,
   friend_id INTEGER,
   PRIMARY KEY (character_id, friend_id)
);