use std::io;

mod client;
mod server;

fn main() {
    println!("Welcome to chat application");
    println!("Please chose type of the application");
    println!("1-Server");
    println!("2-Client");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read the input");

    let typeApp:i8 = input.trim().parse().unwrap();

    if typeApp == 1 {
        println!("Starting chat server ...");
        server::start_server();
    }
    else if typeApp == 2 {
        println!("Starting chat client ...");
        client::start_client();
    }
    else {
        println!("Please provide a valid choice");
    }
    
}
