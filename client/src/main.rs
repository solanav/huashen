mod network;
mod system;

fn main() {
    system::key::read_event();
    
    //network::reactive::start_server("127.0.0.1", "8080")
    //    .expect("Failed to start the server");
}
