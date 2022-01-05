#![feature(proc_macro_hygiene, decl_macro)]

mod db;
mod models;
mod openlibrary;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::{response::content, State};

use db::Db;
use models::{Mutation, QueryRoot};

#[macro_use]
extern crate rocket;

extern crate dotenv;

pub type BookSchema = Schema<QueryRoot, Mutation, EmptySubscription>;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[get("/playground")]
fn graphql_playground() -> content::Html<String> {
    content::Html(playground_source(GraphQLPlaygroundConfig::new("/graph")))
}

#[get("/graph?<query..>")]
async fn graphql_query(schema: &State<BookSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema).await
}

#[post("/graph", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<BookSchema>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema).await
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let connection_string =
        std::env::var("DATABASE_STRING").expect("Missing DB string in .env file");
    println!("DATABASE_STRING: {}", connection_string);
    let pool = Db::create_pool(&connection_string)
        .await
        .expect("Failed to open pool or migrate DB");

    let schema = Schema::build(QueryRoot, Mutation, EmptySubscription)
        .data(pool)
        .finish();

    rocket::build().manage(schema).mount(
        "/",
        routes![index, graphql_playground, graphql_query, graphql_request],
    )
}
