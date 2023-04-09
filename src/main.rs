// use std::{convert::From};

// #[allow(dead_code)]
// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// impl From<i32> for Number {
//     fn from(item: i32) -> Self {
//         Number { value: item }
//     }
// }

// fn basic_fn() {
//     println!("This is just a basic function with no arguments.");
// }

// fn fn_with_inputs(num1: i32, num2: i32) {
//     println!("The num1={} and num2={}", num1, num2);
// }

// fn fn_with_inputs_and_outputs(num1: i32, num2: i32) -> i32 {
//     num1 * num2
// }

// fn main() {
//     basic_fn();
//     fn_with_inputs(23, 32);
//     let product = fn_with_inputs_and_outputs(23, 32);
//     println!("The product of the two numbers are = {}", product);
//     let full_name = {
//         let first_name = "Noble";
//         let last_name = "Varghese";
//         format!("{} {}", first_name, last_name)
//     };
//     println!("The full name is {}", full_name);
//     let mut n = String::new();
//     std::io::stdin()
//         .read_line(&mut n)
//         .expect("Failed to read the input");
//     let n: f64 = n.trim().parse().expect("Invalid input. Input not a float value");
//     println!("The input from the user is {}", n)

// }

/*
   Rust Ownership
       - Each value has a variable and that's called its owner
       - There can be only one owner at a time.
       - When the owner goes out of scope, the value will be dropped.

    Stack overflow might happen when variables are continuously declared on the stack in a loop.
    Why is ownship moved or transfered in case of non-primitive type such as a string or a vector.
        This is because the memory allocation for those types happen on the heap memory and not on the stack. Using the heap involves
        the use of the OS for memory allocation and returning a pointer to that location.
*/

// fn main() {
//     let x = 32.6;
//     let y = x;
//     println!("x: {} and y:{}", x, y);

//     let s1 = String::from("abc");
//     let s2 = & s1;
//     println!("the values of s1={} and s2={}", s1, s2);
// }

// use std::vec;

// fn stack_func(mut num: i64) {
//     println!("The current value of the number is {}", num);
//     num = 67;
//     println!("The updated number is {}", num);
// }

// fn heap_func(var: &mut Vec<i32>) {
//     println!("The first value of var is {:?}", var);
//     var.push(60);
//     println!("Var: {:?}", var);
// }

// fn main() {
//     let stack_num = 32;
//     let mut heap_vec = vec![4, 5, 6];
//     stack_func(stack_num);
//     println!(" The value inside the main of stack_num is {}", stack_num);
//     heap_func(&mut heap_vec);
//     println!(" The value inside the main of heap_num is {:?}", heap_vec);

//     let string1 = String::from("This is a long string");
//     let string2 = String::from("This is another long string");

//     // Here we have combined the two strings without copying the contents of the two strings.
//     let vec_string = vec![&string1, &string2];
//     println!("The value of the combined strings are: {:?}", vec_string);
// }

// fn main() {
//     // let mut heap_num = vec![4, 5, 6];
//     // let ref1 = &mut heap_num;
//     // let ref2 = &mut heap_num;
//     // println!("ref1: {:?} and ref2", ref1);

//     let mut heap_num = vec![4, 5, 6];
//     let ref2 = &heap_num;
//     let ref3 = &heap_num;
//     let ref1 = &mut heap_num;
//     println!(
//         "ref1: {:?} and ref2: {:?} and heap: {:?}",
//         ref1, ref2, heap_num
//     );
// }

// use std::io::Read;

// fn some_fn(a1: &String, a2: &str) {
//     println!("The string values are {} & {}", a1, a2)
// }

// fn main() {
//     let s1 = String::from("String value from 1");
//     let s2 = "myself another string";
//     some_fn(&s1, s2);

//     println!("The existing ones are {} {}", s1, s2);
//     let marks = 65;
//     if marks <= 70 && marks >= 60 {
//         println!("the stiudent has a distinction.!");
//     }

//     // println!("Guess any number between 1 and 20.");
//     // while guess != true {
//     //     let mut number = String::new();
//     //     std::io::stdin()
//     //         .read_line(&mut number)
//     //         .expect("Failed to read the input");

