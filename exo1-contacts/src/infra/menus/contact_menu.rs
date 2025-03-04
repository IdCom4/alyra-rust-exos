
use std::collections::HashMap;
use std::sync::Arc;
use crate::{entities::{contact::Contact, unique_entity::TUniqueEntity}, use_cases::contacts_use_case::ContactsUseCases};

use super::{ edit_contact_menu, Menu, MenuOption, MenuOptionFn, RefreshContactsFn };

pub fn get_contact_menu(refresh_contacts: Arc<RefreshContactsFn>, contact: &Contact) -> Menu<'static> {

  let edit_contact_menu = Arc::new(edit_contact_menu::get_edit_contact_menu(refresh_contacts.clone(), &contact));

  let generate_lines = {
    let refresh_contacts = refresh_contacts.clone();
    let contact = contact.clone();
    Box::new(move || {
      let contacts = refresh_contacts();
      let contact_info = 
        contacts
          .iter()
          .find(|c| {
            c.get_unique_entity().get_id() == contact.get_unique_entity().get_id()
          })
          .map(|c| c.clone().to_string())
          .unwrap_or_else(|| "Contact not found".to_string());
        
      vec![
        "<. Back\n\
        \n".to_string(),
        contact_info,
        "\n\
        1. Edit\n\
        2. Delete".to_string(),
      ]
    })
  };

  Menu::new(
    generate_lines,//Box::new(move || vec![]),
    HashMap::from([
      ("<".to_string(), Box::new(|_, _, _| MenuOption::Back) as MenuOptionFn),
      ("1".to_string(), {
        let edit_contact_menu = Arc::clone(&edit_contact_menu);
        Box::new(move |_, _, _| MenuOption::GoTo(edit_contact_menu.clone())) as MenuOptionFn
      }),
      ("2".to_string(), {
        let contact_id = contact.get_unique_entity().get_id().clone();
        Box::new(move |_, _, contacts_use_cases: &ContactsUseCases| {
            let _ = contacts_use_cases.delete_contact(&contact_id);
            MenuOption::Back
        }) as MenuOptionFn
      }),
    ]),
    Box::new(|_, _, _| { println!("Invalid input"); MenuOption::Nothing})
  )
}