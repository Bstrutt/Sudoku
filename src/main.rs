mod display;
mod checks;
use display :: print_board;
fn main() {
    let mut state = [[0; 9]; 9];
    state[0] = [1,0,0, 0,0,0, 0,0,0];
    state[1] = [0,2,0, 0,0,0, 0,0,0];
    state[2] = [0,0,3, 0,0,0, 0,0,0];
    state[3] = [0,0,0, 4,0,0, 0,0,0];
    state[4] = [0,0,0, 0,5,0, 0,0,0];
    state[5] = [0,0,0, 0,0,6, 0,0,0];
    state[6] = [0,0,0, 0,0,0, 0,0,0];
    state[7] = [0,0,0, 0,0,0, 0,0,0];
    state[8] = [0,0,0, 6,4,0, 7,0,1];
   
    print_board(&state);

    if check_legal(&state) {
        print!("\nBoard is legal");
    } else {
        print!("Board is not legal");
    }
    
}

