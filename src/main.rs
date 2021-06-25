use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "mutation.graphql"
)]
pub struct InsertData;

fn main() {
    println!("Hello, world!");
}
