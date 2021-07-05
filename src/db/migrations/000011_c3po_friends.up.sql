WITH MainGuy AS (
	SELECT id as character_id FROM starwars.characters WHERE name = 'C3-P0'
), 
Results AS (
SELECT MainGuy.character_id, id as friend_id FROM MainGuy
LEFT JOIN
starwars.characters 
ON name IN ('Princess Leia', 'Luke Skywalker', 'R2-D2')
)

INSERT INTO starwars.friends (character_id, friend_id)
SELECT character_id, friend_id FROM Results

