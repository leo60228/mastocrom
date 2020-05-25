use graphql_client::GraphQLQuery;
use url::Url;

pub type URL = Url;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/search_query.graphql",
    response_derives = "Debug"
)]
pub struct SearchQuery;
