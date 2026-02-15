#[test]
fn envelope_test() {
    const W: u32 = 25;
    const H: u32 = 10;
    let k = W as f32 / H as f32; // 2.5
    for y in 0..H {
        for x in 0..W {
            let row1 = y == 0;
            let rowN = y == H - 1;
            let col1 = x == 0;
            let colN = x == W - 1;
            let is_hor = row1 || rowN;
            let is_ver = col1 || colN;
            let yk = (y as f32 * k) as u32;
            let is_diag1 = x == yk;
            let is_diag2 = x == W - 1 - yk;

            let sym = if is_hor || is_ver || is_diag1 || is_diag2 {
                "*"
            } else {
                " "
            };
            print!("{}", sym);
        }
        println!();
    }
}

#[test]
fn envelope_test2() {
    const W: u32 = 25;
    const H: u32 = 10;
    let k = W as f32 / H as f32;

    let yk = |y: u32| (y as f32 * k) as u32;

    for y in 1..=H {
        for x in 1..=W {
            let sym: char = match (x, y) {
                (_, 1) => '*',
                (_, H) => '*',
                (1, _) => '*',
                (W, _) => '*',
                _ if x == yk(y) => '*',
                _ if x == W - yk(y) => '*',
                _ => ' ',
            };
            print!("{}", sym);
        }
        println!();
    }
}