//     //     let number = number.trim().parse().expect("Invalid input type");
//     //     if my_number == number {
//     //         println!("You guessed the number correctly !");
//     //         guess = true;
//     //     } else {
//     //         println!("Please try again !!");
//     //     }
//     // }

//     // println!("Enter any number and I can tell you the next number that is divisible by 2 and 5");
//     // let mut number = String::new();
//     // std::io::stdin()
//     //     .read_line(&mut number)
//     //     .expect("Failed to read the input");
//     // let mut number: i32 = number.trim().parse().expect("Invlaid input type");
//     // let mut divisible_by_2_5 = false;
//     // while divisible_by_2_5 != true {
//     //     number += 1;
//     //     if number % 2 == 0 && number % 5 == 0 {
//     //         println!("Number divisible by 2 and 5 is {}", number);
//     //         divisible_by_2_5 = true;
//     //     }
//     // }

//     let mut some_vec = vec![20, 30, 40, 50, 60, 70];
//     for i in &mut some_vec {
//         *i += 5;
//         println!("{:?}", i);
//     }
//     println!("{:?}", some_vec);
// }

/*
    Write a program for finding the difference of the square of the sum and the sum of square of the first N number (where N is a user defined input that you program will take). for instance, if the user enters the number of let say 5,
Then you should first compute the squae of sum = (1+2+3+4+5)^2  = 225

and next you will compute the sum of square as  = (1^2  + 2^2  + 3^2  + 4^2  + 5^2)  = (1 + 4+ 9 + 16 +25 ) = 55

and finally you will compute the difference = 225 - 55 = 170.

 */

// fn compute_sq(num: i32) -> i32 {
//     num * num
// }

// fn compute_sq_sum(num: i32) -> i32 {
//     let mut sum_sq = 0;
//     for i in 0..=num {
//         sum_sq += compute_sq(i);
//     }
//     sum_sq
// }

// fn compute_sum_sq(num: i32) -> i32 {
//     let mut sum = 0;
//     for i in 0..=num {
//         sum += i
//     }
//     sum * sum
// }

// fn main() {
//     println!("enter a number");
//     let mut input = String::new();
//     std::io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read the input");
//     let input: i32 = input.trim().parse().expect("Invalid type");
//     let difference = compute_sum_sq(input) - compute_sq_sum(input);
//     println!("The answer is {}", difference);
// }

// fn main() {
//     println!("enter a number");
//     let mut input = String::new();
//     std::io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read the input");
//     let input: i32 = input.trim().parse().expect("Invalid type");
//     let mut sum = 0;
//     for i in 0..input {
//         if i % 3 == 0 && i % 5 == 0 {
//             println!("{}", i);
//             sum += i;
//         } else if i % 3 == 0 {
//             println!("{}", i);
//             sum += i;
//         } else if i % 5 == 0 {
//             println!("{}", i);
//             sum += i;
//         }
//     }
//     println!("The sum is {}", sum);
// }

// fn new_stack(max_size: usize) -> Vec<u32> {
//     let vec = Vec::with_capacity(max_size);
//     vec
// }

// fn pop(stack: &mut Vec<u32>) -> Option<u32> {
//     let popped_val = stack.pop();
//     println!("The popped value is {:?}", popped_val.unwrap());
//     popped_val
// }

// fn push(num: u32, max_size: usize, stack: &mut Vec<u32>) {
//     if stack.len() == max_size {
//         println!("Stack is full...");
//     } else {
//         stack.push(num);
//         println!("The stack is {:?}", stack);
//     }
// }

// fn input() -> u32 {
//     let mut n = String::new();
//     std::io::stdin()
//         .read_line(&mut n)
//         .expect("Value not found.");
//     let n: u32 = n.trim().parse().expect("Invalid input");
//     n
// }

