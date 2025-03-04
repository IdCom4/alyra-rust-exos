use std::error::Error as Error;

use super::dao::TDAO as TDAO;
use crate::entities::contact::Contact as Contact;

pub trait TContactDAO: TDAO<Contact> {

  /// Find a contact by its name
  fn find_by_name(&self, name: &str) -> Result<Contact, Box<dyn Error>>;

  //// Find a contact by its phone number
  // fn find_by_phone(&self, phone: &str) -> Result<Contact, Box<dyn Error>>;

}