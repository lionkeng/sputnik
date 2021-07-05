WITH MainGuy AS (
	SELECT id as character_id FROM starwars.characters WHERE name = 'Luke Skywalker'
), 
Results AS (
SELECT MainGuy.character_id, id as friend_id FROM MainGuy
LEFT JOIN
starwars.characters 
ON name IN ('Princess Leia', 'Han Solo', 'Ben Kenobi')
)

INSERT INTO starwars.friends (character_id, friend_id)
SELECT character_id, friend_id FROM Results

