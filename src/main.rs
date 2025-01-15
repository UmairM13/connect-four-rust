use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use std::time::Duration;

// const BOARD: &[char; 42] = &['.'; 42];
// The reason why we don't do the above is because we can't index into a slice with a usize
// We can only index into an array with a usize
// We also need the board to be mutable

fn display_board(board: &[char; 42]) {
    for row in board.chunks(7) {
        println!(" {}  ", row.iter().collect::<String>());
    }

    println!(" 1234567");
}

fn valid_move(board: &mut [char; 42], column: usize) -> bool {
    if board[column] != '.' {
        false
    } else {
        true
    }
}

fn drop_piece(board: &mut [char; 42], column: usize, player: char) {
    for i in (0..6).rev() {
        let index = i * 7 + column;
        if board[index] == '.' {
            board[index] = player;
            break;
        }
    }
}

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

fn take_turn(board: &mut [char; 42], player: char) -> bool {
    loop {
        display_board(board);
        let column = match get_player_input(board) {
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

fn game_loop_ai() {
    let mut board = ['.'; 42];
    loop {
        if !take_turn(&mut board, 'R') {
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

        ai_move(&mut board);
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

fn game_loop() {
    let mut board = ['.'; 42];
    loop {
        if !take_turn(&mut board, 'R') {
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

        if !take_turn(&mut board, 'B') {
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

fn get_player_input_timeout() -> Option<usize> {
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

fn game_loop_ai_timeout() {
    let mut board = ['.'; 42];
    for player in ["R", "B"].iter().cycle().take(43) {
        display_board(&board);

        if *player == "R" {
            match get_player_input_timeout() {
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

    match choice.trim().parse::<u32>() {
        Ok(0) => println!("Game exited. Thanks for playing!"),
        Ok(1) => game_loop(),
        Ok(2) => game_loop_ai(),
        Ok(3) => game_loop_ai_timeout(),

        _ => println!("Invalid choice"),
    }
}
