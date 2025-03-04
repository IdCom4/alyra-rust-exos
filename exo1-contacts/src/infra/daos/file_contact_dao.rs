use std::error::Error as Error;

use crate::entities::contact::Contact as Contact;
use crate::use_cases::apis::daos::{ contact_dao::TContactDAO as TContactDAO, dao::TDAO as TDAO };
use super::file_dao::FileDAO;

pub struct FileContactDAO {
  file_dao: FileDAO<Contact, fn(&Contact) -> String, fn(&str) -> Contact>,
}

// File-based implementation of the ContactDAO trait
impl FileContactDAO {
    pub fn new(file_path: String) -> Self {
        FileContactDAO { file_dao: FileDAO::new(
          file_path,
          | contact | contact_serialization::serialize(contact),
          | str_contact | contact_serialization::deserialize(str_contact),
        ) }
    }
}

// TDAO implementation
impl TDAO<Contact> for FileContactDAO {
    fn create(&self, entity: &Contact) -> Result<String, Box<dyn Error>> {
        self.file_dao.create(entity)
    }

    fn update(&self, entity: &Contact) -> Result<(), Box<dyn Error>> {
        self.file_dao.update(entity)
    }

    fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
        self.file_dao.delete(id)
    }

    fn find_all(&self) -> Result<Vec<Contact>, Box<dyn Error>> {
        self.file_dao.find_all()
    }

    fn find_by_id(&self, id: &str) -> Result<Contact, Box<dyn Error>> {
        self.file_dao.find_by_id(id)
    }
}

// TContactDAO implementation
impl TContactDAO for FileContactDAO {

    fn find_by_name(&self, name: &str) -> Result<Contact, Box<dyn Error>> {
      let file_content = self.file_dao.read_file()?;
    
      match file_content.lines().find(|line| { 
        let contact: Contact = (self.file_dao.json_deserializer)(line);
        contact.get_name().trim().to_lowercase() == name.trim().to_lowercase()
      }) {
        Some(str_contact) => Ok((self.file_dao.json_deserializer)(str_contact)),
        None => Err("Contact not found".into())
      }
    }

    // fn find_by_phone(&self, phone: &str) -> Result<Contact, Box<dyn Error>> {
    //   let file_content = self.file_dao.read_file()?;
    
    //   match file_content.lines().find(|line| { 
    //     let contact: Contact = (self.file_dao.json_deserializer)(line);
    //     contact.get_phone() == phone
    //   }) {
    //     Some(str_contact) => Ok((self.file_dao.json_deserializer)(str_contact)),
    //     None => Err("Contact not found".into())
    //   }
    // }

}

/// Contact serialization/deserialization
mod contact_serialization {
    use serde::{Serialize, Deserialize};

    use crate::entities::{contact::Contact as Contact, unique_entity::TUniqueEntity};
    use crate::infra::json as Json;

    #[derive(Serialize, Deserialize)]
    struct ContactSerialization {
        id: String,
        name: String,
        phone: String,
    }

    impl From<Contact> for ContactSerialization {
        fn from(contact: Contact) -> Self {
            ContactSerialization {
                id: contact.get_unique_entity().get_id().to_string(),
                name: contact.get_name().to_string(),
                phone: contact.get_phone().to_string(),
            }
        }
    }

    impl From<ContactSerialization> for Contact {
        fn from(contact_serialization: ContactSerialization) -> Self {
            Contact::instance(
                contact_serialization.id,
                contact_serialization.name,
                contact_serialization.phone,
            )
        }
    }

    pub fn serialize(contact: &Contact) -> String {
        Json::serialize(&ContactSerialization::from(contact.clone()))
    }

    pub fn deserialize<'a>(json: &'a str) -> Contact {
        let contact_serialization: ContactSerialization = Json::deserialize(json);
        Contact::from(contact_serialization)
    }
}