use bit_vec::BitVec;

pub fn solve(state: &mut [[u32; 9]; 9]) ->() {
    let dummy = BitVec::from_elem(10,true);
    let mut poss_matrix = [[& dummy; 9]; 9];

    

    for i in 0..9{
        for j in 0..9{
            let cur_num = state[i][j];
            if cur_num != 0 {
                fill_row(&mut poss_matrix, i, cur_num);
                fill_col(&mut poss_matrix, j, cur_num);
                fill_fam(&mut poss_matrix, i, j, cur_num);
            }
        }
    }
}

fn fill_row(poss_matrix: &mut [[BitVec; 9]; 9], i: usize, cur_num: u32){
    for k in 0..9{
        poss_matrix[i][k].set(cur_num as usize, false);
    }
}
fn fill_col(poss_matrix: &mut [[BitVec; 9]; 9], j: usize, cur_num: u32){
    for k in 0..9{
        poss_matrix[k][j].set(cur_num as usize, false);
    }
}
fn fill_fam(poss_matrix: &mut [[BitVec; 9]; 9], i: usize, j: usize, cur_num: u32){
    let k = i/3;
    let l = j/3;
    for x in 0..3{
        for y in 0..3{
            poss_matrix[k*3+x][l*3+y].set(cur_num as usize, false);
        }
    }
}