// fn main() {
//     println!("Please mention the size of the stack");
//     let stack_size = input();
//     let mut new_stack = new_stack(stack_size as usize);
//     loop {
//         println!("Choose one....");
//         println!("1. Push \n2. Pop \n3. Display \n4.Size \n5.Exit");
//         let choice = input();
//         match choice {
//             1 => {
//                 println!("Enter the input to apppend to the stack.");
//                 let ele = input();
//                 push(ele, stack_size as usize, &mut new_stack);
//             }
//             2 => {
//                 pop(&mut new_stack);
//             }
//             3 => {
//                 println!("the current stack is {:?}", new_stack);
//             }
//             4 => {
//                 println!("The size of the current stack is {}", new_stack.len());
//             }
//             5 => break,
//             _ => println!("Exiting..."),
//         }
//         println!("Choose 1 to continue and 0 to exit");
//         let break_choice = input();
//         println!("Break choice = {}", break_choice);
//         if break_choice == 1 {
//             continue;
//         } else {
//             println!("Ending the program.");
//             break;
//         }
//     }
// }

// fn new_stack(size: usize) -> Vec<char> {
//     let vec = Vec::with_capacity(size);
//     vec
// }

// fn input() -> String {
//     let mut n = String::new();
//     std::io::stdin()
//         .read_line(&mut n)
//         .expect("Value not found.");

//     let n = n.trim().parse().expect("Invalid input");
//     n
// }

// fn push(stack: &mut Vec<char>, input: char, max_size: usize) {
//     if stack.len() == max_size {
//         println!("The string reached max size");
//     } else {
//         stack.push(input);
//     }
// }

// fn pop(stack: &mut Vec<char>) -> Option<char> {
//     stack.pop()
// }

// fn main() {
//     println!("Enter the string to be reversed.");
//     let input_string = input();
//     let string_size = input_string.len();
//     let mut stack = new_stack(string_size);
//     let mut reverse_string = String::new();

//     for i in input_string.chars() {
//         push(&mut stack, i, string_size);
//     }

//     for _i in 0..stack.len() {
//         reverse_string.push(pop(&mut stack).unwrap());
//     }
//     println!("The input string is : {:?}", input_string);
//     println!("The reversed string is : {:?}", reverse_string);
// }

// Traits and defaults

// struct Person {
//     citizenship: String,
//     name: String,
//     age: i32,
//     salary: i32,
//     gender: char,
// }

// struct Student {
//     name_stg: String,
//     age: i32,
//     sex: char,
//     country: String,
// }

// trait GeneralInfo {
//     fn info(&self) -> (&str, i32, char);
//     fn country_info(&self) -> &str {
//         "This is a standard case"
//     }
// }

// impl GeneralInfo for Person {
//     fn info(&self) -> (&str, i32, char) {
//         (&self.name, self.age, self.gender)
//     }
//     fn country_info(&self) -> &str {
//         &self.citizenship
//     }
// }

// impl GeneralInfo for Student {
//     fn info(&self) -> (&str, i32, char) {
//         (&self.name_stg, self.age, self.sex)
//     }
// }

// fn main() {
//     let person = Person {
//         name: String::from("Noble Varghese"),
//         age: 27,
//         citizenship: String::from("Indian"),
//         salary: 20_000,
//         gender: 'M',
//     };
//     // let student_1 = Student {
//     //     name_stg: String::from("Noble Varghese"),
//     //     age: 22,
//     //     sex: 'M',
//     //     country: String::from("Indian"),
//     // };

//     println!("{:?}, {:?}", person.info(), person.country_info());
// }

// Functions inside a trait

// use std::vec;

// struct Data {
//     some_data: Vec<i32>,
// }

// trait BasicStats {
//     fn mean(&self) -> f32;
//     fn variance(&self) -> f32;
// }

// impl BasicStats for Data {
//     fn mean(&self) -> f32 {
//         let mut sum = 0;
//         for i in self.some_data.iter() {
//             sum += i;
//         }
//         sum as f32 / self.some_data.len() as f32
//     }

//     fn variance(&self) -> f32 {
//         let mean = self.mean();
//         let mut sum_squared_diff = 0.0;
//         for i in self.some_data.iter() {
//             sum_squared_diff += (*i as f32 - mean) * (*i as f32 - mean);
//         }
//         sum_squared_diff as f32 / self.some_data.len() as f32
//     }
// }

