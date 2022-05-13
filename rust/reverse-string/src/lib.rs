pub fn reverse(input: &str) -> String {
    let mut s = String::new();
    let chars = input.chars().rev();

    for c in chars {
        s = format!("{}{}", s, c)
    }
    s
}
