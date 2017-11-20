use std::fmt;
use std::borrow::Cow;

use rocket::request::FromParam;
use rocket::http::RawStr;
use rand::{self, Rng};

/// Table to retrieve base 36 values from.
const BASE36: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";

/// A _probably_ unique paste ID.
pub struct Id<'a>(Cow<'a, str>);

impl<'a> Id<'a> {
    /// Generate a _probably_ unique ID with `size` characters.
    /// For readibility, the character used are from sets [0-9], [a-z].
    /// Notice that we also allow underscore and dash, but not when creating
    /// new random id.
    pub fn new(size: usize) -> Id<'static> {
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE36[rng.gen::<usize>() % 36] as char);
        }

        Id(Cow::Owned(id))
    }
}

impl<'a> fmt::Display for Id<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Returns `true` if `id` is valid Id.
/// Allowed characters are lowercase alphanumeric, dash, or underscore.
fn valid_id(id: &str) -> bool {
    id.chars().all(|c| {
        (c >= 'a' && c <= 'z')
            || (c >= '0' && c <= '9')
            || (c == '-')
            || (c == '_')
    })
}

/// Returns an instance of `Id` if the path segment is a valid Id.
/// Otherwise returns the invalid Id as the `Err` value.
impl<'a> FromParam<'a> for Id<'a> {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<Id<'a>, &'a RawStr> {
        if valid_id(param) {
            Ok(Id(Cow::Borrowed(param)))
        } else {
            Err(param)
        }
    }
}
