fn some_fun() {
    println!("we can access the function in the file_1 crate.");
}

mod maths {
    pub mod basic_math {
        pub fn multiplication(num1: &i32, num2: &i32) -> i32 {
            let result = num1 * num2;
            printing(&result);
            result
        }

        fn printing(num: &i32) {
            println!("The final result is : {}", num);
            crate::file_1::some_fun();
        }
    }
}

pub fn rect_area(length: &i32, breadth: &i32) -> i32 {
    use maths::basic_math::multiplication;
    let area = multiplication(length, breadth);
    println!("The area of the rectangle is : {}", area);
    area
}
