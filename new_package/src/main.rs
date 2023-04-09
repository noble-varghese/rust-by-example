use new_package;
// use new_package::file_1;
// use new_package::file_2;

fn main() {
    new_package::printing();
    println!("Hello, world!");
    let len = 43;
    let breadth = 56;
    new_package::file_1::rect_area(&len, &breadth);
    new_package::file_2::printing();
}