// fn main() {
//     let my_data = Data {
//         some_data: vec![3,4,5,6,6,7,7,7,8,8]
//     };

//     println!("The mean of the data is {:?}", my_data.mean());
//     println!("The variance of the data is {:?}", my_data.variance());
// }

//Enums

// use std::vec;

// enum Conveyance {
//     Car = 30,
//     Train,
//     Air,
// }
// impl Conveyance {
//     fn travel_allowance(&self, miles: i32) -> f32 {
//         let allowance = match self {
//             Conveyance::Air => miles as f32 * 14.0 * 2.0,
//             Conveyance::Train => miles as f32 * 14.0 * 2.0,
//             Conveyance::Car => miles as f32 * 14.0 * 2.0,
//         };
//         allowance
//     }
// }

// fn main() {
//     let participant_1 = Conveyance::Car;
//     let participant_3 = Conveyance::Train;
//     let participant_2 = Conveyance::Air;
//     println!(
//         "The value the option is {}",
//         participant_1.travel_allowance(40)
//     );
//     println!(
//         "The value the option is {}",
//         participant_3.travel_allowance(50)
//     );
//     println!(
//         "The value the option is {}",
//         participant_2.travel_allowance(60)
//     );

//     let some_vec = vec![2, 3, 4, 5, 6];
//     let result1 = match some_vec.get(3) {
//         Some(a) => Ok(a),
//         None => Err("The value does not exits."),
//     };

//     println!("{:?}", result1);
// }

// Hashmaps

// use std::{collections::HashMap, hash::Hash};

// fn main() {
//     let mut person = HashMap::new();
//     person.insert("Noble", 40);
//     person.insert("Robin", 30);
//     person.insert("Mikku", 20);

//     println!("The age is {:?}", person.get("Noble").unwrap());

//     if person.contains_key("Noble") {
//         println!("Yes the hashmap contains the value");
//     } else {
//         println!("Nope the hashmap doesn't have the value");
//     }

//     match person.get("Robin") {
//         Some(value) => println!("Yes the value exist: {}", value),
//         None => println!("Nope the value is nto exsiting..."),
//     }

//     for (name, age) in &person {
//         println!("The person {} has an age of {}", name, age);
//     }

//     let mut likes = HashMap::new();
//     likes.insert("Noble", "Mango");
//     likes.insert("Robin", "Apple");
//     likes.insert("Mikku", "Banana");

//     // Vectors with values in hash map

//     let some_vec = vec![2, 3, 4, 6, 6, 6, 78, 8, 9, 9, 0, 4, 6, 3, 22];
//     let mut freq_vec = HashMap::new();
//     for i in &some_vec {
//         let freq = freq_vec.entry(*i).or_insert(0);
//         *freq += 1;
//     }
//     println!("The final value is {:?}", freq_vec);
// }

//Lifetimes
/*
    Why do we use lifetime ?
    Dangling reference or danlging pointer:
        Danlging reeferences are mostly encountered when dealing with functions.
        Undermined lifetimes.
        lifetime will correspond to the minimum lifetime of the two references in question.

*/

// fn main() {
//     let s1 = "Hello";
//     let v: &str;
//     {
//         let s2 = String::from("World");
//         v = some_fn(s1, s2.as_str());
//     }

//     println!("{}", v);
// }

// fn some_fn<'a, 'b>(first_str: &'a str, second_str: &'b str) -> &'a str {
//     println!("{}", second_str);
//     first_str
// }

// fn main() {
//     let i = 50;
//     let result: &i32;
//     {
//         let j = 80;
//         result = greater(&i, &j);
//         println!("{}", result);
//     }
// }

// fn greater<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
//     if i > j {
//         i
//     } else {
//         j
//     }
// }

// struct Person<'a> {
//     name: &'a str,
//     age: i32,
// }

// fn main() {
//     let first_name = "Nouman";
//     let mut nouman = Person {
//         name: first_name,
//         age: 40,
//     };
//     let last_name = String::from("Varghese");
//     nouman.name = &last_name;

//     println!(
//         "The name of the person is {} and his age is {}",
//         nouman.name, nouman.age
//     );
// }

