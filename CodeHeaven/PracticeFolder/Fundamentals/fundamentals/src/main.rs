// Rust constant naming convention is to use all upper case with underscores between words
// const MAX_POINTS: u32 = 100_000;
fn main() {
    //   let mut x = 5;
    //   println!("The value of x is: {}", x);
    //   x = 6;
    //   println!("The value of x is: {}", x);

    // let x = 5;
    //     let x = x + 1;
    //     let x = x * 2;
    //     println!("The value of x is: {}", x);

    // we cant do this if we use mut variable as it will overwrite the previous without using it

    // let space = "    ";
    // let space = space.len();
    //     println!("The value of x is: {}", space);

    // parse() is used to change the value dataType and we need to provide the type
    // i32 for signed Integer and u32 for unsigned integer (REFER TO TABLE ON PG 54

    // let guess: u32 = "42".parse().expect("Not a number!");

    // for float f64 or f32

    // addition
    //  let sum = 5 + 10;
    // subtraction
    //  let difference = 95.5 - 4.3;
    // multiplication
    //  let product = 4 * 30;
    // division
    //  let quotient = 56.7 / 32.2;
    // remainder
    //  let remainder = 43 % 5;

    //  boolean
    // let t = true;
    // let f: bool = false;

    // let a= "a2wdsw";
    // println!("{}", a);

    // creating array is same as in JS
    // and so is function creation

    //  let tup: (i32, f64, u8) = (500, 6.4, 1);
    //  destructing
    // let (x, y, z) = tup;  println!("The value of y is: {}", y);
    // let x: (i32, f64, u8) = (500, 6.4, 1);
    //  let five_hundred = x.0;
    //   let six_point_four = x.1;
    //    let one = x.2;

    // arrays in Rust have a fixed length and a fixed datatype meaning cannot add &str in i32 arr
    // let arr= [2,3,2];
    //   println!("{}\t{}",arr[0],arr[2]
    // );
    // another_function(5);
    let i: i32 = 23;

    let x = 1;
    let y = {
        let x = 3;
        // the line below is an expression that why it evaluate
        x + 1
    };

    let x = five();
    println!("The value of x is: {}", x);

    //  the if statements are exactly the same as JS
    let condition = true;
    // it can be used in let statement as a block code
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // the return type of if statement should be same

    // loop{
    //     println!("again!");
    //     break;
    // }

    // let mut number1 = 3;
    // while number1 != 0 {
    //     println!("{}!", number1);
    //     number1 = number1 - 1;
    // }

    // for loops are a little different than JS as it is used to iterate over a collection of items like an array or a range of numbers (1..4) or (1..=4) (to include the last number) and unlike JS it is not used to iterate over a condition
    let a = [10, 20, 30, 40, 50];

    for element in a{
        println!("the value is: {}", element);
    }
    // q.iter() returns each element in a collection by using reference instead of the value itself and we can use it to iterate over the collection 
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4) 
    { println!("{}!", number);
}
// instead of using a condition for reverse we can use rev() on a range to reverse it and it does the same thing as JS
    for number in (1..4).rev() 
    { println!("{}!", number);
}

//  using = in the range includes the last number 
    for number in (1..=4).rev() 
    { println!("{}!", number);
}
}
//
// fn another_function(x: i32) {
//     println!("adaswdsdw",);
//     println!("{}", x);
// }

//  the functions dont  use a return keyword but the last line is the return value without semicolon
fn five() -> i32 {
    5
}
