use std::io;

fn main() {
    let mut input= String::new();

    println!("Enter your weight in kilos:");
    io::stdin().read_line(&mut input).unwrap();

    let weight = input.trim().parse().unwrap();
    
    println!("Your weight on the moon is {}",moon_weight(weight));
}

fn moon_weight(weight:f32)->f32{
    (weight/9.81)*1.622
}