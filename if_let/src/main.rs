fn main() {
    let config = Some(3u8);
    match config {
        Some(max) => println!("the max is configed to b {max}"),
        _ => (),
    }
    if let Some(max) = config {
        println!("the max is configed to b {max}");
    }
}
