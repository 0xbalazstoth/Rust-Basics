fn litres(time: f64) -> i32 {
    return (time * 0.5) as i32;
}

fn main() {
    println!("{}", litres(6.7));
}