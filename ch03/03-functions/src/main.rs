fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("{y}");

    let example = five();
    println!("{example}");

    another_func(5, 'h');
}

fn five() -> i32 {
    5
}

fn another_func(value: i32, unit_label: char) {
    println!("the measurement is {value}{unit_label}");
}
