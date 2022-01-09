#![feature(proc_macro_hygiene, decl_macro)]

mod cors;
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

#[macro_use]
extern crate dotenv_codegen;

pub type BookSchema = Schema<QueryRoot, Mutation, EmptySubscription>;

#[options("/graph")]
fn index() -> &'static str {
    "Hello world!"
}

#[get("/pg")]
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
    dotenv!("DATABASE_URL");

    let connection_string = dotenv::var("DATABASE_URL")
        .expect("Missing DATABASE_URL in .env file or environment variables");

    let pool = Db::create_pool(&connection_string)
        .await
        .expect("Failed to open pool or migrate DB");

    let schema = Schema::build(QueryRoot, Mutation, EmptySubscription)
        .data(pool)
        .finish();

    rocket::build().manage(schema).attach(cors::Cors).mount(
        "/",
        routes![index, graphql_playground, graphql_query, graphql_request],
    )
}
