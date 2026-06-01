fn first_word(s: &String) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> &str {
    let mut idx: usize = 0;
    for &item in s.as_bytes().iter() {
        if item == b' ' {
            idx += 1
        }
    }
    &s[idx + 2..]
    // + 2 bc its zero indexed (+1)
    // and another +1 to get rid of space
}

fn main() {
    let hi = String::from("hi malika");
    let new = first_word(&hi);
    let new2 = second_word(&hi);

    println!("{}", new);
    println!("{}", new2);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3])
}
