/// console - envelope
// ******************************
// *  *                      *  *
// *    *                  *    *
// *      *              *      *
// *        *          *        *
// *          *      *          *
// *            *  *            *
// *             **             *
// *           *    *           *
// *         *        *         *
// *       *            *       *
// *     *                *     *
// *   *                    *   *
// * *                        * *
// ******************************
fn envelope(width: u32, height: u32) {

    let range_y = 1..=height;
    for y in range_y {
        for x in 1..=width {
            let is_hor = y == 1 || y == height;
            let is_ver = x == 1 || x == width;
            let k = width as f32 / height as f32;
            let is_diag = (y as f32 * k) as u32 == x;
            let show = is_hor || is_ver || is_diag;

            let sym = if show { '*' } else { ' ' };
            print!("{sym}");
        }
        print!("\n");
    }
}

#[test]
fn test1() {
    // envelope(10, 10);
    // envelope(20, 10);
    envelope(20, 10);
    envelope(24, 10);
    envelope(27, 10);
    envelope(30, 10);
    // envelope(35, 10);
}

// a > b
// a >= b
// a < b
// a <= b
//
// NEVER !!!!!!!!!!!!! : a == b
// 0.30000000000000004
// 0.30000000000000005


#[test]
fn test2() {
    let x = 0.1;
    let y = 0.2;
    let z = x + y; // 0.30000000000000004
    println!("{z}");
}