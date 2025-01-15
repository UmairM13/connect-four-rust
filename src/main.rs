use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use std::time::Duration;

// const BOARD: &[char; 42] = &['.'; 42];
// The reason why we don't do the above is because we can't index into a slice with a usize
// We can only index into an array with a usize
// We also need the board to be mutable

// {} is a placeholder for the value of the variable
// board.chunks pretty much splits the board into 7 chunks, one for each row
//.iter is to iterator or go through each element in the row
// .collect::<String>() is to collect all the elements in the row into a string
fn display_board(board: &[char; 42]) {
    for row in board.chunks(7) {
        println!(" {}  ", row.iter().collect::<String>());
    }

    println!(" 1234567");
}

// returns type is boolean.
// the board type is 42 characters and is mutable because we are going to change the board
fn valid_move(board: &mut [char; 42], column: usize) -> bool {
    board[column] == '.'
}

// the .rev() is to reverse the range so that we can drop the pieces at
// the bottom of the board
fn drop_piece(board: &mut [char; 42], column: usize, player: char) {
    for i in (0..6).rev() {
        let index = i * 7 + column;
        if board[index] == '.' {
            board[index] = player;
            break;
        }
    }
}

// F is a generic type that takes a closure as an argument
// It takes a mutable reference to the board, a character and a closure
// the reference reperesents the column that the player wants to drop the piece
fn take_turn<F>(board: &mut [char; 42], player: char, input_fn: F) -> bool
where
    F: Fn(&[char; 42]) -> Option<usize>,
{
    loop {
        display_board(board);
        let column = match input_fn(board) {
            Some(column) => column,
            None => {
                println!("Exiting game");
                return false;
            }
        };

        if valid_move(board, column) {
            drop_piece(board, column, player);
            return true;
        } else {
            println!("Please enter a valid column (1-7) or 0 to exit");
        }
    }
}

// Essentially the same as python code.
fn check_win(board: &[char; 42], player: char) -> bool {
    // Check for horizontal win
    for row in 0..6 {
        for col in 0..4 {
            let start = row * 7 + col;
            if board[start] == player
                && board[start + 1] == player
                && board[start + 2] == player
                && board[start + 3] == player
            {
                return true;
            }
        }
    }

    // Check for vertical win
    for row in 0..3 {
        for col in 0..7 {
            let start = row * 7 + col;
            if board[start] == player
                && board[start + 7] == player
                && board[start + 14] == player
                && board[start + 21] == player
            {
                return true;
            }
        }
    }

    // Check for diagonal win bottom left to top right
    for row in 0..3 {
        for col in 0..4 {
            let start = row * 7 + col;
            if board[start] == player
                && board[start + 8] == player
                && board[start + 16] == player
                && board[start + 24] == player
            {
                return true;
            }
        }
    }

    // Check for diagonal win top left to bottom right
    for row in 0..3 {
        for col in 0..4 {
            let start = (5 - row) * 7 + col;
            if board[start] == player
                && board[start - 6] == player
                && board[start - 12] == player
                && board[start - 18] == player
            {
                return true;
            }
        }
    }

    false
}

fn is_full(board: &[char; 42]) -> bool {
    !board.contains(&'.')
}

// mut rng is a mutable variable that is a random number generator
// loop is an infinite loop
// column is a random number between 0 and 7
// if the move is valid, drop the piece and return true
fn ai_move(board: &mut [char; 42]) -> bool {
    let mut rng = rand::thread_rng();
    loop {
        let column = rng.gen_range(0..7);
        if valid_move(board, column) {
            drop_piece(board, column, 'B');
            return true;
        }
    }
}

// std::io::stdin....
// handles the user input and stores it in the input variable
// the input is then trimmed to remove any whitespace
// has error handling as well
// match input.trim()...
// tries to parse the input into a usize (unsigned integer)
// which reperesents the column that the player wants to drop the piece
// _ is a catch all for any other value that is not 0 or a number between 1 and 7
fn get_player_input(_board: &[char; 42]) -> Option<usize> {
    loop {
        println!("Enter a column (1-7) or 0 to exit: ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(0) => return None,
            Ok(column) if column >= 1 && column <= 7 => {
                return Some(column - 1);
            }
            _ => println!("Please enter a valid column (1-7) or 0 to exit"),
        }
    }
}

// _board is a reference to the board
// it takes the board though it is not used
// event::poll(timeout).unwrap()
// checks if there is any user input within the timeout period
// unwrap is to handle any errors
// Event::Key is used to check if any key is pressed
// And Yes it is a bit finicky, also that means
// that the user doesn't have to press enter after entering the column number
// luckily in connect four there is only 7 columns so therefore
// the user can only enter one digit, it doesn't really work
// with more than one digit because it enters after the first key press
fn get_player_input_timeout(_board: &[char; 42]) -> Option<usize> {
    println!("You have 5 seconds to enter a column (1-7) or 0 to exit: ");
    let timeout = Duration::from_secs(5);

    if event::poll(timeout).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Char('0') => None, // Exit the game
                KeyCode::Char(c) if c >= '1' && c <= '7' => {
                    Some(c.to_digit(10).unwrap() as usize - 1)
                }
                _ => {
                    println!("Invalid input.");
                    None
                }
            }
        } else {
            None
        }
    } else {
        println!("Time's up!");
        None
    }
}

fn game_loop<F>(input_fn: F)
where
    F: Fn(&[char; 42]) -> Option<usize>,
{
    let mut board = ['.'; 42];
    loop {
        if !take_turn(&mut board, 'R', &input_fn) {
            break;
        }
        if check_win(&board, 'R') {
            display_board(&board);
            println!("Red wins!");
            break;
        }
        if is_full(&board) {
            display_board(&board);
            println!("It's a tie!");
            break;
        }

        if !take_turn(&mut board, 'B', &input_fn) {
            break;
        }
        if check_win(&board, 'B') {
            display_board(&board);
            println!("Blue wins!");
            break;
        }
        if is_full(&board) {
            display_board(&board);
            println!("It's a tie!");
            break;
        }
    }
}

fn game_loop_ai(input_fn: fn(&[char; 42]) -> Option<usize>) {
    let mut board = ['.'; 42];
    for player in ["R", "B"].iter().cycle().take(43) {
        display_board(&board);

        if *player == "R" {
            match input_fn(&board) {
                Some(column) if valid_move(&mut board, column) => {
                    drop_piece(&mut board, column, 'R')
                }
                Some(_) | None => {
                    println!("Exiting game.");
                    break;
                }
            }
        } else {
            ai_move(&mut board);
        }

        display_board(&board);

        if check_win(&board, player.chars().next().unwrap()) {
            println!("Player {} wins!", player);
            break;
        }
        if is_full(&board) {
            println!("It's a tie!");
            break;
        }
    }
}

fn main() {
    println!("Connect 4!");
    println!("1. Play against another player");
    println!("2. Play against AI");
    println!("3. Play against a harder AI with timed player moves");
    println!("Please select an option (0 to exit): ");

    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    // matches the choice that the user makes and runs the corresponding game
    match choice.trim().parse::<u32>() {
        Ok(0) => println!("Game exited. Thanks for playing!"),
        Ok(1) => game_loop(get_player_input),
        Ok(2) => game_loop_ai(get_player_input),
        Ok(3) => game_loop(get_player_input_timeout),
        _ => println!("Invalid choice"),
    }
}
