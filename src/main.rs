mod display;
mod checks;
mod solver;

use display :: print_board;
use solver::solve;
fn main() {
    let mut state = [[0; 9]; 9];
    state[0] = [0,0,0, 0,0,0, 2,0,0];
    state[1] = [0,8,0, 0,0,7, 0,9,0];
    state[2] = [6,0,2, 0,0,0, 5,0,0];
    state[3] = [0,7,0, 0,6,0, 0,0,0];
    state[4] = [0,0,0, 9,0,1, 0,0,0];
    state[5] = [0,0,0, 0,2,0, 0,4,0];
    state[6] = [0,0,5, 0,0,0, 6,0,3];
    state[7] = [0,9,0, 4,0,0, 0,7,0];
    state[8] = [0,0,6, 0,0,0, 0,0,0];
   
    solve(&mut state);
    print_board(&state);

    if checks::check_legal(&state) {
        print!("\nBoard is legal. ");
    } else {
        print!("Board is not legal. ");
    }

    if checks::check_solved(&state) {
        print!("\nBoard is solved. ");
    } else {
        print!("Board is not solved. ");
    }

}

