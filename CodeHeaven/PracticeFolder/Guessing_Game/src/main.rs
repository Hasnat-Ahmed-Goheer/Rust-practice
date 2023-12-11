extern crate rand;

use std::io; // std :standard library io(input, output)
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game!!!\n\n");
    println!("Please input your Guess!");
    
    // all variables are immutable by default like let foo = 5; is immutable but if we set it let mut foo  = 5; then it becomes mutable
    let secret_number = rand::thread_rng().gen_range(1,
101);// rng means random number generator gen means generator


    println!("the secret number is:{}", secret_number);
loop{
        let mut guess = String:: new();
    io::stdin().read_line(&mut guess).expect("Failed to read the input");
    // let guess : u32 = guess.trim().parse().expect("Please Enter a Number !");
    // note the we added the match keyword before the guess.trim() it is important that we do that in case we are passing an object for the Result enum to make sure that the if user enters a correct value it uses that value in cmp arms otherwise just continue into the next iteration
    let guess :u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    // let mut guess1 = String::new();
    // io::stdin().read_line(&mut guess1)
    // .expect('error reading'); 1
    // println!("{}", guess1);

    // use the book for all the references


    println!("You guessed:{}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Large!"),
        Ordering::Equal => {println!("Bingo!");
        break;},
    }
    }
}
