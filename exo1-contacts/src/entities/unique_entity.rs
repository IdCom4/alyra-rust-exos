pub trait TUniqueEntity: Clone {
  fn get_unique_entity(&self) -> &UniqueEntity;
  fn get_unique_entity_mut(&mut self) -> &mut UniqueEntity;
}

pub struct UniqueEntity {
    id: String,
}

impl UniqueEntity {
    pub fn new(id: String) -> UniqueEntity {
        UniqueEntity { id }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
}

impl Clone for UniqueEntity {
    fn clone(&self) -> UniqueEntity {
        UniqueEntity { id: self.id.clone() }
    }
}

impl Default for UniqueEntity {
    fn default() -> Self {
        UniqueEntity { id: String::from("") }
    }
}