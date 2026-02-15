/// simple brackets_is_valid
fn is_valid(s: &str) -> bool {
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

// {}     - t
// {}{}   - t
// {{}}   - t
// {{}{}} - t
//
// {{}    - f
// {}}    - f
// }{     - f
// {      - f
// }      - f
#[test]
fn test() {
    let test_data = [
        ("{}", true),
        ("{}{}", true),
        ("{{}}", true),
        ("{{}{}}", true),

        ("{{}{}{}{{}}", false),
        ("{{}", false),
        ("{}}", false),
        ("}{", false),
        ("{", false),
        ("}", false),
    ];

    for (input, expected) in test_data {
        let real = is_valid(input);
        println!("input: {input:20} expected: {expected:5} real: {real:5}");
        assert_eq!(real, expected);
    }
}
