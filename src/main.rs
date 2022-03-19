use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to spawn our app.");
    println!("{}", listener.local_addr().unwrap().ip());
    println!("{}", listener.local_addr().unwrap().port());
    run(listener)?.await
}
