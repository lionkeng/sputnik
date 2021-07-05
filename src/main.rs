extern crate anyhow;
use std::{env, ops::DerefMut};

use async_graphql::{
  http::{playground_source, GraphQLPlaygroundConfig},
  EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_rocket::{Query, Request, Response};
use dotenv::dotenv;
use rocket::{response::content, routes, State};

// use starwars::{QueryRoot, StarWars};
mod db;
mod models;

use db::{Pool, PoolManager};

use models::{CharacterKind, QueryRoot, Friend};
use sqlx::{Row, postgres::PgRow};

pub type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[rocket::get("/hello")]
async fn hello(_schema: &State<MySchema>) -> String {
  "🚀 says hello!".to_string()
}

/*
#[rocket::get("/friends")]
async fn friends(_schema: &State<MySchema>, pool: &State<Pool>) -> String {
    let query_str = format!(
      r#"SELECT id, name FROM starwars.characters
      JOIN starwars.friends ON starwars.friends.friend_id = id AND
      starwars.friends.character_id = {}"#,
      1
    );
    let friends = sqlx::query_as::<_, Friend>(query_str.as_str())
    .fetch_all(pool.get().await.unwrap().deref_mut())
    .await
    .unwrap();
  let results = format!("{:?}", friends);
  return results;
  // println!("{:?}", friends);
  // "hello friends!".to_string()
}
*/

#[rocket::get("/")]
fn graphql_playground() -> content::Html<String> {
  content::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<MySchema>, query: Query) -> Response {
  query.execute(schema).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<MySchema>, request: Request) -> Response {
  request.execute(schema).await
}

#[rocket::main]
async fn main() {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
  let mgr = PoolManager { url: database_url };
  let db_pool = Pool::new(mgr, 16);
  let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
    .data(db_pool)
    .finish();
  rocket::build()
    .manage(schema)
 //   .manage(db_pool)
    .mount(
      "/",
      routes![
        graphql_query,
        graphql_request,
        graphql_playground,
        hello,
      ],
    )
    //    .attach(db::stage_database())
    .launch()
    .await
    .unwrap();
}
