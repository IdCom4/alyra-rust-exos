
use std::error::Error as Error;

use crate::entities::contact::Contact;
use super::apis::daos::contact_dao::TContactDAO;

// Representation of all business rules related to contacts management
pub struct ContactsUseCases {
    contact_dao: Box<dyn TContactDAO>
}

impl ContactsUseCases {

  pub fn new(contact_dao: Box<dyn TContactDAO>) -> Self {
    ContactsUseCases {
      contact_dao
    }
  }

  pub fn add_contact(&self, new_contact: NewContactDTO) -> Result<String, Box<dyn Error>> {
    self.contact_dao.create(&Contact::from(new_contact))
  }

  pub fn get_contacts(&self) -> Vec<Contact> {
      match self.contact_dao.find_all() {
          Ok(contacts) => contacts,
          Err(_) => Vec::new()
      }
  }

  // pub fn get_by_id(&self, id: &str) -> Result<Contact, Box<dyn Error>> {
  //   self.contact_dao.find_by_id(id)
  // }

  pub fn get_by_name(&self, name: &str) -> Result<Contact, Box<dyn Error>> {
    self.contact_dao.find_by_name(name)
  }

  pub fn delete_contact(&self, id: &str) -> Result<(), Box<dyn Error>> {
    self.contact_dao.delete(id)
  }
    
}

pub struct NewContactDTO {
  pub name: String,
  pub phone: String,
}

impl From<NewContactDTO> for Contact {
  fn from(dto: NewContactDTO) -> Self {
    Contact::new(dto.name, dto.phone)
  }
}

