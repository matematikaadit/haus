use std::fmt;
use std::convert::AsRef;

use regex::Regex;
use rocket::http::RawStr;
use rocket::request::FromFormValue;

lazy_static! {
    static ref ALLOWED_URL: Regex = Regex::new("(?i)https?://").unwrap();
}

pub struct Url(String);

impl<'a> FromFormValue<'a> for Url {
    type Error = &'static str;

    fn from_form_value(form_value: &'a RawStr) -> Result<Url, &'static str> {
        match form_value.url_decode() {
            // Only allows http or https protocol
            Ok(url) =>
                if ALLOWED_URL.is_match(&url) {
                    Ok(Url(url))
                } else {
                    Err("url not allowed")
                }
            _ => Err("not valid utf-8 value")
        }
    }
}

#[derive(FromForm)]
pub struct Data {
    pub url: Url
}


impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}

impl AsRef<str> for Data {
    fn as_ref(&self) -> &str {
        self.url.as_ref()
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
