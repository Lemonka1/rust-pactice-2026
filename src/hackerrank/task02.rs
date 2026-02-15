fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test1() {
    let x = 5;
    let y = 10;
    let real = add(5,10);
    let expected = 15;

    assert_eq!(real, expected);
}
