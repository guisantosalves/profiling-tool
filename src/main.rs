
mod server;

#[tokio::main] // macro que transforma o main em async
async fn main() {
    println!("System Profiler rodando em http://localhost:3000");
    server::start().await;
}
