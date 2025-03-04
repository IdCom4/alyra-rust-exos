
use std::collections::HashMap;
use std::sync::Arc;
use crate::entities::{contact::Contact, unique_entity::TUniqueEntity};
use crate::use_cases::contact_use_case::ContactUpdateDTO;

use super::{ Menu, MenuOption, MenuOptionFn, RefreshContactsFn };

pub fn get_edit_contact_menu(refresh_contacts: Arc<RefreshContactsFn>, contact: &Contact) -> Menu<'static> {

  let generate_lines = {
    let refresh_contacts = refresh_contacts.clone();

    let contacts = refresh_contacts().clone();
    let contact = contacts.iter()
      .find(|c| c.get_unique_entity().get_id() == contact.get_unique_entity().get_id());

    let contact_info = match contact {
      Some(c) => c.to_string(),
      None => "Contact not found".to_string(),
    };
    Box::new(move || {
      vec![
        "=== EDIT CONTACT ===\n\
        <. Back\n\
        \n".to_string(),
        contact_info.clone(),
        "\n\
        1|<new_name>\n\
        2|<new_phone>".to_string(),
      ]
    })
  };

  let closure: MenuOptionFn = {
    let contact_id = contact.get_unique_entity().get_id().clone();
    let contact_name = contact.get_name().clone();
    let contact_phone = contact.get_phone().clone();

    Box::new(move |input, contact_use_cases, _| {
      let parts: Vec<&str> = input.split('|').collect();
      if parts.len() != 2 {
          println!("Invalid input format");
          return MenuOption::Nothing;
      }

      let request = parts[0].trim();
      let new_value = parts[1].trim().to_string();

      if request == "1" {
        let _ = contact_use_cases.update_contact(ContactUpdateDTO {
            id: contact_id.clone(),
            name: new_value,
            phone: contact_phone.clone(),
        });
        MenuOption::Back
      } else if request == "2" {
        let _ = contact_use_cases.update_contact(ContactUpdateDTO {
            id: contact_id.clone(),
            name: contact_name.clone(),
            phone: new_value,
        });
          MenuOption::Back
      } else {
          println!("Invalid input");
          MenuOption::Nothing
      }
    })
  };

  Menu::new(
    generate_lines,
    HashMap::from([
      ("<".to_string(), Box::new(|_, _, _| MenuOption::Back) as MenuOptionFn),
    ]),
    closure
  )
}
