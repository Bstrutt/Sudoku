use bit_vec::BitVec;

pub fn check_legal(state: &[[u32; 9]; 9]) -> bool {
    return (check_cols(state) && check_rows(state));
}



fn check_cols(state: &[[u32; 9]; 9]) -> bool{
    let mut duplicates = BitVec::from_elem(10, false);
    for i in 0..9{
        for j in 0..9{
            if state[j][i] == 0{
                //do nothing
            } else {
                if duplicates.get(state[j][i] as usize) == Some(true) {
                    return false;
                } else {
                    duplicates.set(state[j][i] as usize, true);
                    //print!("{}", state[i][j]);
                }
            }
        }
        for i in 0..10{
            duplicates.set(i, false);
        }
    }
    return true;

}

fn check_rows(state: &[[u32; 9]; 9]) -> bool{
    let mut duplicates = BitVec::from_elem(10, false);
   
    for y in state.iter(){
        for x in y{
            if *x == 0{
                //do nothing
            }
            else { 
                if duplicates.get(*x as usize) == Some(true) {
                    return false;
                } else {
                    duplicates.set(*x as usize, true);
                }
            }
        }
        for i in 0..10{
            duplicates.set(i, false);
        }
    }
    return true;
}
//fn check_fams(state: &[[u32; 9]; 9]){}
//pub fn check_solved(state: &[[u32; 9]; 9]){}