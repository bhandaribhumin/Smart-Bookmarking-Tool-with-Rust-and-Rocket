/*
First, it checks if the query string matches “tw” meaning the user entered “tw” with no other text.
If so it sends them to the Twitter homepage. Otherwise, it checks if the first 4 characters match “tw @”.
If it does, then it constructs a twitter profile url.
 The final alternative is taking the text and using it in a search query.
*/
pub fn construct_twitter_url(query: &str) -> String {
    if query == "tw" {
        let twitter_dotcom = "https://twitter.com";

        twitter_dotcom.to_string()

    // Check if it looks like a Twitter profile
    } else if &query[..4] == "tw @" {
        construct_twitter_profile_url(&query[4..])
    } else {
        // Assume the other match is "tw sometext"
        // and search on Twitter
        construct_twitter_search_url(&query[3..])
    }
}

pub fn construct_twitter_profile_url(profile: &str) -> String {
    // fill in logic
    format!("https://twitter.com/{}", profile)
}

pub fn construct_twitter_search_url(query: &str) -> String {
    // fill in logic
    String::from("Hello world")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_twitter_url() {
        let fake_query = "tw";
        assert_eq!(construct_twitter_url(fake_query), "https://twitter.com");
    }

    #[test]
    fn test_construct_twitter_url_query() {
        let fake_query = "tw hello world";
        assert_eq!(
            construct_twitter_url(fake_query),
            "https://twitter.com/search?q=hello%20world"
        );
    }

    #[test]
    fn test_construct_twitter_url_profile() {
        let fake_query = "tw @fbOpenSource";
        assert_eq!(
            construct_twitter_url(fake_query),
            "https://twitter.com/fbOpenSource"
        );
    }

    #[test]
    fn test_construct_twitter_profile_url() {
        let fake_profile = "jsjoeio";
        assert_eq!(
            construct_twitter_profile_url(fake_profile),
            "https://twitter.com/jsjoeio"
        );
    }

    #[test]
    fn test_construct_twitter_search_url() {
        let fake_query = "hello world";
        assert_eq!(
            construct_twitter_search_url(fake_query),
            "https://twitter.com/search?q=hello%20world"
        );
    }
}
