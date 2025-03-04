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


  let find_contact_menu = Arc::new(find_contact_menu::get_find_contact_menu(refresh_contacts.clone()));

  let mut options: HashMap<String, MenuOptionFn> = HashMap::from([
    ("<".to_string(), Box::new(|_, _, _| MenuOption::Back) as MenuOptionFn),
    ("?".to_string(), {
      let find_contact_menu = Arc::clone(&find_contact_menu);
      Box::new(move |_, _, _| MenuOption::GoTo(find_contact_menu.clone())) as MenuOptionFn
    }),
  ]);

  let contacts = refresh_contacts();
  
  for (index, _) in contacts.iter().enumerate() {
    options.insert((index + 1).to_string(), {
      println!("index: {}", index + 1);
      let contact = contacts.get(index).unwrap().clone();
      let refresh_contacts = refresh_contacts.clone();

      Box::new(move |_, _, _| {
        MenuOption::GoTo(Arc::new(contact_menu::get_contact_menu(refresh_contacts.clone(), &contact)))
      }) as MenuOptionFn
    });
  }

  Menu::new(
    generate_lines,
    options,
    Box::new(|_, _, _| { println!("Invalid input"); MenuOption::Nothing })
  )
}