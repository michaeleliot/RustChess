use std::{fmt::{Display, Debug}};

//  TODO: Decide if this is better as an Option and not an N value
#[derive(Clone, Copy, PartialEq)]
pub enum BoardSpaceState {
  X,
  O,
  N
}


impl Debug for BoardSpaceState {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    Display::fmt(&self, f)
  }
}

impl Display for BoardSpaceState {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      BoardSpaceState::X => write!(f, "X"),
      BoardSpaceState::O => write!(f, "O"),
      BoardSpaceState::N => write!(f, " "),
    }
  }
}