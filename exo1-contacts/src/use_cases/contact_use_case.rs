use std::error::Error as Error;

use crate::entities::contact::Contact;
use super::apis::daos::contact_dao::TContactDAO;

// Representation of all business rules related to specific contact
pub struct ContactUseCases {
  contact_dao: Box<dyn TContactDAO>
}

impl ContactUseCases {

  pub fn new(contact_dao: Box<dyn TContactDAO>) -> Self {
    ContactUseCases {
      contact_dao
    }
  }

  pub fn update_contact(&self, payload: ContactUpdateDTO) -> Result<(), Box<dyn Error>> {
    self.contact_dao.update(&Contact::from(payload))
  }
    
}

pub struct ContactUpdateDTO {
  pub id: String,
  pub name: String,
  pub phone: String,
}

impl From<ContactUpdateDTO> for Contact {
  fn from(dto: ContactUpdateDTO) -> Self {
    Contact::instance(dto.id, dto.name, dto.phone)
  }
}

