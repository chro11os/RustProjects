use std::{ops::Range};

fn drawBoard(board: &[[char; 3]; 3]) {
    let BoardRange: Range<usize> = 1..3;
    
    println!("-------------");
    for i in &BoardRange {
        println!("| ", );
        for j in &BoardRange {
            println!("{}", board[i][j]);
        }
    }
    println!("-------------");
}
fn checkWin(board: &[[char; 3]; 3], player: char) -> bool {
    let PlayerRange: Range<usize> = 1..3;
    if board[&PlayerRange][0] == player
        && board[&PlayerRange][1] == player
        && board[&PlayerRange][2] == player
    {
        return true;
    } else {
        return false;
    }
}

fn main() {}
