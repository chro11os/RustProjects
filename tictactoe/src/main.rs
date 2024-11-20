use std::{io, ops::Range};

fn drawBoard(board: &[[char; 3]; 3]) {
    let mut BoardRange: Range<usize> = 1..3;

    println!("-------------");

    for i in BoardRange.clone() {
        println!("| ");
        for j in BoardRange.clone() {
            println!("{}", board[i][j]);
        }
    }

    println!("-------------");
}

fn main() {}
