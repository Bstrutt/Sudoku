mod display;
use display :: print_board;
fn main() {
    let mut state = [[0; 9]; 9];
    state[0] = [0,0,0, 0,0,0, 0,0,0];
    state[1] = [0,0,0, 0,0,0, 0,0,0];
    state[2] = [0,0,0, 0,0,0, 0,0,0];
    state[3] = [0,0,0, 0,0,0, 0,0,0];
    state[4] = [0,0,0, 0,0,0, 0,0,0];
    state[5] = [0,0,0, 0,0,0, 0,0,0];
    state[6] = [0,0,0, 0,0,0, 0,0,0];
    state[7] = [0,0,0, 0,0,0, 0,0,0];
    state[8] = [0,0,3, 6,4,2, 7,0,1];
    
    print_board(&state);
}

