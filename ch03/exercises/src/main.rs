fn main() {
    let one = f_to_c(100);
    println!("100 f in c is {one}c");

    let two = fib(10);
    println!("fib(10) = {two}");

    song();
}

fn f_to_c(fahrenheit: i16) -> i16 {
    (fahrenheit - 32) * 5 / 9
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}

fn song() {
    let lines: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 0..13 {
        print!("On the {i}th day of Christmas\nMy true love gave to me\n");
        for j in (0..i).rev() {
            let line: &str = lines[j];
            println!("{line}");
        }
        println!("\n");
    }
}
