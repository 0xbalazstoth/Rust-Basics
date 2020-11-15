fn make_upper_case(s: &str) -> String {
    return s.to_uppercase().to_string();
}

fn main() {
    println!("{}", make_upper_case("asd"));
}