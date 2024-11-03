use std::{fmt, str::FromStr};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum RequestParameter {
    Url(String, Option<String>),
    Header(String, Option<String>),
    Body(String, Option<String>),
}

impl fmt::Display for RequestParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestParameter::Url(k, v) => write!(f, "URL({} : {})", k, v.clone().unwrap_or("None".to_owned())),
            RequestParameter::Header(k, v) => write!(f, "HEADER({} : {})", k, v.clone().unwrap_or("None".to_owned())),
            RequestParameter::Body(k,v ) => write!(f, "BODY({} : {})", k, v.clone().unwrap_or("None".to_owned())),
        }
    }
}

fn deserialization_helper(input : &str, param_name : &str) -> Result<(String, Option<String>), ()>{
    let mut tmp = input.replace(param_name, "");
    tmp = tmp.replace('(', "");
    tmp = tmp.replace(')', "");
    let mut items : Vec<&str> = tmp.split(':').collect();
    if items.len() != 2 { return Err(()); }
    items[0] = items[0].trim();
    items[1] = items[1].trim();

    if items[1] == "None" { return Ok((items[0].to_owned(), None)); }

    Ok((items[0].to_owned(), Some(items[1].to_owned())))
}

impl FromStr for RequestParameter {
    type Err = ();

    fn from_str(input: &str) -> Result<RequestParameter, Self::Err> {
        if input.starts_with("URL") {
            let (k, v) = deserialization_helper(input, "URL").unwrap();
            return Ok(RequestParameter::Url(k, v)); 
        }

        if input.starts_with("HEADER") {
            let (k, v) = deserialization_helper(input, "HEADER").unwrap();
            return Ok(RequestParameter::Header(k, v)); 
        }

        if input.starts_with("BODY") {
            let (k, v) = deserialization_helper(input, "BODY").unwrap();
            return Ok(RequestParameter::Body(k, v)); 
        }

        Err(())
    }
}

#[cfg(test)]
mod rockery_request_parameter_tests {
    use super::*;

    #[test]
    fn test_serialization() {
        let mut subject = RequestParameter::Url("API".to_owned(), None);
        assert_eq!(subject, RequestParameter::from_str(&subject.to_string()).unwrap());

        subject = RequestParameter::Url("API".to_owned(), Some("Something".to_owned()));
        assert_eq!(subject, RequestParameter::from_str(&subject.to_string()).unwrap());

        subject = RequestParameter::Header("API".to_owned(), None);
        assert_eq!(subject, RequestParameter::from_str(&subject.to_string()).unwrap());

        subject = RequestParameter::Header("API".to_owned(), Some("Something".to_owned()));
        assert_eq!(subject, RequestParameter::from_str(&subject.to_string()).unwrap());

        subject = RequestParameter::Body("API".to_owned(), None);
        assert_eq!(subject, RequestParameter::from_str(&subject.to_string()).unwrap());

        subject = RequestParameter::Body("API".to_owned(), Some("Something".to_owned()));
        assert_eq!(subject, RequestParameter::from_str(&subject.to_string()).unwrap());
    }
}