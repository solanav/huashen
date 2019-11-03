mod os_utils;
mod net_utils;

fn main() {
    net_utils::start_server()
        .expect("Failed to start the server");
}
