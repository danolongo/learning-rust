use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("enter array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failes to read line");

    let index: usize = index.trim().parse().expect("index entered was not a num");

    let element = a[index];

    println!("the value of the element at idx {index} is {element}");
    another();
}

fn another() {
    println!("another");
}
