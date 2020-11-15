fn scope_and_shadowing() {
    let a = 123;
    println!("a = {}", a);

    {
        let b = 456;
        println!("inside, b = {}", b);
    }
    // println!("inside, b = {}", b); //Nope, b is inside, you can't call outside
}

fn main() {
    scope_and_shadowing();
}