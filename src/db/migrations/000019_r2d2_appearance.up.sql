WITH episodes AS (
  SELECT unnest(enum_range(NULL::starwars.enum_episode)) as episode
),
MainGuy AS (
  SELECT id as character_id FROM starwars.characters WHERE name = 'R2-D2'
),
Results AS (
  SELECT MainGuy.character_id, episode FROM MainGuy
LEFT JOIN
 episodes 
ON episode IN ('A New Hope', 'Return of the Jedi', 'Empire Strikes Back', 'The Force Awakens', 'The Rise of Skywalker')
)
INSERT INTO starwars.episode_appearance (character_id, episode)
SELECT character_id, episode FROM Results
