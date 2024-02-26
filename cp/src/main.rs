fn main() {
    let args: Vec<String> = std::env::args().map(|y| y.to_owned()).collect();
    println!("Hello world! {:?}", args);
}
