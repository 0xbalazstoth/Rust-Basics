use std::mem; //Memory package

fn main() {
    //unsigned 0.. 255 +, != -
    let a:u32 = 123; //32 bits, not changeble(inmutable)
    println!("a = {}", a); //To print out number, need to use "{}"
    //a = 456; nope

    //signed -,+
    let mut b:i32 = 17; //32 bits, changeble(mutable)
    println!("b = {}", b);
    b = 27; // yapp
    println!("CHANGED b = {}", b);

    let mut c = 123456789;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}, after modification", c);

    //Size
    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8); //ex: 64 Bit

    //Char
    let d:char = 'x'; //Or without char
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    //Non-whole numbers
    let e = 2.5; //double
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    //Boolean, true: false
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));

    //Inline condition
    let f = 4 > 1; // true
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));
}