use std::cmp::min;

fn parking_roof(cars: &[u32], k: usize) -> u32 {
    let mut cars = cars.to_vec();
    cars.sort();

    let roof_size = |idx: usize| cars[idx + 2] - cars[idx] + 1;

    let mut m = roof_size(0);

    for idx in 1..cars.len() - k + 1 {
        let size = roof_size(idx);
        m = min(m, size);
    }

    m
}

#[test]
fn test1() {
    let cars = [6, 2, 12, 7];
    let k = 3;
    let roof_expected = 6;
    let roof_real = parking_roof(&cars, k);
    assert_eq!(roof_expected, roof_real);
}
