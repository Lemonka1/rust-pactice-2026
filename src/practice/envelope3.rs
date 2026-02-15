#[test]
fn envelope() {
    const W: u32 = 25; // 20 ... 29
    const H: u32 = 10; // 10
    let k = W as f32 / H as f32; // 2.0 ... 2.9

    for y in 0..H {
        for x in 0..W {
            let is_hor = y == 0 || y == H - 1;
            let is_ver = x == 0 || x == W - 1;

            // TODO
            let is_diag = x == y || y == W - 1 - x;

            let to_show = is_hor || is_ver || is_diag;

            let sym = if to_show { '*' } else { ' ' };
            print!("{sym}");
        }
        println!();
    }
}
