pub fn print_board(state: &[[u32; 9]; 9] ){
    let mut ycount = 0;
    for y in state.iter() {
        ycount = ycount + 1;
        let mut xcount = 0;
        for x in y {
            xcount = xcount + 1;
            if *x == 0{
                print!("0");
            } else {
                print!("{}", *x);
            }
            if xcount%3 == 0 && xcount%9 != 0{
                print!("|");
            }
        }
        if ycount%3 == 0 && ycount%9 != 0{
            print!("\n------------")
        }
        print!("\n")
    }
}
