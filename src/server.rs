use log::{error, info};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap_or_else(|e| {
            error!("Failed to read from socket");
            panic!("Program will exit due to error.");
        });

        if bytes_read == 0 {
            return;
        }
        // unpack
        // match
        // respond
        //

        info!(
            "Received: {}",
            String::from_utf8_lossy(&buffer[..bytes_read])
        );
        stream
            .write(&buffer[..bytes_read])
            .expect("Failed to write to socket");
    }
}

pub fn start_node(port: u16) -> std::io::Result<()> {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let listener = TcpListener::bind(address).unwrap_or_else(|e| {
        error!("Error creating a TcpListener for port {} -- {}", port, e);
        panic!("Program will exit due to error.");
    });
    info!("Server listening on port {}", port);
    // mutex<vec> blocks
    // signal

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // clone signal
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}
