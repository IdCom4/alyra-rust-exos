use std::error::Error as Error;

use crate::entities::unique_entity::TUniqueEntity as TUniqueEntity;

pub trait TDAO<T: TUniqueEntity> {

  /// Create a new entity and insert it in the database
  fn create(&self, entity: &T) -> Result<String, Box<dyn Error>>;

  /// Update an entity in the database
  fn update(&self, entity: &T) -> Result<(), Box<dyn Error>>;

  /// Delete an entity from the database
  fn delete(&self, id: &str) -> Result<(), Box<dyn Error>>;

  /// Find all entities in the database
  fn find_all(&self) -> Result<Vec<T>, Box<dyn Error>>;

  /// Find an entity by its id
  fn find_by_id(&self, id: &str) -> Result<T, Box<dyn Error>>;
}