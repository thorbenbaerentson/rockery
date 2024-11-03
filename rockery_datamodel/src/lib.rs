#[cfg(test)]
mod rockery_test_helper;

mod rockery_project;
mod rockery_service;
mod rockery_service_method;
mod rockery_request_method;
mod rockery_request_parameter;
mod rockery_service_type;
mod rockery_service_authentication;
mod rockery_project_binder;

pub mod prelude {
    use crate::rockery_project;
    use crate::rockery_service;
    use crate::rockery_service_method;
    use crate::rockery_request_method;
    use crate::rockery_request_parameter;
    use crate::rockery_service_type;
    use crate::rockery_service_authentication;
    use crate::rockery_project_binder;

    pub use rockery_project_binder::RockeryProjectBinder;
    pub use rockery_project::RockeryProject;
    
    pub use rockery_service_type::RockeryServiceType;
    pub use rockery_service::RockeryService;
    
    pub use rockery_service_method::RockeryServiceMethod;
    pub use rockery_request_method::RequestMethod;
    pub use rockery_request_parameter::RequestParameter;

    pub use rockery_service_authentication::AuthenticationType;
    pub use rockery_service_authentication::RockeryServiceAuthentication;
    pub use rockery_service_authentication::NoAuthenticationAuthentication;
}


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        
    }
}
