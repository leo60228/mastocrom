use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Page {
    pub url: Url,
    pub title: Option<String>,
    #[serde(default)]
    pub scp_title: Option<String>,
}
