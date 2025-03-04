
use std::{collections::HashMap, sync::Arc};

use super::{ find_contact_menu, add_contact_menu, list_contacts_menu, Menu, MenuOption, MenuOptionFn, RefreshContactsFn };

pub fn get_main_menu(refresh_contacts: Arc<RefreshContactsFn>) -> Menu<'static> {

  Menu {
    generate_lines: Box::new(move || {
      vec![
        "=== MAIN MENU ===\n".to_string(),
        "1. List contacts".to_string(),
        "2. Find contact".to_string(),
        "3. Add contact".to_string(),
        "4. Exit".to_string()
      ]
    }),
    generate_options: {
      Box::new(move || { 
        let refresh_contacts = refresh_contacts.clone();
        HashMap::from([
          ("1".to_string(), { let refresh_contacts = refresh_contacts.clone(); Box::new(move |_, _, _| MenuOption::GoTo(list_contacts_menu::get_list_contacts_menu(refresh_contacts.clone()))) as MenuOptionFn }),
          ("2".to_string(), { let refresh_contacts = refresh_contacts.clone(); Box::new(move |_, _, _| MenuOption::GoTo(find_contact_menu::get_find_contact_menu(refresh_contacts.clone()))) as MenuOptionFn }),
          ("3".to_string(), { Box::new(move |_, _, _| MenuOption::GoTo(add_contact_menu::get_add_contact_menu())) as MenuOptionFn }),
          ("4".to_string(), { Box::new(|_, _, _| MenuOption::Quit) as MenuOptionFn }),
        ])
      })
    },
    default: Box::new(|_, _, _| { println!("Invalid input"); MenuOption::Nothing })
  }
}