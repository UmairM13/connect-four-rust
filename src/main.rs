use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use std::io::{self, Write};
use std::time::Duration;

type Board = [char; 42];

// Function to display the board
fn display_board(board: &Board) -> String {
    let board_display: String = board
        .chunks(7)
        .map(|row| format!(" {}  ", row.iter().collect::<String>()))
        .collect::<Vec<String>>()
        .join("\n");
    format!("{}\n 1234567", board_display)
}

// Pure function to check if a move is valid
fn valid_move(board: &Board, column: usize) -> bool {
    column < 7 && board[column] == '.'
}

// Pure function to drop a piece and return a new board
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

// Pure function to check for a win
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
// Pure function to handle player input with a timeout
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

// Pure function to handle player input without a timer
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
