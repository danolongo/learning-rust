fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    println!("{:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn second_word(s: &String) -> (usize, usize) {
    let bytes = s.as_bytes();
    let mut first = String::from("");
    let mut second = String::from("");
    let mut num = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && num == 0 {
            first = String::from(&s[num..i]);
            num += 1;
        }
        if item == b' ' && num == 1 {
            second = String::from(&s[i..s.len()]);
        }
    }

    (first.len(), second.len())
}

fn or_first_word(s: &String) -> &str {
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {
    let mut s = String::from("my name is daniel");

    let word = first_word(&s);
    println!("{}", word);

    let (one, two) = second_word(&s);
    println!("{}, {}", one, two);

    let res = or_first_word(&s);
    println!("{}", res);

    s.clear();
}
