// fn main() {
// let mut s = String::from("Hello");
// s.push_str(", World");
// println!("{}",s);
// instead of calling this a shallow copy, it’s known as a move. because it literally moves the reference to the second var to ensure that when both variable go out of scope the second one frees the memory from heap instead of the first one (this is known as double free error
// let s1 = String::from("Hello World");
// let _s2  = s1;
// println!("{}",s1);
// Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
// If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.
// let s1 = String::from("hello");
// let s2 = s1.clone();
// println!("s1 = {}, s2 = {}", s1, s2);
// this is possible because integers types are known and they have a specified memory allocation therefore they are stored in stacks,there’s no difference between deep and shallow copying here,
//  Rust has a special annotation called the Copy trait that we can place on types like integers that are stored on the stack  all of the basic types like int , float ,bool are copy and tuples containing these types are also copy
// let x = 5;
// let y = x;
// println!("x = {}, y = {}", x, y);
// let s = String::from("Hello!");
// let x = 5;
// takes_ownership(s);
// / s's value moves into the function...
// ... and so is no longer valid here.
// makes_copy(x);
// x would move into the function, // but i32 is Copy, so it’s okay to still // use x afterward.
// let s1 = give_ownership();
// //  s2 comes into scope.
// let s2 = String::from ("Hello");
// // s3 comes into scope and s2 gives the ownership to the function
// let s3 = take_and_give_back(s2);
// println!("{},{}",s1,s3);
// Rust provides a drop function that is a special function that makes sure that the var that is about to go out of scope has it allocated memory removed unless the data has been moved to be owned by another variable.
// Here, s3 goes out of scope and is dropped. s2 goes out of scope but was // moved, so nothing happens. s1 goes out of scope and is dropped.
// }
// fn give_ownership() ->String{
// gives_ownership will move its // return value into the function // that calls it.
//     let string = String::from ("hello");
// string comes into scope.
//     string
//  string is returned and
// moves out to the calling function
// }
// fn take_and_give_back(str :String) -> String{
//     str
// }
// fn takes_ownership(string: String){
//     println!("{}", string);
// }
// fn makes_copy(x:u32){
//     println!("{}",x);
// }
// fn main() {
//     let s1 = String::from("hello");
//     let (s2, len) = calculate_length(s1);
//     println!("The length of '{}' is {}.", s2, len);
// }
// if we have a type that does not have a definite size we call those primitive types usize in rust or dynamically sized types
// fn calculate_length(s: String) -> (String, usize) {
// len() returns the length of a String.
//     let length = s.len();
//     (s,length)
// }
// because of the concept of moving we have to move the string to and from the fun if we want to use it in the main fun which is a hassle and thus we use references
// fn main() {
// . Functions that have references as parameters instead of the actual values mean we won’t need to return the values in order to give back ownership, since we never had ownership. We call having references as function parameters borrowing
//     let s1 = String :: from ("hello World");
//     let len = calculate_length(&s1);
//         println!("The length of '{}' is {}.", s1, len);
//     println!()
// }
// fn calculate_length(s: &String) -> usize {
// len() returns the length of a String.
//         let length = s.len();
//         length
//or s.len()
//     }
// fn main() {
//  let mut s = String::from("hello");
//  as variables are immutable by default, so are references
//   change(&s);
// if we pass the reference as a mut it becomes mutable and we have to make sure that the value String type var is also mutable
//     change(&mut s);
//  }
//  fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }
//But mutable references have one big restriction: you can only have one mutable reference to a particular piece of data in a particular scope.
// fn main() {
// let mut s = String::from("hello");
// let m1 = &mut s;
// let m2 = &mut s;
// cannot borrow `s` as mutable more than once at a time and it can be fixed by by creating a block scope
// {let m1 = &mut s;}
// let m2 = &mut s;
// however if we use the var  as this it can be used by mire than one var as it is not being used in the data race
// let r1 = &s;
// let r2 = &s;
//  cannot borrow `s` as mutable because it is also borrowed as immutable
// let r3 = &mut s;
// println!("{},{}",  r1,r2);
// }
// In Rust, by contrast, the compiler guarantees that references will never be dangling references: if we have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
// fn main () {
// let reference_to_nothing = dangle();
// }
//  fn dangle() -> &String {
// dangle returns a reference to a String
//   let s = String::from("hello");
// s is a new String
// &s // we return a reference to the String, s
//} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!
//  At any given time, you can have either but not both of: • One mutable reference. • Any number of immutable references. 1. References must always be valid.

