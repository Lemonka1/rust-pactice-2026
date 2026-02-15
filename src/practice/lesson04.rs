/// console - rhombus
//      *
//     ***
//    *****
//   *******
//  *********
// ***********
//  *********
//   *******
//    *****
//     ***
//      *

//      *
//     **
//    ***
//   ****
//  *****
// ******
fn rhombus1(size: u32) {
    for y in 0..size {
        for x in 0..size {
            let c = if x + y >= size - 1 { "*" } else { " " };
            print!("{c}");
        }
        println!();
    }
}
// *
// **
// ***
// ****
// *****
// ******
fn rhombus2(size: u32) {
    for y in 0..size {
        for x in 0..size {
            let c = if mirror(x, size) + y >= size { "*" } else { " " };
            print!("{c}");
        }
        println!();
    }
}
// ******
//  *****
//   ****
//    ***
//     **
//      *
fn rhombus3(size: u32) {
    for y in 0..size {
        for x in 0..size {
            let c = if x + mirror(y, size) >= size { "*" } else { " " };
            print!("{c}");
        }
        println!();
    }
}

// ******
// *****
// ****
// ***
// **
// *
fn rhombus4(size: u32) {
    for y in 0..size {
        for x in 0..size {
            let c = if mirror(x, size) + mirror(y, size) > size { "*" } else { " " };
            print!("{c}");
        }
        println!();
    }
}

#[test]
fn test1() {
    rhombus1(6);println!();
    rhombus2(6);println!();
    rhombus3(6);println!();
    rhombus4(6);
}

fn mirror(x: u32, size: u32) -> u32 {
    size - x
}

#[test]
fn mirror_test() {
    let size = 6;
    let test_data: [(u32, u32); 7] = [(0, 6), (1, 5), (2, 4), (3, 3), (4, 2), (5, 1), (6, 0)];

    for (a, b) in test_data {
        let m = mirror(a, size);
        assert_eq!(m, b);
    }
}
