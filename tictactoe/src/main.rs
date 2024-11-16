use std::io;
fn drawBoard(board: &[[char; 3]; 3]) {
    let mut range = 1..3;

    println!("-------------");

    for i in range {
        println!("| ");
        for j in range {
            drawBoard(board[i:usize][j:usize]);
        }
    }
    println!("-------------");
}
fn main() {
    drawBoard(&[]);
}