// fn main () {
//   let line = String :: from ("Hello world!");
//   let word = first_word(&line);
//   println!("{}",word);
// }

// fn main() {
//let mut s = String::from("hello world");
//let word = first_word(&s); // word will get the value 5.
// s.clear(); // This empties the String, making it equal to "".
// word still has the value 5 here, but there's no more string that
// we could meaningfully use the value 5 with. word is now totally invalid!
// }
// fn first_word(s: &String) -> usize {
// let bytes = s.as_bytes();
//println!("{:?}",bytes);// creates a array in a nice format for the [u8] that helps in printing the bytes can also be written as println!("{:#?},bytes")
// [72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33] the bytes array is like this it is created by as_bytes method
//know that iter is a method that returns each element in a collection and enumerate wraps the result of iter and returns each element as part of a tuple instead.
// The first element of the returned tuple is the index, and the second element is a reference to the element
// since tuple can be destructed therefore we do it like below (i,&item)
// for (i, &item) in bytes.iter().enumerate()  // since it is a [u8] array therefore it has to iterated and enumerated so that it becomes an array that can be looped over (i is index and since the string is passed as reference therefore we have to use use a reference var like &item so that they point at the same thing in the heap)

// {
//We search for the byte that represents the space by using the byte literal syntax. If we find a space, we return the position. Otherwise, we return the length of the string by using s.len(): that means that b is for byte
// since we checking if there is space space in between the words then return the value of index
//     if item == b' ' {
//        return i;
//       }
//   }
//    s.len()
// }

// A string slice is a reference to part of a String, and looks like this:
// We can create slices using a range within brackets by specifying [starting_index..ending_index], where starting_index is the first position included in the slice and ending_index is one more than the last position included in the slice.

//     fn main (){
//       let s = String :: from ("hello world");
//       let hello = &s[0..5];
//  if starting from 0 index
//       let s = String::from("hello");
//        let slice = &s[0..2];
//        let slice = &s[..2];
//        let len = s.len();
//  for slicing from an index to the length of the string
//        let slice = &s[1..];
//  for slicing the string from start to the length of the string
//        let slice = &s[..]
//  }

// fn main() {
//     let s = String::from("hello from the other side");
//     let word = first_word(&s);
//     println!("{}", word);

    // Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. Because clear needs to truncate the String, it tries to take a mutable reference, which fails
    // s.clear();
// }
// The type that signifies “string slice” is written as &str
// fn first_word(s: &String) -> &str {
    // let bytes = s.as_bytes();
    // for (i, &item) in bytes.iter().enumerate() {
        // we are checking the item in the byte to determine the length of the first word s
        // if item == b' ' {
            // changing the string to the slice we get by the condition
            // return &s[0..i];
        // }
    // }
    // since the string is changed so we can return it
    // &s[..]
// }

// fn main (){
//   let s= String :: from ("Hello World!");
//   let index = first_word_index(&s);
//   let word = &s[..index];
//   println!("{}",word);
// }

// fn first_word_index(s:&String) -> usize {
// converting the string reference into bytes
//   let bytes = s.as_bytes();
//   for (i,&item) in bytes.iter().enumerate(){
//     if item == b' '{
//       return i;
//     }
//   }
//because string is being passed as reference so the concept of borrowing is causing this
//   s.len()
// }

// string literals being stored inside the binary. Now that we know about slices, we can properly understand string literals: let s = "Hello, world!";
// The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference

// =========REMEMBER+============
// String creating using String:: from creates a string in the heap
// string created using let s = "Hello World" creates a String literal &str which is basically a binary in heap and it is immutable

// it allows us to use the same function on both Strings and &strs
// we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the entire String. Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality
// fn first_word (s: &str) -> {

// }

// fn main() {
//     let my_string = String::from("hello world"); // first_word works on slices of `String`s
//     let word = first_word(&my_string[..]);
//     println!("{}", word);
//     let my_string_literal = "hello world"; // first_word works on slices of string literals
//     let word = first_word(&my_string_literal[..]); // since string literals *are* string slices already,
//     println!("{}", word);
    // this works too, without the slice syntax!
//     let word = first_word(my_string_literal);
//     println!("{}", word);
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

fn main (){
  // This slice has the type &[i32]. It works the same way as string slices do,by storing a reference to the first element and a length. 
  let a = [1,2,3,4,5,6,67,8,9,20];
  let slice = &a[..6];
  // however it can only be printed by using:? or :#? 
  println!("{:?}",slice);

}