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

  fn read_file(&self) -> Result<String, Box<dyn Error>> {
    let path = Path::new(&self.file_path);

    match path.exists() {
      true => std::fs::read_to_string(path).map_err(|e| e.to_string().into()),
      false => Ok("".to_string())
    }
  }

  #[allow(dead_code)]
  pub fn write_file(&self, table_content: Vec<T>) {
    let table_content = table_content.iter().map(|entity| (self.json_serializer)(entity)).collect::<Vec<String>>().join(",\n");
    std::fs::write(Path::new(&self.file_path), format!("[\n{}\n]", table_content)).map_err(|e| e.to_string()).unwrap();
  }

  pub fn write_file_str(&self, table_content: Vec<String>) {
    let table_content = table_content.join(",\n");
    std::fs::write(Path::new(&self.file_path), format!("[\n{}\n]", table_content)).map_err(|e| e.to_string()).unwrap();
  }

  pub fn get_str_entries(&self) -> Result<Vec<String>, Box<dyn Error>> {
    let file_content = self.read_file()?;
    let lines = file_content.lines().collect::<Vec<&str>>();

    // remove first and last lines
    match lines.len() > 2 {
      true => return Ok(lines[1..lines.len() - 1].into_iter().enumerate().map(|(index, s)| {
          if index >= lines.len() - 3 { s.to_string() } 
          else {
            // remove trailing comma
              s[..s.len() - 1].to_string()
          }
        }).collect::<Vec<String>>()),
      false => return Ok(lines.into_iter().map(|s| s.to_string()).collect::<Vec<String>>()),
    }
  }

  #[allow(dead_code)]
  pub fn get_entries(&self) -> Result<Vec<T>, Box<dyn Error>> {
    let entries = self.get_str_entries()?;
    Ok(entries.into_iter().map(|s| (self.json_deserializer)(&s)).collect::<Vec<T>>())
    // Ok(Vec::new())
  }

  pub fn get_next_available_id(&self, entries: &Vec<String>) -> Result<u64, Box<dyn Error>> {
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
    let mut entries = self.get_str_entries()?;

    let new_id = self.get_next_available_id(&entries)?.to_string();
    let mut new_entity = entity.clone();
    new_entity.get_unique_entity_mut().set_id(new_id.clone());

    entries.push((self.json_serializer)(&new_entity));

    self.write_file_str(entries);

    Ok(new_id)
  }


  fn update(&self, entity: &T) -> Result<(), Box<dyn Error>> {
      let updated_entries: Vec<String> = self.get_str_entries()?.iter().map(|line| {

        let current_entity: T = (self.json_deserializer)(line);
        
        if current_entity.get_unique_entity().get_id() == entity.get_unique_entity().get_id() {
          (self.json_serializer)(&entity)
        } else {
          line.to_string()
        }
      }).collect();

      self.write_file_str(updated_entries);

      Ok(())
  }

  fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
    let updated_entries: Vec<String> = self.get_str_entries()?.into_iter().filter(|line| {
      let entity: T = (self.json_deserializer)(line);

      entity.get_unique_entity().get_id() != id
    }).collect();

    self.write_file_str(updated_entries);

    Ok(())
  }

  fn find_all(&self) -> Result<Vec<T>, Box<dyn Error>> {
      Ok(self.get_entries()?)
  }

  fn find_by_id(&self, id: &str) -> Result<T, Box< dyn Error>> {
    match self.get_entries()?.into_iter().find(|entity| {      
      entity.get_unique_entity().get_id() == id
    }) {
      Some(entity) => Ok(entity),
      None => Err("Entity not found".into())
    }
  }
}
