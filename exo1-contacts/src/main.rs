use std::sync::Arc;

mod infra;
mod entities;
mod use_cases;

use infra::menus::main_menu::get_main_menu;
use infra::menus::{Menu, MenuOption};

use crate::use_cases::contact_use_case::ContactUseCases;
use crate::use_cases::contacts_use_case::ContactsUseCases;
use crate::infra::daos::file_contact_dao::FileContactDAO;

const CONTACTS_FILE: &str = "contacts.json";

fn main() {
  // init use cases
  let contact_use_case = ContactUseCases::new(Box::new(FileContactDAO::new(CONTACTS_FILE.to_string())));
  let contacts_use_case = ContactsUseCases::new(Box::new(FileContactDAO::new(CONTACTS_FILE.to_string())));

  // Convert use cases to 'static references
  let contact_use_case_static: &'static ContactUseCases = Box::leak(Box::new(contact_use_case));
  let contacts_use_case_static: &'static ContactsUseCases = Box::leak(Box::new(contacts_use_case));

  // init menus
  let refresh_contacts = Box::new(move || contacts_use_case_static.get_contacts());
  let mut menu_stack: Vec<Menu> = vec![get_main_menu(Arc::new(refresh_contacts))];
  let mut current_menu = menu_stack.last().unwrap();

  loop {

    current_menu.display();

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Error reading input");

    let choice_boxed: Box<str> = choice.into_boxed_str();
    let choice_static: &'static str = Box::leak(choice_boxed);

    let option = current_menu.dispatch(choice_static, contact_use_case_static, contacts_use_case_static);

    match option {
      MenuOption::GoTo(menu) => {
        menu_stack.push(menu);
        current_menu = menu_stack.last().unwrap();
      },
      MenuOption::Back => {
        menu_stack.pop();
        if let Some(menu) = menu_stack.last() {
          current_menu = menu;
        } else {
          break;
        }
      },
      MenuOption::Quit => {
        break;
      },
      MenuOption::Nothing => {},
    }
  }
}
