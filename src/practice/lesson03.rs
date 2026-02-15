/// console - envelope
fn envelope(w: u32, h: u32) {
    // W = 20..29
    // H = 10
    let k: f32 = w as f32 / h as f32; // k = 2.9

    for y in 0..h {
        for x in 0..w {
            let is_hor = y == 0 || y == h - 1;
            let is_ver = x == 0 || x == w - 1;
            let id_diag1 = x == (y as f32 * k) as u32;
            let id_diag2 = false; //x == w - 1 - y ;

            let c = if is_hor || is_ver || id_diag1 || id_diag2 {
                '*'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}

/// console - rhombus
#[test]
fn test1() {
    // envelope(10, 10);
    // println!("---");
    envelope(29, 10);
    // println!("---");
    // envelope(35, 10);
    // println!("---");
}

#[test]
fn test33() {
    let x = 0.1 + 0.2;
    println!("{}", x); // 0.30000000000000004
    println!("{}", x == 3.0) // false
}

#[test]
fn test34() {
    let a: f32 = 16777216_f32;
    let b = a + 1_f32;
    let c = b + 1_f32;
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}
