fn main() {
    // new empty vector
    let mut v: Vec<i32> = Vec::new(); // we specify type bc we are initting empty
    // or let v mut = vec![1, 2, 3, 4, 5, 6]; to init w values

    v.push(6);
    v.push(7);
    v.push(6);
    v.push(7);

    println!("{:?}", &v);

    let third: &i32 = &v[2];
    println!("the third element is {third}");

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("the third element is {third}"),
        None => println!("there is no third element"),
    }

    let mut a = vec![1, 2, 3, 4, 5];
    // let dne = &a[100];
    let dne = v.get(100);
    // for .get() the return type is Option<&T> (either Some or None) if out of bounds just returns
    // None
    // for [index] returns &T, if out of bounds program crashes

    a.push(1);
    let first = &a[0];
    println!("first element: {first}");

    let mut arr = vec![100, 32, 57];
    for i in &arr {
        println!("{i}");
    }

    println!("break");

    for i in &mut arr {
        *i += 50;
    }
    println!("{:?}", &arr);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        let v = vec![1, 2, 3, 4];
    }

    // --------------------------------------------------------
    // strings
    // --------------------------------------------------------

    // native rust string type is str, typically seen borrowed as &str
    // String is from the standard lib, its a growable owned UTF-8 ecoded string
    let mut s = String::new();

    let data = "hello malika";
    let s = data.to_string();
    // or
    let s = "hello malika".to_string();
    // or
    let mut s = String::from("hello malika");
    s.push_str("chaudhary");
    s.push('M'); // can only use push() with a char

    let s2 = data.to_string() + &s;
    // string on the left must be owned to concactenate

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}"); // same as line above

    let s1 = String::from("malika");
    // let h = &s1[0]; is NOT VALID
    // Strings are wrappers over Vec<u8>

    // --------------------------------------------------------
    // hash maps
    // --------------------------------------------------------

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // like vectors, hashmaps are stored in the heap
    // hashmaps has keys of type String and i32, i16, i8, etc

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("fav color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value become invalid from here on
    // HashMap.insert() takes ownership of params

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    scores
        .entry(String::from("Blue"))
        .and_modify(|score| *score = 100)
        .or_insert(67);

    println!("{:?}", scores["Blue"]);
    println!("{scores:?}");

    let text = "hello hi my name is daniel";
    let mut map1 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map1.entry(word).or_insert(0);
        *count += 1;
    }
}
