use bit_vec::BitVec;

pub fn check_legal(state: &[[u32; 9]; 9]) -> bool {
    return check_cols(state) && check_rows(state) && check_fams(state);
}

pub fn check_solved(state: &[[u32; 9]; 9]) -> bool{
    return check_legal(state) && check_nonzero(state);
}


fn check_nonzero(state: &[[u32; 9]; 9]) -> bool{
    for i in 0..9{
        for j in 0..9{
            if state[i][j] == 0{
                return false;
            }
        }
    }
    return true;
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
fn check_fams(state: &[[u32; 9]; 9]) -> bool{
    let mut duplicates = BitVec::from_elem(10, false);
    for l in 0..3{
        for k in 0..3{
            for i in 0..3{
                for j in 0..3{
                    let x = state[l*3+i][k*3+j];
                    if x == 0 {
                        //do nothing
                    } else {
                        if duplicates.get(x as usize) == Some(true){
                            return false;
                        } else {
                            duplicates.set(x as usize, true);
                        }
                    }

                }
            }
            for i in 0..10{
                duplicates.set(i, false);
            }
        }
    }
    return true;
}
