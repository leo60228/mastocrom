use super::types::Page;
use anyhow::Result;

pub fn search(query: impl AsRef<str>) -> Result<Option<Page>> {
    let query = query.as_ref();

    let is_scp = query.starts_with("scp-") && query.chars().skip(4).all(|c| c.is_ascii_digit());
    let query = if is_scp { &query[4..] } else { query };

    Ok(attohttpc::get("https://crom-dev.avn.sh/search")
        .param("q", query)
        .send()?
        .json()
        .ok())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scp001() {
        assert_eq!(
            search("001").unwrap().unwrap().url,
            "http://www.scp-wiki.net/scp-001".parse().unwrap()
        );
        assert_eq!(
            search("scp-001").unwrap().unwrap().url,
            "http://www.scp-wiki.net/scp-001".parse().unwrap()
        );
    }

    #[test]
    fn unthreaded() {
        assert_eq!(
            search("unthreaded").unwrap().unwrap().url,
            "http://www.scp-wiki.net/unthreaded".parse().unwrap()
        );
    }

    #[test]
    fn nonexistent() {
        assert!(search("asdfasufausdhfaksjfh").unwrap().is_none());
    }

    #[test]
    fn scp033() {
        let (scp0, scp1, scp2) = (search("033"), search("scp-033"), search("scp-scp-033"));
        assert_eq!(
            scp0.unwrap().unwrap().url,
            "http://www.scp-wiki.net/scp-033".parse().unwrap()
        );
        assert_eq!(
            scp1.unwrap().unwrap().url,
            "http://www.scp-wiki.net/scp-033".parse().unwrap()
        );
        assert_eq!(
            scp2.unwrap().unwrap().url,
            "http://www.scp-wiki.net/scp-scp-033".parse().unwrap()
        );
        // edge case; scp-033 and scp-scp-033 are both articles
    }
}
