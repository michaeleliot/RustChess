use std::io;

mod board;
mod board_space;

fn main() {
    let mut game_over: bool = false;
    let mut board: board::Board = Default::default();

    while !game_over {
      println!();
      println!("{}", board);
      println!("What is your move? (Select 0 through 8)");
      println!("0 | 1 | 2");
      println!("3 | 4 | 5");
      println!("6 | 7 | 8");

      let mut input_request_result = request_input();
      while input_request_result.is_none() {
        input_request_result = request_input();
      }
      let selected_position = input_request_result.unwrap();
  
      let place_result = board.place(selected_position, board_space::BoardSpaceState::X);
      
      match place_result {
        Ok(player_winner) => {
          game_over = player_winner.is_some();
          if !game_over {
            let ai_result = board.ai_turn();
            match ai_result {
              Ok(ai_winner) => {
                game_over = ai_winner.is_some()
              },
              Err(e) => {
                println!();
                println!("{}", e); //Only error right now is stalemate
                game_over = true
              }
            }
          }
        },
        Err(e) => println!("{}", e),
      }; 
    }
}

fn request_input() -> Option<usize> {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("failed to read line");
  input.pop(); //Remove newline
  let selected_position_result = input.parse::<usize>();
  match selected_position_result {
    Ok(result) => {
      return Some(result)
    },
    Err(_) => {
      println!("Invalid Move, Please Try Again");
      return None
    }
  }
}