use std::{collections::HashMap, fs::File, io::{BufReader, Read, Write}, path::PathBuf};

use serde::{ser::Error, Deserialize, Serialize};
use serde_json::Result;

use crate::prelude::RockeryService;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RockeryProject {
    name : String,

    #[serde(skip)]
    dirty : bool,

    services : HashMap<String, RockeryService>,
}

impl RockeryProject {
    pub fn is_dirty(&self) -> bool  { self.dirty }

    pub fn get_name(&self) -> &str { &self.name }
    pub fn set_name(&mut self, name : &str) {
        self.name = name.to_string();
        self.dirty = true;
    }

    pub fn new(name : &str) -> Self  {
        RockeryProject {
            name : name.to_owned(),
            dirty : true,

            services : HashMap::new(),
        }
    }

    /// Get all services this project contains.
    pub fn get_services(&mut self) -> &mut HashMap<String, RockeryService> {
        &mut self.services
    }

    /// Add the given service to the project and return its id within this project.
    pub fn add_service(&mut self, service : RockeryService) -> String {
        let mut name = service.get_name().to_owned();
        let mut i = 1;
        while self.services.contains_key(&name) {
            name = format!("{}_{}", service.get_name(), i);
            i += 1;
        }

        self.services.insert(name.clone(), service);

        name
    }

    /// Removes the given service from the list of services and returns the service itself.
    pub fn remove_service(&mut self, service : &str) -> std::result::Result<RockeryService, String> {
        if !self.services.contains_key(service) {
            return Err(format!("A service with the name {} does not exist within project {}.", service, self.get_name()));
        }

        Ok(self.services.remove(service).unwrap())
    }

    /// Retrieve a specific service from this project.
    pub fn get_service(&mut self, name : &str) -> &RockeryService {
        &self.services[name]
    }

    pub fn save(&mut self, path : &PathBuf) -> Result<()> {
        let v = serde_json::to_string(self)?;
        match File::create(path) {
            Ok(mut f) => {
                match f.write_all(v.as_bytes()) {
                    Ok(_) => { },
                    Err(e) => {
                        return Err(serde_json::Error::custom(format!("Could not write file. {}", e)));
                    },
                };
            },
            Err(e) => {
                return Err(serde_json::Error::custom(format!("Project path not set. {}", e)));
            },
        };

        self.dirty = false;

        Ok(())
    }

    pub fn load(path : PathBuf) -> Result<Self> {
        match File::open(path) {
            Ok(f) => {
                let mut buf_reader = BufReader::new(f);
                let mut contents = String::new();
                match buf_reader.read_to_string(&mut contents) {
                    Ok(_s) => {
                        let mut value: RockeryProject = serde_json::from_str(contents.as_str())?;
                        value.dirty = false;
                        Ok(value)
                    },

                    Err(e) => {
                        Err(serde_json::Error::custom(format!("Could not load project file. {}", e)))
                    },
                }
            },
            Err(e) => {
                Err(serde_json::Error::custom(format!("Project path not set. {}", e)))
            }
        }
    }
}

impl Default for RockeryProject {
    fn default() -> Self {
        Self::new("Unnamed")
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::rockery_test_helper::get_feiertage_service;
    use super::*;

    #[test]
    fn test_getter_setter() {
        let mut subject = RockeryProject::new("Unnamed");
        assert!(subject.is_dirty());

        let name = "REST-Test".to_owned();
        let file = "Test.proj.json";
        assert!(subject.get_services().is_empty());

        let id = subject.add_service(get_feiertage_service());

        assert!(subject.is_dirty());

        // Name
        assert_eq!(subject.get_name(), "Unnamed");
        subject.set_name(&name);
        assert_eq!(subject.get_name(), name);
        assert!(subject.is_dirty());

        if subject.save(&PathBuf::from_str("Test.proj.json").unwrap()).is_err() { 
            panic!("Could not save file even so i should be able to do so.");
        }

        assert!(!subject.is_dirty());

        assert!(subject.get_services().len() == 1);
        assert!(subject.remove_service(&id).is_ok());
        assert!(subject.get_services().is_empty());

        let check = RockeryProject::load(file.into()).unwrap();
        assert!(!check.is_dirty());
        assert_eq!(check.get_name(), subject.get_name());
    }
}