use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub enum RockeryServiceType {
    #[default]
    Generic,
    Soap,   // Url or path to wsdl file
    Rest,   // Url or path to wadl file
}

impl std::fmt::Display for RockeryServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RockeryServiceType::Generic => write!(f, "GENERIC"),
            RockeryServiceType::Soap => write!(f, "SOAP"),
            RockeryServiceType::Rest => write!(f, "REST"),
        }
    }
}

impl FromStr for RockeryServiceType {
    type Err = ();

    fn from_str(input: &str) -> Result<RockeryServiceType, Self::Err> {
        if input.starts_with("GENERIC") {
            return Ok(RockeryServiceType::Generic); 
        }

        if input.starts_with("SOAP") {
            return Ok(RockeryServiceType::Soap); 
        }

        if input.starts_with("REST") {
            return Ok(RockeryServiceType::Rest); 
        }

        Err(())
    }
}

#[cfg(test)]
mod rockery_service_type_tests {
    use super::*;

    #[test]
    fn serialization_tests() {
        let mut subject = RockeryServiceType::Generic;
        assert_eq!(subject, RockeryServiceType::from_str(&subject.to_string()).unwrap());

        subject = RockeryServiceType::Soap;
        assert_eq!(subject, RockeryServiceType::from_str(&subject.to_string()).unwrap());

        subject = RockeryServiceType::Rest;
        assert_eq!(subject, RockeryServiceType::from_str(&subject.to_string()).unwrap());
    }
}