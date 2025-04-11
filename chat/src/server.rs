use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread, vec};

pub fn start_server() {
    let server = TcpListener::bind("127.0.0.1:6000").unwrap();

    println!("Server is running at port 6000");

    for stream in server.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut write_stream = stream.try_clone().unwrap();

    let read_thread = thread::spawn(move || {
        loop {
            let mut buff = vec![0; 1024];
            
            match stream.read(&mut buff) {
                Ok(msg) => {
                    let msg = buff.into_iter().take_while(|&x| x!= 0).collect::<Vec<_>>();
                    let msg = String::from_utf8(msg).unwrap();

                    println!("Client {:?} = {:?}", stream.peer_addr().unwrap(), msg);
                }
                Err(e) => {
                    println!("Got error {:?}", e);
                    break;
                }
            }
        }
    });

    let write_thread = thread::spawn(move || {
        loop {
            let mut buff = String::new();
            std::io::stdin().read_line(&mut buff).unwrap();
            let msg = buff.trim().to_string();
            write_stream.write_all(msg.as_bytes()).unwrap();
            write_stream.flush().unwrap();
        }
    });

    let _ = read_thread.join();
    let _ = write_thread.join();
}