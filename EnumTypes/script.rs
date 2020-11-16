enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let playerDirection:Direction = Direction::Down;

    match playerDirection {
        Direction::Up => println!("Up!"),
        Direction::Down => println!("Down!"),
        Direction::Left => println!("Left!"),
        Direction::Right => println!("Right!"),
    }
}