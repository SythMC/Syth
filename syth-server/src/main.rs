mod connection;
mod heartbeat;
mod protocol;

use std::net::TcpListener;
use syth_config::Config;

fn main() {
    let config = Config::load();
    let address = format!("{}:{}", config.server.ip, config.server.port);
    let listener = TcpListener::bind(address).unwrap();

    heartbeat::send_heartbeat(
        &config.server.port,
        &config.server.max_players,
        &config.server.server_name,
        &config.server.public,
    );

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = connection::handle_connection(stream, &config) {
                    eprintln!("An error occured: {}", e)
                }
            }
            Err(e) => eprintln!("An error occured: {}", e),
        }
    }
}
