mod os_utils;
mod net_utils;

fn main() {
    net_utils::start_server("127.0.0.1", "8080")
        .expect("Failed to start the server");
}
