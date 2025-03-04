use std::sync::Arc;
use std::collections::HashMap;

use super::{ contact_menu, find_contact_menu, Menu, MenuOption, MenuOptionFn, RefreshContactsFn };

pub fn get_list_contacts_menu(refresh_contacts: Arc<RefreshContactsFn>) -> Menu<'static> {

  let generate_lines = {
    let refresh_contacts = refresh_contacts.clone();
    Box::new(move || {
      let mut lines =  vec![
        "=== CONTACTS ===\n\
        <. Back\n\n\
        ?. Find contact by name\n\
        \n\
        Enter contact's index to select it:".to_string(),
      ];

      let contacts = refresh_contacts();

      if contacts.len() == 0 {
        lines.push("- No contacts -".to_string());
      } else {
        lines.extend(contacts.iter().enumerate().map(|(index, contact)| format!("{}. {}", index + 1, contact.to_string())));
      }
    
      lines
    })
  };

  let generate_options = {
    let refresh_contacts = refresh_contacts.clone();

    Box::new(move || {
      
      let contacts = refresh_contacts();

      // static options
      let mut options: HashMap<String, MenuOptionFn> = HashMap::from([
        ("<".to_string(), Box::new(|_, _, _| MenuOption::Back) as MenuOptionFn),
        ("?".to_string(), { let refresh_contacts = refresh_contacts.clone(); Box::new(move |_, _, _| MenuOption::GoTo(find_contact_menu::get_find_contact_menu(refresh_contacts.clone()))) as MenuOptionFn }),
      ]);

      // dynamic options
      for (index, _) in contacts.iter().enumerate() {
        let refresh_contacts = refresh_contacts.clone();

        options.insert((index + 1).to_string(), {
          let contact = contacts.get(index).unwrap().clone();
          let refresh_contacts = refresh_contacts.clone();

          Box::new(move |_, _, _| {
            MenuOption::GoTo(contact_menu::get_contact_menu(refresh_contacts.clone(), &contact))
          }) as MenuOptionFn
        });
      }
      
      options
    })
  };

  Menu::new(
    generate_lines,
    generate_options,
    Box::new(|_, _, _| { println!("Invalid input"); MenuOption::Nothing })
  )
}