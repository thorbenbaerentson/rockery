use std::fmt;
use serde::{Deserialize, Serialize};

use crate::prelude::{ RequestMethod, RequestParameter };

/// Each service consists of several service method. 
/// Each method represents a single request against a service, that can be parameterized.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RockeryServiceMethod {
    name : String,
    method_uri : String,
    request_method : RequestMethod,

    /// This is a list of all parameters, and optionally a default value for each parameter.
    /// If a default value is set and a request is made without providing a value for that parameter, then
    /// the default value is used.
    /// If the parameter should not be used at all, provide None as default for it and 
    /// provide None for it, when making a request.
    parameter : Vec<RequestParameter>,

    body : Option<String>,
}

impl RockeryServiceMethod {
    pub fn new(name : &str, uri : &str) -> Self {
        RockeryServiceMethod {
            name : name.to_owned(),
            method_uri : uri.to_owned(),
            request_method : RequestMethod::Get,
            parameter : Vec::new(),
            body : None,
        }
    }

    pub fn get_uri(&self) -> &str { &self.method_uri }
    pub fn set_uri(&mut self, uri : &str) { self.method_uri = uri.to_owned(); }

    pub fn get_name(&self) -> &str { &self.name }
    pub fn set_name(&mut self, name : &str) { self.name = name.to_owned(); }

    pub fn add_parameter(&mut self, param : RequestParameter) {
        self.parameter.push(param);
    }

    pub fn get_parameter(&self) -> &Vec<RequestParameter> {
        &self.parameter
    }

    pub fn get_request_method(&self) -> RequestMethod {
        self.request_method
    }

    pub fn set_request_method(&mut self, method : RequestMethod) {
        self.request_method = method;
    }

    pub fn get_body(&self) -> &Option<String> { &self.body }
    pub fn set_body(&mut self, body : &str) { self.body = Some(body.to_owned()); }

}

impl fmt::Display for RockeryServiceMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RockeryServiceMethod")
            .field("method_uri", &self.get_uri())
            .field("request_method", &self.get_request_method())
            // .field("base_url", &self.get_parameter().iter().map(|p|))
            .finish()
    }
}

#[cfg(test)]
mod rockery_service_method_tests {
    use super::*;

    #[test]
    fn getters_and_setters() {
        let mut subject = RockeryServiceMethod::new("test","api");
        assert_eq!(subject.get_uri(), "api");
        assert_eq!(subject.get_request_method(), RequestMethod::Get);
        assert_eq!(subject.get_parameter().len(), 0);

        subject.set_request_method(RequestMethod::Connect);
        assert_eq!(subject.get_request_method(), RequestMethod::Connect);

        subject.set_uri("aci");
        assert_eq!(subject.get_uri(), "aci");

        subject.add_parameter(RequestParameter::Header("API-KEY".to_owned(), Some("Test".to_owned())));
        assert_eq!(subject.get_parameter().len(), 1);

        for p in subject.get_parameter() {
            match p {
                RequestParameter::Header(k, v) => { 
                    assert_eq!("API-KEY".to_owned(), *k);
                    assert_eq!(Some("Test".to_owned()), *v);
                },
                _ => unreachable!(),
            }
        }
    }


}


