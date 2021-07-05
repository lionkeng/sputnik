WITH MainGuy AS (
  SELECT id as character_id FROM starwars.characters WHERE name = 'Han Solo'
)
DELETE FROM starwars.episode_appearance WHERE character_id = (SELECT character_id FROM MainGuy)
