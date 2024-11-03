use std::{collections::HashMap, path::PathBuf, str::FromStr};
use serde::{ Deserialize, Serialize };
use crate::prelude::{RockeryProject, RockeryService};

/// A collection of projects. It basically serves as a memory to compile all projects has been working on and is loaded by
/// default.
#[derive(Serialize, Deserialize, Debug)]
pub struct RockeryProjectBinder {
    // Projects are stored outside the project binder.
    #[serde(skip)]
    name_to_project_map : HashMap<String, usize>,

    #[serde(skip)]
    projects : Vec<RockeryProject>,

    // Path to all project we know about. 
    project_paths : Vec<PathBuf>,
}

impl RockeryProjectBinder {
    const DEFAULT_PROJECT_PATH : &'static str = "projects.json";

    pub fn new() -> Self {
        RockeryProjectBinder {
            name_to_project_map : HashMap::new(),
            projects : Vec::new(),
            project_paths : Vec::new(),
        }
    }

    /// Create a new project with a unique, generic name in the context of this project collection.
    /// Returns the name of the newly created project.
    pub fn create_project(&mut self) -> &RockeryProject {
        let mut name = "Unnamed".to_owned();
        let mut counter = 1;
        while self.name_to_project_map.contains_key(&name) {
            name = format!("{}_{}", name, counter);
            counter += 1;
        }

        let pro = RockeryProject::new(&name);
        let index = self.projects.len();
        self.project_paths.push(PathBuf::from_str(&format!("{}.json", pro.get_name())).unwrap());
        self.projects.push(pro);
        self.name_to_project_map.insert(name.clone(), index);

        &self.projects[index]
    }

    pub fn get_projects(&mut self) -> &mut Vec<RockeryProject> {
        &mut self.projects
    }

    pub fn get_project(&mut self, index : usize) -> Option<&mut RockeryProject> {
        if index >= self.projects.len() { return None; }

        Some(&mut self.projects[index])
    }

    pub fn get_project_file(&self, index : usize) -> Option<&PathBuf> {
        if index >= self.project_paths.len() { return None; }

        Some(self.project_paths.get(index).unwrap())
    }

    pub fn set_project_file(&mut self, index : usize, path : &PathBuf) -> Result<(), String>{
        if index >= self.project_paths.len() { return Err("The project index was invalid.".to_owned()); }

        self.project_paths[index] = path.to_owned();

        Ok(())
    }

    pub fn add_service(&mut self, project_index : usize, service : RockeryService) -> Result<(), String> {
        if self.projects.len() >= project_index {
            return Err("Invalid index".to_owned());
        }

        self.projects[project_index].add_service(service);

        Ok(())
    }

    // pub fn remove_service(&mut self, project_index : usize, serive_index : usize) -> Result<(), String> {
    //     if self.projects.len() >= project_index {
    //         return Err("Invalid index".to_owned());
    //     }

    //     self.projects[project_index].
    // }

    pub fn rename_project(&mut self, old_name : &str, new_name : &str) -> Result<&mut RockeryProject, String> {
        if self.name_to_project_map.contains_key(new_name) {
            return Err("Project cannot be renamed because the new name is already taken. Project names must be unique.".to_owned())
        }

        if !self.name_to_project_map.contains_key(old_name) {
            return Err(format!("A project with the name {} does not exist.", old_name));
        }

        let old_index = self.name_to_project_map[old_name];
        self.name_to_project_map.remove_entry(old_name);
        self.projects.get_mut(old_index).unwrap().set_name(new_name);

        self.name_to_project_map.insert(new_name.to_owned(), old_index);

        Ok(&mut self.projects[old_index])
    }

    pub fn remove_project(&mut self, name : &str) -> Result<(), String> {
        if !self.name_to_project_map.contains_key(name) {
            return Err(format!("A project with the name {} does not exist.", name));
        }

        let index = self.name_to_project_map[name];
        self.projects.remove(index);
        self.name_to_project_map.remove(name);
        self.project_paths.remove(index);

        Ok(())
    }

    pub fn load_project(&mut self, file : &PathBuf) -> Result<&mut RockeryProject, String> {
        match RockeryProject::load(file.clone()) {
            Ok(p) => {
                if self.name_to_project_map.contains_key(p.get_name()) {
                    return Err(format!("Could not load '{}' because there is a another project with the same name already loaded.", p.get_name()));
                }

                let index = self.projects.len();
                self.name_to_project_map.insert(p.get_name().to_owned(), index);
                self.projects.push(p);
                self.project_paths.push(file.to_owned());

                Ok(&mut self.projects[index])
            },
            
            Err(e) => {
                Err(format!("Could not load a project. Error: {}.", e))
            },
        }
    }

