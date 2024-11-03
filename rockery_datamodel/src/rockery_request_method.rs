use core::fmt;

use serde::{Deserialize, Serialize};
use std::str::FromStr;

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum RequestMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestMethod::Get => write!(f, "GET"),
            RequestMethod::Head => write!(f, "HEAD"),
            RequestMethod::Post => write!(f, "POST"),
            RequestMethod::Put => write!(f, "PUT"),
            RequestMethod::Delete => write!(f, "DELETE"),
            RequestMethod::Connect => write!(f, "CONNECT"),
            RequestMethod::Options => write!(f, "OPTIONS"),
            RequestMethod::Trace => write!(f, "TRACE"),
            RequestMethod::Patch => write!(f, "PATCH"),
        }
    }
}

impl FromStr for RequestMethod {
    type Err = ();

    fn from_str(input: &str) -> Result<RequestMethod, Self::Err> {
        match input {
            "GET"  => Ok(RequestMethod::Get),
            "HEAD"  => Ok(RequestMethod::Head),
            "POST"  => Ok(RequestMethod::Post),
            "PUT" => Ok(RequestMethod::Put),
            "DELETE" => Ok(RequestMethod::Delete),
            "CONNECT" => Ok(RequestMethod::Connect),
            "OPTIONS" => Ok(RequestMethod::Options),
            "TRACE" => Ok(RequestMethod::Trace),
            "PATCH" => Ok(RequestMethod::Patch),
            _      => Err(()),
        }
    }
}

#[cfg(test)]
mod rockery_request_method_tests {
    use super::*;

    #[test]
    fn serialization_tests() {
        assert_eq!(RequestMethod::Get.to_string(), RequestMethod::from_str("GET").unwrap().to_string());
        assert_eq!(RequestMethod::Head.to_string(), RequestMethod::from_str("HEAD").unwrap().to_string());
        assert_eq!(RequestMethod::Post.to_string(), RequestMethod::from_str("POST").unwrap().to_string());
        assert_eq!(RequestMethod::Put.to_string(), RequestMethod::from_str("PUT").unwrap().to_string());
        assert_eq!(RequestMethod::Delete.to_string(), RequestMethod::from_str("DELETE").unwrap().to_string());
        assert_eq!(RequestMethod::Connect.to_string(), RequestMethod::from_str("CONNECT").unwrap().to_string());
        assert_eq!(RequestMethod::Options.to_string(), RequestMethod::from_str("OPTIONS").unwrap().to_string());
        assert_eq!(RequestMethod::Trace.to_string(), RequestMethod::from_str("TRACE").unwrap().to_string());
        assert_eq!(RequestMethod::Patch.to_string(), RequestMethod::from_str("PATCH").unwrap().to_string());
    }
}
