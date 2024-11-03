
#[cfg(test)]
pub(crate) fn get_feiertage_service() -> crate::prelude::RockeryService {
    use crate::prelude::*;

    let mut service = RockeryService::new(RockeryServiceType::Rest,"https://feiertage-api.de");
    service.set_authentication(AuthenticationType::None(NoAuthenticationAuthentication {}));
    service.set_name("Feiertage API");
    service.set_service_type(RockeryServiceType::Rest);
    
    let m = service.add_method("get feiertage", "api");
    m.add_parameter(RequestParameter::Url("jahr".to_owned(), Some("2024".to_owned())));
    m.add_parameter(RequestParameter::Url("nur_land".to_owned(), None));
    m.add_parameter(RequestParameter::Url("nur_daten".to_owned(), None));
    m.add_parameter(RequestParameter::Url("callback".to_owned(), None));

    service
}