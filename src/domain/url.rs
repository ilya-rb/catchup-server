use regex::Regex;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Url(pub String);

const URL_REGEX: &str =
    "[(http(s)?)://(www.)?a-zA-Z0-9@:%._+~#=]{2,256}.[a-z]{2,6}b([-a-zA-Z0-9@:%_+.~#?&//=]*)";

impl Url {
    pub fn new(value: String) -> Self {
        let re = Regex::new(URL_REGEX).expect("Failed to compile URL regex");
        let matches = re.captures(value.as_str());
        assert!(
            matches.is_some(),
            "{}",
            format!("Invalid URL {}", value.as_str())
        );

        Url(value)
    }
}
