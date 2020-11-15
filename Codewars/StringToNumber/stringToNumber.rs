fn string_to_number(s: &str) -> i32 {
    let num:i32 = s.trim().parse().unwrap();
    return num;
}

fn main() {
    println!("{}", string_to_number("10"));
}