use std::io;

fn main() {
    // Initialize the board and players
    let mut board = [[' '; 3]; 3]; // 2D array of 3x3 initialized with spaces
    let mut player = 'X';

    println!("Welcome to Tic-Tac-Toe!");

    // Game loop for 9 turns
    for turn in 0..9 {
        // Draw the board
        draw_board(&board);

        // Prompt for valid input
        let (row, col) = loop {
            println!("Player {}, enter row (0-2) and column (0-2): ", player);

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");

            // Parse the input into two integers (row and col)
            if let Some((r, c)) = parse_input(&input) {
                if r < 3 && c < 3 && board[r][c] == ' ' {
                    break (r, c); // Valid input, exit the loop
                }
            }

            println!("Invalid move. Try again.");
        };

        // Make the move
        board[row][col] = player;

        // Check for a win after making a move
        if check_win(&board, player) {
            draw_board(&board);
            println!("Player {} wins!", player);
            return; // Exit the game after a win
        }

        // Switch to the other player
        player = if player == 'X' { 'O' } else { 'X' };
    }

    // Draw the final board
    draw_board(&board);

    // Check for a draw
    println!("It's a draw!");
}

// Function to draw the Tic-Tac-Toe board
fn draw_board(board: &[[char; 3]]) {
    println!("-------------");
    for row in board {
        print!("| ");
        for &cell in row {
            print!("{} | ", cell);
        }
        println!("\n-------------");
    }
}

// Function to check for a win
fn check_win(board: &[[char; 3]], player: char) -> bool {
    // Check rows and columns
    for i in 0..3 {
        if (board[i][0] == player && board[i][1] == player && board[i][2] == player) ||
           (board[0][i] == player && board[1][i] == player && board[2][i] == player) {
            return true;
        }
    }
    // Check diagonals
    if (board[0][0] == player && board[1][1] == player && board[2][2] == player) ||
       (board[0][2] == player && board[1][1] == player && board[2][0] == player) {
        return true;
    }
    false
}

// Function to parse user input as (row, col)
fn parse_input(input: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() == 2 {
        if let (Ok(row), Ok(col)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
            return Some((row, col));
        }
    }
    None
}
