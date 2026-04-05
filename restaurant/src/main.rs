use rand::Rng;

use std::{collections::HashMap, fmt::Result, io::Result as IoResult};

// below equivalent to:
// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// brings all public items defined in a path into scope.
// be careful, glob operator(below) makes it harder to tell what names
// are in scope and where a name used in the program was defined.
use std::collections::*;
// glob operator is used when making tests to bring everything that
// needs testing

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_num = rand::thread_rng().gen_range(1..=100);
}
