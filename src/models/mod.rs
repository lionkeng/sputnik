use std::ops::DerefMut;

use crate::db::Pool;
use async_graphql::*;
use serde::{Deserialize, Serialize};
use sqlx::{self, Row, postgres::PgRow};
use strum_macros::{Display, EnumString};
pub struct QueryRoot;

#[derive(
  Clone, Copy, Debug, Deserialize, Display, Enum, Eq, EnumString, PartialEq, Serialize, sqlx::Type,
)]
#[sqlx(type_name = "enum_character_kind")]
pub enum CharacterKind {
  Human,
  Droid,
  Wookie,
}
#[derive(SimpleObject, sqlx::FromRow)]
#[graphql(complex)]
#[derive(Debug)]
struct Character {
  id: i32,
  name: String,
  kind: CharacterKind,
}

#[ComplexObject]
impl Character {
  pub async fn friends(&self, ctx: &Context<'_>) -> FieldResult<Vec<Character>> {
    let pool = ctx.data::<Pool>().unwrap();
    let query_str = format!(
      r#"SELECT id, name, kind FROM starwars.characters
      JOIN starwars.friends ON starwars.friends.friend_id = id AND
      starwars.friends.character_id = {}"#,
      self.id
    );
    let result = sqlx::query_as::<_, Character>(query_str.as_str())
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
    let query_str = format!("SELECT id, name, kind FROM starwars.characters");
    let result = sqlx::query(query_str.as_str())
      .map(|row: PgRow| Character {
        id: row.get("id"),
        name: row.get("name"),
        kind: row.get("kind"),
      })
      .fetch_all(pool.get().await.unwrap().deref_mut())
      .await
      .unwrap();
    return Ok(result);
  }
}
