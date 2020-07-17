extern crate redis;

use redis::{Client, Commands};

fn main() {
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_connection().unwrap();
    let _: () = conn.set("answer", 42).unwrap();
    let answer: i32 = conn.get("answer").unwrap();
    println!("Answer: {}", answer);
    let carlitos: String = conn.get("carlitos").unwrap();
    print!("Carlitos: {}", carlitos);
}
