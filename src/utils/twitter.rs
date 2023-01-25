extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Its like importing whitespace and other signs from percent_encoding crate
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_tw_url(query: &str) -> String {
    if query == "tw" {
        let main_page = "https://twitter.com";
        main_page.to_string()
    } else if &query[..4] == "tw @" {
        construct_tw_profile_url(&query[4..])
    } else {
        construct_tw_search_url(&query[3..])
    }
}

pub fn construct_tw_profile_url(profile: &str) -> String {
    format!("https://twitter.com/{}", profile)
}

pub fn construct_tw_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let twitter_search_url = format!("https://twitter.com/search?q={}", encoded_query);

    twitter_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_tw_url() {
        let demo = "tw";
        assert_eq!(construct_tw_url(demo), "https://twitter.com");
    }

    #[test]
    fn test_construct_tw_url_query() {
        let demo = "tw hello world";
        assert_eq!(
            construct_tw_url(demo),
            "https://twitter.com/search?q=hello%20world"
        );
    }

    #[test]
    fn test_construct_tw_url_profile() {
        let demo = "tw @tomarm698";
        assert_eq!(construct_tw_url(demo), "https://twitter.com/tomarm698");
    }

    #[test]
    fn test_construct_tw_profile_url() {
        let demo = "tomarm698";
        assert_eq!(
            construct_tw_profile_url(demo),
            "https://twitter.com/tomarm698"
        )
    }

    #[test]
    fn test_construct_tw_search_url() {
        let demo = "hello world";
        assert_eq!(
            construct_tw_search_url(demo),
            "https://twitter.com/search?q=hello%20world"
        )
    }
}
