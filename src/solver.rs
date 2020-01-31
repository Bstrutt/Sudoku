use crate::checks;
use crate::display;
pub fn solve(state: &mut [[u32; 9]; 9]) ->() {
    solve_cell(state, 0, 0);
}

fn solve_cell(state: &mut [[u32; 9]; 9], x: usize, y: usize) -> bool{

    
    if state[x][y]!=0 { 
        return solve_next(state, x, y);
    } else { // if there isnt already something in the cell
        for i in 1..10{
            state[x][y] = i;
            display::print_board(state);
            if checks::check_legal(&state) { // check new state for legality
                if solve_next(state, x, y) {
                    return true;   
                } else {
                    continue;
                }
            } else {
                continue; //if not legal try the next number
            }
        }
       state[x][y] = 0;
        return false;
    }
}
fn solve_next(state: &mut [[u32; 9]; 9], x: usize, y: usize) -> bool{
    if x==8{
        if y==8{
            return true;
        } else {
            return solve_cell(state, x, y+1);
        }
    } else {
        if y==8{
            return solve_cell(state, x+1, 0);
        } else {
            return solve_cell(state, x, y+1);
        }
    } 
}
