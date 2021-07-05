use std::ops::DerefMut;

use crate::db::Pool;
use async_graphql::*;
use serde::{Deserialize, Serialize};
use sqlx;
use strum_macros::{Display, EnumString};
pub struct QueryRoot;

#[derive(
  Clone, Copy, Debug, Deserialize, Display, Enum, Eq, EnumString, PartialEq, Serialize, sqlx::Type,
)]
#[sqlx(type_name = "starwars.enum_character_kind")]
pub enum CharacterKind {
  Human,
  Droid,
  Wookie,
}

impl Default for CharacterKind {
  fn default() -> Self {
    CharacterKind::Human
  }
}

#[derive(SimpleObject, sqlx::FromRow, Debug)]
#[graphql(complex)]
pub struct Friend {
  pub id: i32,
  pub name: String,
}

#[ComplexObject]
impl Friend {
  pub async fn kind(&self, ctx: &Context<'_>) -> FieldResult<std::option::Option<CharacterKind>> {
    let pool = ctx.data::<Pool>().unwrap();
    let result = sqlx::query_as!(
      Character,
      r#"SELECT id, name, kind as "kind: CharacterKind" FROM starwars.characters"#
    )
    .fetch_all(pool.get().await.unwrap().deref_mut())
    .await
    .unwrap();
    for character in result {
      if character.id == self.id {
        return Ok(character.kind)
      }
    }
    return Ok(None)
  }
}

/*
impl Default for Friend {
  fn default() -> Self {
    Friend {
      id: 0,
      name: "nobody".to_string(),
      kind: None,
    }
  }
}
*/

#[derive(SimpleObject, sqlx::FromRow)]
#[graphql(complex)]
#[derive(Debug)]
struct Character {
  id: i32,
  name: String,
  kind: std::option::Option<CharacterKind>,
}

#[ComplexObject]
impl Character {
  pub async fn friends(&self, ctx: &Context<'_>) -> FieldResult<Vec<Friend>> {
    let pool = ctx.data::<Pool>().unwrap();
    let query_str = format!(
      r#"SELECT id, name FROM starwars.characters
      JOIN starwars.friends ON starwars.friends.friend_id = id AND
      starwars.friends.character_id = {}"#,
      self.id
    );
    let result = sqlx::query_as::<_, Friend>(query_str.as_str())
      .fetch_all(pool.get().await.unwrap().deref_mut())
      .await
      .unwrap();
    println!("{:?}", result);
    return Ok(result);
  }
}

#[Object]
impl QueryRoot {
  async fn hello(&self, _ctx: &Context<'_>) -> String {
    "GraphQL says hello!".to_string()
  }

  async fn characters(&self, ctx: &Context<'_>) -> FieldResult<Vec<Character>> {
    let pool = ctx.data::<Pool>().unwrap();
    let result = sqlx::query_as!(
      Character,
      r#"SELECT id, name, kind as "kind: CharacterKind" FROM starwars.characters"#
    )
    .fetch_all(pool.get().await.unwrap().deref_mut())
    .await
    .unwrap();
    return Ok(result);
  }
}
