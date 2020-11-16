fn main() {
    let tup1 = (10, 15, 20, 25, "Rust is safe, blazing fast", (1,5, 76, 45)); //Tuple
    println!("{}", (tup1.5).2);

    let tup2 = (1, 2, 3);
    let (f, s, t) = tup2;
    println!("{}", f);
    println!("{}", s);
    println!("{}", t);
}