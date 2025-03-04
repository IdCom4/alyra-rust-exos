
use std::collections::HashMap;
use std::sync::Arc;

use super::{ contact_menu, Menu, MenuOption, MenuOptionFn, RefreshContactsFn };

pub fn get_find_contact_menu(refresh_contacts: Arc<RefreshContactsFn>) -> Menu<'static> {
  Menu::new(
    Box::new(||
      vec![
        "=== FIND CONTACT ===\n\
        <. Back\n\n\
        Enter contact's name:".to_string(),
      ]
    ),
    HashMap::from([
      ("<".to_string(), Box::new(|_, _, _| MenuOption::Back) as MenuOptionFn),
    ]),
    {
      Box::new(move |input, _, contacts_use_cases| { 
        match contacts_use_cases.get_by_name(&input) {
          Ok(contact) => {
            MenuOption::GoTo(Arc::new(contact_menu::get_contact_menu(refresh_contacts.clone(), &contact)))
          },
          Err(_) => {
            MenuOption::Nothing
          }
        }
      })
    }
  )
}