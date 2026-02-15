fn validate(s: &str) -> bool {
    let mut count = 0;
    for c in s.chars() {
        match c {
            '{' => count += 1,
            '}' if count == 0 => return false,
            '}' => count -= 1,
            _ => {}
        }
    }
    count == 0
}

#[test]
fn validation_test() {
    let test_data: [(&str, bool); 10] = [
        // valid
        // input, output
        ("", true),
        ("{}", true),
        ("{}{}", true),
        ("{{}}", true),
        ("{{}{}}", true),
        // invalid
        ("{{}", false),
        ("{}}", false),
        ("}{", false),
        ("{", false),
        ("}", false),
    ];

    for (input, expected) in test_data {
        let real = validate(input);
        println!("input: {input}, real: {real}, expected: {expected}");
        assert_eq!(real, expected);
    }
}

#[test]
fn test() {
    let s = "
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
            print!(\"{}\", sym);
        }
        println!();
    }
}
    ";

    let is_valid = validate(&s.to_string());
    print!("{is_valid}");
}
