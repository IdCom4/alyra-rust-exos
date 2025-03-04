use std::collections::HashMap;
use std::sync::Arc;

use crate::{entities::contact::Contact, use_cases::{contact_use_case::ContactUseCases, contacts_use_case::ContactsUseCases}};

pub mod main_menu;
pub mod list_contacts_menu;
pub mod add_contact_menu;
pub mod find_contact_menu;
pub mod contact_menu;
pub mod edit_contact_menu;

pub enum MenuOption {
  GoTo(Arc<Menu<'static>>),
  Back,
  Quit,
  Nothing,
}

pub type RefreshContactsFn = Box<dyn Fn() -> Vec<Contact>>;
pub type MenuOptionFn<'a> = Box<dyn Fn(String, &'a ContactUseCases, &'a ContactsUseCases) -> MenuOption + 'a>;

pub struct Menu<'a> {
  generate_lines: Box<dyn Fn() -> Vec<String> + 'a>,
  options: HashMap<String, MenuOptionFn<'a>>,
  default: MenuOptionFn<'a>,
}

impl<'a> Menu<'a> {
  pub fn new(generate_lines: Box<dyn Fn() -> Vec<String> + 'a>, options: HashMap<String, MenuOptionFn<'a>>, default: MenuOptionFn<'a>) -> Self {
      Menu { generate_lines, options, default }
  }

  pub fn display(&self) {
      print!("{}[2J", 27 as char);
      print!("\x1B[2J\x1B[1;1H");

      let lines = (self.generate_lines)();
      
      for line in lines {
          println!("{}", line);
      }
  }

  pub fn dispatch(&self, input: &'a str, contact_use_cases: &'a ContactUseCases, contacts_use_cases: &'a ContactsUseCases) -> MenuOption {
      let input = input.trim().to_lowercase().to_string();
      match self.options.get(&input) {
          Some(action) => action(input, contact_use_cases, contacts_use_cases),
          None => (self.default)(input, contact_use_cases, contacts_use_cases),
      }
  }
}