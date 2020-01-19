mod display;
mod checks;
mod solver;

use display :: print_board;
use solver::solve;
fn main() {
    let mut state = [[0; 9]; 9];
    state[0] = [0,0,0, 0,0,0, 0,0,0];
    state[1] = [0,0,0, 0,0,0, 0,0,0];
    state[2] = [0,0,0, 0,0,1, 0,0,0];
    state[3] = [0,0,0, 0,0,0, 0,0,0];
    state[4] = [0,0,0, 0,0,0, 0,0,0];
    state[5] = [0,0,0, 0,0,0, 0,0,0];
    state[6] = [0,0,0, 0,0,0, 0,0,0];
    state[7] = [0,0,0, 0,0,0, 0,0,0];
    state[8] = [0,0,0, 0,0,0, 0,0,0];
   
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

