use std::io;
fn drawBoard(board: &[[char; 3]; 3]) {
    let mut BoardRange = 1..3;

    println!("-------------");

    for i in BoardRange {
        println!("| ");
        for j in BoardRange {
            println!("{}", board[i][j]);
        }
    }

    println!("-------------");
}

fn main() {
    drawBoard();
}
