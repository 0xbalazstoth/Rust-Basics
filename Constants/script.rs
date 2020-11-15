const YOU_CANT_CHANGE_ME:u8 = 0;
static mut z:i32 = 1; //Static mut is unsafe

fn main() {
    println!("{}", YOU_CANT_CHANGE_ME);
    
    unsafe {
        println!("Unsafe: {}", z);
    }
}