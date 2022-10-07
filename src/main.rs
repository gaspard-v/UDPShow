extern crate ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::net;
use std::time::Duration;
use std::io;
const BUFFER_SIZE: usize = 10000;

fn main() {
    println!("Programme started...");
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    let addr = net::SocketAddr::from(([0,0,0,0], 53));
    let udp_socket_result = net::UdpSocket::bind(addr);

    let udp_socket = match udp_socket_result {
        Ok(socket) => socket,
        Err(error) => panic!("Unable to open socket: {:?}", error),
    };
    udp_socket.set_read_timeout(Some(Duration::from_millis(300))).expect("Unable to set socket timeout");
    let mut buffer = [0; BUFFER_SIZE];
    while running.load(Ordering::SeqCst) {    
        let message_result = udp_socket.recv_from(&mut buffer);

        let (amt, src) = match message_result {
            Ok(result) => result,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut=> {
                continue;
            }
            Err(error) => panic!("Unable to read message from socket {:?}", error),
        };

        let message_utf8 = String::from_utf8_lossy(&buffer[..amt]);
        println!(r#"Message receive from {}, message:
################### MESSAGE START ###################
{}
#################### MESSAGE END #################### 
"#, src.to_string(), message_utf8);
    }
}
