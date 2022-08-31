use std::{fmt::{Display, Debug}};
use colored::Colorize;
use rand::Rng;

use crate::board_space::BoardSpaceState;

#[derive(Debug)]
pub struct Board {
  spaces: [BoardSpaceState ; 9]
}

impl Display for Board {
  fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
      for (i, space) in self.spaces.iter().enumerate() {
        let mut space_string = format!("{}", space);

        let is_not_last_column = i <= 5;
        if is_not_last_column {
          space_string = space_string.underline().to_string();
        }
        print!("{}", space_string);

        let is_end_of_row = i % 3 == 2;
        if is_end_of_row {
          println!(""); 
        } else { 
          let mut divider = format!("|");
          if is_not_last_column {
            divider = divider.underline().to_string();
          }
          print!("{}", divider);
        }
      }

      Ok(())
  }
}


impl Default for Board {
  fn default() -> Self {
      Board {
        spaces: [BoardSpaceState:: N; 9]
      }
  }
}


impl Board {
  pub fn place(&mut self, space_index: usize, piece: BoardSpaceState) -> Result<Option<BoardSpaceState>, &str> {
    if space_index < 9 && self.spaces[space_index] != BoardSpaceState::N {
      return Err("Illegal Move")
    }
    self.spaces[space_index] = piece;

    return Ok(self.check_game_over());
  }

  pub fn ai_turn(&mut self) -> Result<Option<BoardSpaceState>, &str> {
    fn choose_space_index(space_indexes: Vec<usize>) -> usize {
      let mut rng = rand::thread_rng();
      let random_space_index = rng.gen_range(0..space_indexes.len());
      return space_indexes[random_space_index]
    }

    fn find_potential_space_indexes(spaces: [BoardSpaceState; 9]) -> Vec<usize> {
      let mut potential_space_indexes = vec![];
      for (i, &item) in spaces.iter().enumerate() {
        if item == BoardSpaceState::N {
          potential_space_indexes.push(i);
        }
      }
      potential_space_indexes
    }

    let potential_space_indexes = find_potential_space_indexes(self.spaces);
    if potential_space_indexes.len() == 0 {
      return Err("Game Over: Stalemate")
    }
    let space_index_choice = choose_space_index(potential_space_indexes);
    let place_result = self.place(space_index_choice, BoardSpaceState::O);
    
    Ok(place_result.unwrap())
  }

  fn check_game_over(&self) -> Option<BoardSpaceState> {
    fn check_three_in_row(three_values: [BoardSpaceState; 3]) -> Option<BoardSpaceState> {
      if three_values.iter().all(|&space| space == BoardSpaceState::X) {
        return Some(BoardSpaceState::X)
      }
      if three_values.iter().all(|&space| space == BoardSpaceState::O) {
        return Some(BoardSpaceState::O)
      }
      None
    }
    
    let optional_winner_top = check_three_in_row(self.spaces[0 .. 3].try_into().unwrap());
    let optional_winner_middle = check_three_in_row(self.spaces[3 .. 6].try_into().unwrap());
    let optional_winner_bottom = check_three_in_row(self.spaces[6 .. 9].try_into().unwrap());

    let optional_winner_first_col = check_three_in_row([self.spaces[0], self.spaces[3], self.spaces[6]].try_into().unwrap());
    let optional_winner_second_col = check_three_in_row([self.spaces[1], self.spaces[4], self.spaces[7]].try_into().unwrap());
    let optional_winner_third_col = check_three_in_row([self.spaces[2], self.spaces[5], self.spaces[8]].try_into().unwrap());

    let optional_winner_left_to_right_diag = check_three_in_row([self.spaces[0], self.spaces[4], self.spaces[8]].try_into().unwrap());
    let optional_winner_right_to_left_diag = check_three_in_row([self.spaces[2], self.spaces[4], self.spaces[6]].try_into().unwrap());


    let all_possible_win_options = [
      optional_winner_top,
      optional_winner_middle,
      optional_winner_bottom,
      optional_winner_first_col,
      optional_winner_second_col,
      optional_winner_third_col,
      optional_winner_left_to_right_diag,
      optional_winner_right_to_left_diag
    ];
    
    let optional_winning_player = all_possible_win_options
      .iter()
      .filter(|&&x| !x.is_none())
      .map(|x| x.unwrap())
      .next();
    
    if optional_winning_player.is_some() {
      println!();
      println!("Game Over, Winner Is {}", optional_winning_player.unwrap());
    }
    return optional_winning_player;
  }
}