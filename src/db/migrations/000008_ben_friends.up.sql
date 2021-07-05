WITH MainGuy AS (
	SELECT id as character_id FROM starwars.characters WHERE name = 'Ben Kenobi'
), 
Results AS (
SELECT MainGuy.character_id, id as friend_id FROM MainGuy
LEFT JOIN
starwars.characters 
ON name IN ('Princess Leia', 'Luke Skywalker')
)

INSERT INTO starwars.friends (character_id, friend_id)
SELECT character_id, friend_id FROM Results

