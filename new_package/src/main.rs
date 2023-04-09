use new_package;
// use new_package::file_1;
// use new_package::file_2;
use array_tool::vec::*;

fn main() {
    new_package::printing();
    println!("Hello, world!");
    let len = 43;
    let breadth = 56;
    new_package::file_1::rect_area(&len, &breadth);
    new_package::file_2::printing();

    let vec_1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 8];
    let vec_2 = vec![1, 2, 3, 4, 5];
    let intersection = vec_1.intersect(vec_2.clone());
    println!(
        "The intersection of the two vectors are: {:?}",
        intersection
    );

    let unique = vec_1.union(vec_2);
    println!("The unique vector from vec_1 are: {:?}", unique);
}
