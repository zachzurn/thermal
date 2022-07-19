use crate::parser::Command;
use std::rc::Rc;


pub fn all() -> Rc<Vec<Command>> {
  let all: Vec<Command> = vec![
  ];

  Rc::new(all)
}