use super::unique_entity::{TUniqueEntity, UniqueEntity as UniqueEntity};

pub struct Contact {
  unique_entity: UniqueEntity,

  // IMPROVEMENT: use value objects for those fields, ensuring their validity
  // without the need of manual checking and leaving no room for errors
  name: String,
  phone: String,
}

impl Contact {
  pub fn new(name: String, phone: String) -> Contact {
    Contact { name, phone, unique_entity: UniqueEntity::default() }
  }

  pub fn instance(id: String, name: String, phone: String) -> Contact {
    Contact { name, phone, unique_entity: UniqueEntity::new(id) }
  }

  pub fn get_name(&self) -> &String {
    &self.name
  }

  pub fn get_phone(&self) -> &String {
    &self.phone
  }

  // pub fn set_name(&mut self, name: String) {
  //   self.name = name;
  // }

  // pub fn set_phone(&mut self, phone: String) {
  //   self.phone = phone;
  // }

  pub fn to_string(&self) -> String {
    format!("{} - {}", self.name, self.phone)
  }
}

// 
impl Clone for Contact {
  fn clone(&self) -> Contact {
    Contact { name: self.name.clone(), phone: self.phone.clone(), unique_entity: self.unique_entity.clone() }
  }
}
impl TUniqueEntity for Contact {
  fn get_unique_entity(&self) -> &UniqueEntity {
    &self.unique_entity
  }

  fn get_unique_entity_mut(&mut self) -> &mut UniqueEntity {
    &mut self.unique_entity
  }  
}