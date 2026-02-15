#[test]
fn envelope() {
    const W: u32 = 25;
    const H: u32 = 10;

    for y in 0..H {
        for x in 0..W {
            let is_hor = y == 0 || y == H - 1;
            let is_ver = x == 0 || x == W - 1;
            let is_diag = x == y;
            let is_diag2 = y == W - 1 - x;
            let to_show = is_hor || is_ver || is_diag || is_diag2;

            let sym = if to_show { '*' } else { ' ' };
            print!("{}", sym);
        }
        println!();
    }
}
