use std::{collections::HashMap, fmt};
use ehttp::Headers;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct RockeryService
{
    name : String,
    service_type : RockeryServiceType,
    authentication : AuthenticationType,

    base_url : String,
    definition_url : Option<String>, // Url or path to wsdl or wadl file

    methods : Vec<RockeryServiceMethod>,
}

impl RockeryService {
    pub fn new(t : RockeryServiceType, base_url : &str, ) -> Self {
        RockeryService {
            name : "Unnamed".to_owned(),
            service_type : t,
            authentication : AuthenticationType::None(NoAuthenticationAuthentication {}),
            base_url : base_url.to_owned(),
            definition_url : None,
            methods : Vec::new(),
        }
    }

    pub fn get_name(&self) -> &String { &self.name }
    pub fn set_name(&mut self, t : &str) { self.name = t.to_owned(); }

    pub fn get_service_type(&self) -> &RockeryServiceType { &self.service_type }
    pub fn set_service_type(&mut self, t : RockeryServiceType) { self.service_type = t; }

    pub fn set_authentication(&mut self, t : AuthenticationType) { self.authentication = t; }

    /// Add another metho to the list of methods and return the newly created item
    /// so it can be configured.
    pub fn add_method(&mut self, name : &str, uri : &str) -> &mut RockeryServiceMethod {
        self.methods.push(RockeryServiceMethod::new(name, uri));
        let index = self.methods.len() - 1;

        &mut self.methods[index]
    }

    pub fn get_request_url(&self, method : usize, params : &[RequestParameter]) -> String {
        let m = &self.methods[method];
        let mut url = format!("{}/{}/", self.base_url, m.get_uri());
        let mut first = true;

        // Prepare a list of url parameters to check if we got default values.
        let url_params : HashMap<String, Option<String>> = 
            params.iter().filter_map(|f| {
                match f {
                    RequestParameter::Url(k, v) => {
                        if v.is_none() { return None; }

                        Some((k.to_owned(), v.clone()))
                    },
                    _ => None
                }
            }).collect();

        for p in m.get_parameter() {
            if let RequestParameter::Url(k, v) = p {
                if v.is_none() && !url_params.contains_key(k) { 
                    continue; 
                }

                if first { 
                    url.push('?');
                    first = false;
                } else {
                    url.push('&');
                }

                if url_params.contains_key(k) {
                    url.push_str(format!("{}={}", k, url_params[k].as_ref().unwrap()).as_str());
                    continue;
                }

                url.push_str(format!("{}={}", k, v.as_ref().unwrap()).as_str());
            }
        }

        url
    }

    fn get_request_header(&self, method : usize, params : &[RequestParameter]) -> Headers {
        let header_params : HashMap<String, Option<String>> = 
            params.iter().filter_map(|f| {
                match f {
                    RequestParameter::Header(k, v) => {
                        if v.is_none() { return None; }
                        Some((k.to_owned(), v.clone()))
                    },
                    _ => None
                }
            }).collect();
        
        let m = &self.methods[method];
        let mut headers = Headers::new(&[]);
        for h in m.get_parameter() {
            if let RequestParameter::Header(k, v) = h {
                if v.is_none() && !header_params.contains_key(k) {
                    continue;
                }

                if header_params.contains_key(k) {
                    headers.insert(k.to_owned(), header_params[k].clone().unwrap().to_owned());
                    continue;
                }

                headers.insert(k.to_owned(), v.clone().unwrap().to_owned());
            }
        }

        headers
    }

    /// Do a request for the method identified by the provided index.
    pub fn do_request(&self, method : usize, params : &[RequestParameter]) -> Result<(u16, String, Headers, Vec<u8>), String> {
        let status : u16;
        let status_text : String;
        let headers : Headers;
        let bytes : Vec<u8>;

        if method >= self.methods.len() {
            let message = format!("Invalid index. There are only {} methods but method {} was requested.", self.methods.len(), method);
           return Err(message);
        }

        let url = self.get_request_url(method, params);

        let mut request = ehttp::Request::get(url);
        request.method = self.methods[method].get_request_method().to_string();
        request.url = self.get_request_url(method, params);
        request.body = vec![];
        request.headers = self.get_request_header(method, params);

        let r = self.authentication.do_request(&mut request);

        match r {
            Ok(res) => {
                status = res.status;
                status_text = res.status_text;
                headers = res.headers;
                bytes = res.bytes;
            },
            Err(e) => {
                let message = format!("Could not execute request. Error: {}", e);
                return Err(message);
            },
        }

        Ok((status, status_text, headers, bytes))
    }
}

impl fmt::Debug for RockeryService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RockeryService")
            .field("service_type", &self.service_type)
            .field("authentication", &self.authentication)
            .field("base_url", &self.base_url)
            .finish()
    }
}

#[cfg(test)]
mod rockery_service_test {
    use crate::rockery_test_helper::get_feiertage_service;

    use super::*;

    #[test]
    fn test_url_logic() {
        let service = get_feiertage_service();

        let mut url = service.get_request_url(0, &Vec::new());
        assert_eq!("https://feiertage-api.de/api/?jahr=2024", url);

        let mut params_override = vec![RequestParameter::Url("jahr".to_owned(), Some("2016".to_owned()))];
        url = service.get_request_url(0, &params_override);
        assert_eq!("https://feiertage-api.de/api/?jahr=2016", url);

        params_override.push(RequestParameter::Url("nur_land".to_owned(), Some("NI".to_owned())));
        url = service.get_request_url(0, &params_override);
        assert_eq!("https://feiertage-api.de/api/?jahr=2016&nur_land=NI", url);


        params_override.push(RequestParameter::Url("nur_daten".to_owned(), Some("1".to_owned())));
        url = service.get_request_url(0, &params_override);
        assert_eq!("https://feiertage-api.de/api/?jahr=2016&nur_land=NI&nur_daten=1", url);
    }

    #[test]
    fn test_feiertage_request() {
        let service = get_feiertage_service();
        let (code, _, _, _) = service.do_request(0, &Vec::new()).unwrap();

        assert_eq!(code, 200);
    }

    #[test]
    fn test_invalid_index() {
        let service = get_feiertage_service();
        if let Ok(( _, _, _, _)) = service.do_request(1, &Vec::new()) {
            panic!("Calling the subject with method id 1 should result in an error.")
        }
    }
}

