use std::error::Error;

use std::path::Path;

use crate::{
  entities::unique_entity::TUniqueEntity,
  use_cases::apis::daos::dao::TDAO
};

/// FileDAO struct, an file-based implementation of the DAO trait
pub struct FileDAO<T: TUniqueEntity, FS: Fn(&T) -> String, FD: Fn(&str) -> T> {
  pub file_path: String,

  pub json_serializer: FS,
  pub json_deserializer: FD,
}

/// Base implementation of the FileDAO struct specifics
impl <T: TUniqueEntity, FS: Fn(&T) -> String, FD: Fn(&str) -> T> FileDAO<T, FS, FD> {
  pub fn new(file_path: String, json_serializer: FS, json_deserializer: FD) -> Self {
    FileDAO {
      file_path,
      json_serializer,
      json_deserializer
    }
  }

  pub fn read_file(&self) -> Result<String, Box<dyn Error>> {
    let path = Path::new(&self.file_path);

    match path.exists() {
      true => std::fs::read_to_string(path).map_err(|e| e.to_string().into()),
      false => Ok("".to_string())
    }
  }

  pub fn write_file(&self, content: &str) {
    std::fs::write(Path::new(&self.file_path), content).map_err(|e| e.to_string()).unwrap();
  }

  pub fn get_next_available_id(&self, entries: Vec<&str>) -> Result<u64, Box<dyn Error>> {
    let nbr_entries = entries.len();
    match nbr_entries {
      0 => Ok(1),
      _ => {
        let last_entry: T = (self.json_deserializer)(&entries[nbr_entries - 1]);
        Ok(last_entry.get_unique_entity().get_id().parse::<u64>()? + 1)
      }
    }
  }
}

// Base implementation of the DAO trait for FileDAO
impl<T: TUniqueEntity> TDAO<T> for FileDAO<T, fn(&T) -> String, fn(&str) -> T> {

  fn create(&self, entity: &T) -> Result<String, Box<dyn Error>> {
    let content = self.read_file()?;
    let entries: Vec<&str> = content.lines().collect();

    let new_id = self.get_next_available_id(entries)?.to_string();
    let mut new_entity = entity.clone();
    new_entity.get_unique_entity_mut().set_id(new_id.clone());
    
    let new_content = {
      let serialized_entity = (self.json_serializer)(&new_entity);

      match content.is_empty() {
        true => format!("{}\n", serialized_entity),
        false => format!("{}{}\n", content, serialized_entity)
      }
    }; 

    self.write_file(&new_content);

    Ok(new_id)
  }


  fn update(&self, entity: &T) -> Result<(), Box<dyn Error>> {
      let content = self.read_file()?;
      let updated_entries: Vec<String> = content.lines().map(|line| {

        let current_entity: T = (self.json_deserializer)(line);
        
        if current_entity.get_unique_entity().get_id() == entity.get_unique_entity().get_id() {
          (self.json_serializer)(&entity)
        } else {
          line.to_string()
        }
      }).collect();

      self.write_file(&updated_entries.join("\n"));

      Ok(())
  }

  fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
    let content = self.read_file()?;
    let updated_entries: Vec<&str> = content.lines().filter(|line| {      
      let curr_entity = (self.json_deserializer)(line);

      curr_entity.get_unique_entity().get_id() != id
    }).collect();

    self.write_file(&updated_entries.join("\n"));

    Ok(())
  }

  fn find_all(&self) -> Result<Vec<T>, Box<dyn Error>> {
      Ok(self.read_file()?.lines().map(|line| (self.json_deserializer)(line)).collect())
  }

  fn find_by_id(&self, id: &str) -> Result<T, Box< dyn Error>> {
    let file_content = self.read_file()?;
    
    match file_content.lines().find(|line| { 
      let entity: T = (self.json_deserializer)(line);
      entity.get_unique_entity().get_id() == id
    }) {
      Some(entity) => Ok((self.json_deserializer)(entity)),
      None => Err("Entity not found".into())
    }
  }
}
