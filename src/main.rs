use dotenv::dotenv;
fn main() {
    dotenv().ok();
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
}
