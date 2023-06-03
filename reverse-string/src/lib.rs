pub fn reverse(input: &str) -> String {
    let mut stack = input.chars().collect::<Vec<char>>();
    let mut result: String = String::new();

    while let Some(c) = stack.pop() {
        result.push(c);
    }

    result
}