    /// Tries to save all projects then the binder itself.
    pub fn save(&mut self) -> Result<(), String> {
        for (i, p ) in &mut self.projects.iter_mut().enumerate() {
            let path = &self.project_paths[i];

            match p.save(path) {
                Ok(_) => { },
                Err(e) => {
                    return Err(format!("Could not save project '{}'. Error: {}.", p.get_name(), e))
                },
            }
        }

        match serde_json::to_string(&self) {
            Ok(e) => {
                match std::fs::write(Self::DEFAULT_PROJECT_PATH, e) {
                    Ok(_) => { },
                    Err(e) => {
                        return Err(format!("Could not store projects binder. Error: {}", e));
                    },
                }
            },

            Err(e) => {
                return Err(format!("Could not store projects binder. Error: {}", e));
            },
        }

        Ok(())
    }

    pub fn load() -> Result<Self, String> {
        match std::fs::read_to_string(Self::DEFAULT_PROJECT_PATH) {
            Ok(e) => {
                match serde_json::from_str::<RockeryProjectBinder>(&e) {
                    Ok(mut e) => {
                        let clone = e.project_paths.clone();
                        for p in &clone{
                            match e.load_project(p) {
                                Ok(_) => { },
                                Err(e) => {
                                    return Err(format!("Could not load a project. Error: {}", e));
                                },
                            }
                        }

                        Ok(e)
                    },
                    Err(e) => {
                        Err(format!("Could not load projects binder. Error: {}", e))
                    },
                }
            },
            Err(e) => {
                Err(format!("Could not load projects binder. Error: {}", e))
            },
        }
    }
}

impl Default for RockeryProjectBinder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_tor() {
        let mut binder = RockeryProjectBinder::new();
        assert!(binder.get_project(0).is_none());
        assert!(binder.get_project(1).is_none());
        assert_eq!(binder.get_projects().len(), 0);

        assert_eq!(binder.create_project().get_name(), "Unnamed");
        assert_eq!(binder.create_project().get_name(), "Unnamed_1");
        assert_eq!(binder.get_projects().len(), 2);
        
        assert_eq!(binder.get_project(0).unwrap().get_name(), "Unnamed");
        assert_eq!(binder.get_project(1).unwrap().get_name(), "Unnamed_1");
        assert!(binder.get_project(2).is_none());
    }

    #[test]
    fn test_renaming_project() {
        let old_name = "Unnamed_1".to_owned();
        let new_name = "Test".to_owned();

        let mut binder = RockeryProjectBinder::new();
        assert_eq!(binder.create_project().get_name(), "Unnamed");
        assert_eq!(binder.create_project().get_name(), old_name);

        assert_eq!(binder.rename_project(&old_name, &new_name).unwrap().get_name(), new_name);

        assert_eq!(binder.get_project(0).unwrap().get_name(), "Unnamed");
        assert_eq!(binder.get_project(1).unwrap().get_name(), new_name);
        assert!(binder.get_project_file(0).is_some());
        assert!(binder.get_project_file(1).is_some());
        assert!(binder.get_project(2).is_none());

        assert!((binder.rename_project("Unnamed", "Test").is_err()));
    }

    #[test]
    fn test_removing_project() {
        let old_name = "Unnamed_1".to_owned();
        let new_name = "Test".to_owned();

        let mut binder = RockeryProjectBinder::new();
        assert_eq!(binder.create_project().get_name(), "Unnamed");
        assert_eq!(binder.create_project().get_name(), old_name);

        assert!(binder.remove_project(&new_name).is_err());
        assert_eq!(binder.rename_project(&old_name, &new_name).unwrap().get_name(), new_name);
        assert!(binder.remove_project(&new_name).is_ok());

        assert_eq!(binder.get_project(0).unwrap().get_name(), "Unnamed");
        assert!(binder.get_project(1).is_none());
    }

    #[test]
    fn test_storing_binder() {
        let mut binder = RockeryProjectBinder::new();
        assert_eq!(binder.create_project().get_name(), "Unnamed");
        assert_eq!(binder.create_project().get_name(), "Unnamed_1");

        // Set a file path...
        assert!(binder.set_project_file(0, &PathBuf::from_str("Test_2.json").unwrap()).is_ok());
        assert!(binder.set_project_file(1, &PathBuf::from_str("Feiertage.json").unwrap()).is_ok());

        binder.rename_project("Unnamed", "Test_1").unwrap();
        binder.rename_project("Unnamed_1", "Feiertage").unwrap();

        // ... now we can store the binder.
        match binder.save() {
            Ok(_) => {},
            Err(e) => println!("{}", e),
        }
        assert!(binder.save().is_ok());

        // Load the binder again and check the result.
        let res = RockeryProjectBinder::load();
        assert!(res.is_ok());

        let mut binder_2 = res.unwrap();
        
        // Reload and compare both projects.
        assert!(binder_2.get_project(0).is_some());
        assert_eq!(binder_2.get_project(0).unwrap().get_name(), "Test_1");
        assert!(binder.get_project(0).is_some());
        assert_eq!(binder.get_project(0).unwrap().get_name(), "Test_1");

    }
}
