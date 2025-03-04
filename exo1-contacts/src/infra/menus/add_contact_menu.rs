
use std::collections::HashMap;

use crate::use_cases::contacts_use_case::NewContactDTO;

use super::{ Menu, MenuOption, MenuOptionFn };

pub fn get_add_contact_menu() -> Menu<'static> {
  Menu::new(
    Box::new(|| { 
      vec![
      "<. Back\n\
      Enter <name> <phone> to add a contact".to_string(),
      ]
    }),
    Box::new(||  HashMap::from([ ("<".to_string(), Box::new(|_, _, _| MenuOption::Back) as MenuOptionFn) ])),
    {
      Box::new(|input, _, contacts_use_cases| {
        let parts: Vec<&str> = input.split(' ').collect();
        if parts.len() != 2 {
            println!("Invalid input format");
            return MenuOption::Nothing;
        }

        let name = parts.get(0).unwrap();
        let phone = parts.get(1).unwrap();

        match contacts_use_cases.add_contact(NewContactDTO {
          name: name.to_string(),
          phone: phone.to_string(),
        }) {
          Ok(_) => {
            MenuOption::Back
          },
          Err(_) => {
            println!("Error adding contact");
            MenuOption::Nothing
          }
        }
      })
    }
  )
}