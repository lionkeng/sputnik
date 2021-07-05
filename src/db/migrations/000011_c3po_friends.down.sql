WITH MainGuy AS (
  SELECT id as character_id FROM starwars.characters WHERE name = 'C3-P0'
)
DELETE FROM starwars.friends WHERE character_id = (SELECT character_id FROM MainGuy)
