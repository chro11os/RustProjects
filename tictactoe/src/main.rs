use std::io;
fn drawBoard(board: &[[char; 3]; 3]) {
    let i:i32 = 3;
    let j:i32 = 3;

    println!("-------------");

    for i in i {
        println!("| ");
        for j in j {
            drawBoard(board[i:usize][j:usize]);
        }
    }
    println!("-------------");
}
fn main() {}
