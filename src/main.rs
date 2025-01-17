use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use std::io::{self, Write};
use std::time::Duration;

// have the board as being immutable so that the ownership is not transferred.
type Board = [char; 42];

// Chunks(7) splits the board into slices of 7 elements each representing a column
// map applies afunction to each row, and row.iter will iterate over each element in the row
// collect::<String>, collecs the character in the row into a single string
// format! will format each row into a string. The {} is filled by the joined row string
//.collect, collects all the formatted row strings into a vector of strings
//.join will join all strings in the vector with new line characters to represent the board
fn display_board(board: &Board) -> String {
    let board_display: String = board
        .chunks(7)
        .map(|row| format!(" {}  ", row.iter().collect::<String>()))
        .collect::<Vec<String>>()
        .join("\n");
    format!("{}\n 1234567", board_display)
}

// Deterministic, always return the same result without altering input
// It receives an immutable reference to Board, ensuring it cannot change.
fn valid_move(board: &Board, column: usize) -> bool {
    column < 7 && board[column] == '.'
}

// column: uszie, column index where the piece will be dropped
//(0..6) generates row indices in revers order from bottom to top
//.find_map iterates over the rows to find the first empty cell in the specified column
// then a new board is created and updated accordingly and the new board is returned
fn drop_piece(board: &Board, column: usize, player: char) -> Board {
    (0..6)
        .rev()
        .find_map(|i| {
            let index = i * 7 + column;
            if board[index] == '.' {
                let mut new_board = *board;
                new_board[index] = player;
                return Some(new_board);
            }
            None
        })
        .expect("Column is full")
}

// This uses iterator-based loops instead of traditional for loops.
// It also combines the results of the four checks
// Essentially it works the same as the normal python solution
// but takes on a more functional appraoch.
// .any and .all are higher order functions to check conditions
fn check_win(board: &Board, player: char) -> bool {
    // Horizontal, vertical, and diagonal checks as before
    (0..6).any(|row| {
        (0..4).any(|col| {
            let start = row * 7 + col;
            (0..4).all(|i| board[start + i] == player)
        })
    }) || (0..3).any(|row| {
        (0..7).any(|col| {
            let start = row * 7 + col;
            (0..4).all(|i| board[start + i * 7] == player)
        })
    }) || (0..3).any(|row| {
        (0..4).any(|col| {
            let start = row * 7 + col;
            (0..4).all(|i| board[start + i * 8] == player)
        })
    }) || (0..3).any(|row| {
        (0..4).any(|col| {
            let start = (5 - row) * 7 + col;
            (0..4).all(|i| board[start - i * 6] == player)
        })
    })
}

// Pure function to check if the board is full
fn is_full(board: &Board) -> bool {
    !board.contains(&'.')
}

// AI move function (pure)
fn ai_move(board: &Board) -> Board {
    let mut rng = rand::thread_rng();

    // Loop until a valid move is found
    loop {
        let column = rng.gen_range(0..7); // Randomly select a column
        if valid_move(board, column) {
            // Find the first available row in the column
            for row in (0..6).rev() {
                let index = row * 7 + column;
                if board[index] == '.' {
                    let mut new_board = *board;
                    new_board[index] = 'B'; // Drop the AI's piece
                    return new_board;
                }
            }
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
fn get_player_input_timeout(_board: &Board) -> Option<usize> {
    println!("You have 5 seconds to enter a column (1-7) or 0 to exit: ");
    let timeout = Duration::from_secs(5);
    if event::poll(timeout).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Char('0') => None,
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

// std::io::stdin....
// handles the user input and stores it in the input variable
// the input is then trimmed to remove any whitespace
// has error handling as well
// match input.trim()...
// tries to parse the input into a usize (unsigned integer)
// which reperesents the column that the player wants to drop the piece
// _ is a catch all for any other value that is not 0 or a number between 1 and 7
fn get_player_input(board: &Board) -> Option<usize> {
    print!("Enter a column (1-7) or 0 to exit: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<usize>() {
        Ok(0) => None,
        Ok(col) if col >= 1 && col <= 7 => {
            if valid_move(board, col - 1) {
                Some(col - 1)
            } else {
                println!("Invalid move. Try again.");
                get_player_input(board)
            }
        }
        _ => {
            println!("Invalid input. Try again.");
            get_player_input(board)
        }
    }
}

// Main game loop for player vs player (no timer)
fn game_loop<F>(input_fn: F)
where
    F: Fn(&Board) -> Option<usize>,
{
    let mut board: Board = ['.'; 42];
    let mut turn = 'R';
    let mut game_over = false;

    while !game_over {
        println!("{}", display_board(&board));

        match input_fn(&board) {
            Some(column) => board = drop_piece(&board, column, turn),
            None => {
                println!("Exiting game.");
                break;
            }
        }

        if check_win(&board, turn) {
            println!("{}", display_board(&board));
            println!("Player {} wins!", turn);
            game_over = true;
        } else if is_full(&board) {
            println!("{}", display_board(&board));
            println!("It's a tie!");
            game_over = true;
        }

        turn = if turn == 'R' { 'B' } else { 'R' };
    }
}

// Main game loop for player vs ai (no timer)
fn game_loop_ai<F>(input_fn: F)
where
    F: Fn(&Board) -> Option<usize>,
{
    let mut board: Board = ['.'; 42];
    let mut turn = 'R';
    let mut game_over = false;

    while !game_over {
        println!("{}", display_board(&board));

        if turn == 'R' {
            match input_fn(&board) {
                Some(column) => board = drop_piece(&board, column, turn),
                None => {
                    println!("Exiting game.");
                    break;
                }
            }
        } else {
            board = ai_move(&board);
        }

        if check_win(&board, turn) {
            println!("{}", display_board(&board));
            println!("Player {} wins!", turn);
            game_over = true;
        } else if is_full(&board) {
            println!("{}", display_board(&board));
            println!("It's a tie!");
            game_over = true;
        }

        turn = if turn == 'R' { 'B' } else { 'R' };
    }
}

// Main game loop for player vs ai with timer
fn game_loop_ai_timeout<F>(input_fn: F)
where
    F: Fn(&Board) -> Option<usize>,
{
    let mut board: Board = ['.'; 42];
    let mut turn = 'R';
    let mut game_over = false;

    while !game_over {
        println!("{}", display_board(&board));

        if turn == 'R' {
            match input_fn(&board) {
                Some(column) => board = drop_piece(&board, column, turn),
                None => {
                    println!("Exiting game.");
                    break;
                }
            }
        } else {
            board = ai_move(&board);
        }

        if check_win(&board, turn) {
            println!("{}", display_board(&board));
            println!("Player {} wins!", turn);
            game_over = true;
        } else if is_full(&board) {
            println!("{}", display_board(&board));
            println!("It's a tie!");
            game_over = true;
        }

        turn = if turn == 'R' { 'B' } else { 'R' };
    }
}

fn main() {
    println!("Connect 4!");
    println!("1. Play against another player");
    println!("2. Play against AI");
    println!("3. Play against AI with timed player moves");
    println!("Please select an option (0 to exit): ");

    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim().parse::<u32>() {
        Ok(0) => println!("Game exited. Thanks for playing!"),
        Ok(1) => game_loop(get_player_input),
        Ok(2) => game_loop_ai(get_player_input),
        Ok(3) => game_loop_ai_timeout(get_player_input_timeout),
        _ => println!("Invalid choice"),
    }
}