// Closure:
/*
   Closures:
       - They are functions without names.
       - Any body if the closure is defined between the two pipe symbols.
            - If there is only one statement inside a closure, then we do not have to keep the body in side the pipes.
        - We can pass the closure as a parameter to another function.
        More on the closures:
        -
*/

// fn main() {
//     let x = 32;
//     let sqaure = |num: i32| println!("The square fo the variable is {}", num * num);
//     sqaure(x);

//     let y = 89;
//     sqaure(y);

//     let print_info = |general_info: &str, name: &str, age: i32| {
//         println!("{}\n\t{}: {}", general_info, name, age)
//     };
//     let general_info = String::from("The details are ");
//     let (person_name, person_age) = (String::from("Noble"), 27);
//     print_info(general_info.as_str(), &person_name, person_age);
// }

// fn division<F: Fn(f32) -> bool>(x: f32, y: f32, f: F) {
//     if f(y) == true {
//         println!("The division result is {}", (x / y) as f32);
//     } else {
//         println!("The division is not possible");
//     }
// }

// fn main() {
//     let division_status = |y: f32| {
//         if y != 0.0 {
//             true
//         } else {
//             false
//         }
//     };
//     division(30.0, 6.0, division_status);
//     division(5.0, 0.0, division_status);

//     let mut vec_1 = vec![2, 3, 4, 6];
//     vec_1[1] = 67;
//     let mut some_closure = || {
//         vec_1.push(43);
//         println!("The vector here is : {:?}", vec_1);
//     };
//     some_closure();
//     vec_1[1] = 67;

// }

/// Function Types
/// Use of the functional pointers will be used to use some external functinos or APIs.

// fn prints_name(name: &str) {
//     println!("The name of the person is {}", name);
// }

// fn prints_full_info(f: fn(&str), some_one: &str, age: i32) {
//     f(some_one);
//     println!(" and the age of the person is {}", age);
// }

// fn main() {
//     let (name, age) = (String::from("Noble"), 40);
//     prints_full_info(prints_name, &name, age);
// }

// fn main() {
//     let some_vec = vec![1, 2, 3, 4, 5, 6, 7, 78];
//     let iter = some_vec.iter().any(|&x| x > 3);
//     let iter_all = some_vec.iter().all(|&x| x > 3);

//     println!("The iterms in the array {:?}", iter);
//     println!("The iterms in the array {:?}", iter_all);

//     // Find function will search for an element using some condition inside a closure.
//     let find_all = some_vec.iter().find(|&&x| x > 0);
//     println!(
//         " The result of the findall functino is {:?}",
//         find_all.unwrap()
//     );

//     let check = some_vec.iter().position(|&x| x > 4);
//     println!(
//         " The result of the findall functino is {:?}",
//         check.unwrap()
//     );

//     let check = some_vec.iter().rposition(|&x| x > 78);
//     if check != None {
//         println!(
//             "The result of the position from right is: {:?}",
//             check.unwrap()
//         );
//     }

//     let a = vec![1, 2, 3, 4, 5, 6, 6, 7, 7, 8, 8];
//     let filtered_values = a.iter().filter(|&x| *x > 5).collect::<Vec<&u32>>();
//     println!("The filtered values are : {:?}", filtered_values);

//     let b = a.clone();
//     let filtered_values = a.into_iter().filter(|&x| x > 5).collect::<Vec<u32>>();
//     println!("The filtered values are : {:?}", filtered_values);

//     let mapped_values = b
//         .iter()
//         .map(|x| 2 * *x)
//         .filter(|x| *x > 10)
//         .collect::<Vec<u32>>();
//     println!("The mapped values are: {:?}", mapped_values);

//     //     println!("{}", iter.next().unwrap());
//     //     println!("{}", iter.next().unwrap());
//     //     println!("{}", iter.next().unwrap());
//     //     println!("{}", iter.next().unwrap());
//     //     println!("{}", iter.next().unwrap());
// }


// Rust modules
/* 
    To organise the files in the rust repository  we use this there is a module system
    Package > crates > modules
 */

