use base64::{engine::general_purpose, Engine};
use ehttp::{Request, Response};
use serde::{Deserialize, Serialize};

#[typetag::serde(tag = "authentication")]
pub trait RockeryServiceAuthentication {
    fn autenticate(&self, request : &mut Request)-> Result<Response, String> ;
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NoAuthenticationAuthentication {

}

#[typetag::serde]
impl RockeryServiceAuthentication for NoAuthenticationAuthentication {
    fn autenticate(&self, request : &mut Request) -> Result<Response, String> {
        ehttp::fetch_blocking(request)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BasicAuthenticationAuthentication {
    username : String,
    password : String,
}

#[typetag::serde]
impl RockeryServiceAuthentication for BasicAuthenticationAuthentication {
    fn autenticate(&self, request : &mut Request) -> Result<Response, String> {
        let auth = format!("Basic {}:{}", self.username, self.password);
        let base4 = general_purpose::STANDARD.encode(auth);
        request.headers.insert("Authorization", base4);

        ehttp::fetch_blocking(request)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AuthenticationType {
    None(NoAuthenticationAuthentication),
    Basic(BasicAuthenticationAuthentication),
}

impl AuthenticationType {
    pub fn do_request(&self, request : &mut Request) -> Result<Response, String> {
        match self {
            AuthenticationType::None(no_authentication_authentication) => {
                no_authentication_authentication.autenticate(request)
            },
            
            AuthenticationType::Basic(basic_authentication_authentication) => {
                basic_authentication_authentication.autenticate(request)
            },
        }
    }
}