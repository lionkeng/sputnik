extern crate anyhow;
use std::env;

use async_graphql::{
  http::{playground_source, GraphQLPlaygroundConfig},
  EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_rocket::{Query, Request, Response};
use dotenv::dotenv;
use rocket::{response::content, routes, State};

mod db;
mod models;

use db::{Pool, PoolManager};

use models::{QueryRoot};

pub type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[rocket::get("/hello")]
async fn hello(_schema: &State<MySchema>) -> String {
  "🚀 says hello!".to_string()
}

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
    .mount(
      "/",
      routes![
        graphql_query,
        graphql_request,
        graphql_playground,
        hello
      ],
    )
    .launch()
    .await
    .unwrap();
}
