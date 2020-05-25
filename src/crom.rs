use super::graphql::{search_query, SearchQuery};
use anyhow::{anyhow, bail, Result};
use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Page {
    pub url: Url,
    pub title: Option<String>,
}

pub fn search(query: impl AsRef<str>) -> Result<Option<Page>> {
    let query = query.as_ref().to_string();
    let variables = search_query::Variables { query };
    let body = SearchQuery::build_query(variables);
    let resp: Response<search_query::ResponseData> =
        attohttpc::post("https://api.crom.avn.sh/graphql")
            .json_streaming(body)
            .send()?
            .json()?;
    if let Some(errors) = resp.errors {
        bail!("GraphQL errors: {:#?}", errors);
    }
    let data = resp.data.ok_or_else(|| anyhow!("Missing data!"))?;
    let first = if let Some(x) = data.search_pages.into_iter().next() {
        x
    } else {
        return Ok(None);
    };
    let title = first.wikidot_info.and_then(|x| x.title);
    let url = first.url;
    Ok(Some(Page { url, title }))
}
