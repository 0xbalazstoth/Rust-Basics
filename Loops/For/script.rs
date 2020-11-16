fn main() {
    let numbers = 30..51; //Range, inclusive
    let f1Teams = vec!["Mercedes", "Ferrari", "Red Bull", "Renault"];

    for i in numbers {
        println!("{}", i);
    }

    for (index, team) in f1Teams.iter().enumerate() {
        println!("index: {}, team name: {}", index, team);
    }
}