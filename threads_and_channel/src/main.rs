use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (transmitter, receiver) = mpsc::channel();

    let v1 = vec![1, 2, 3, 4];

    thread::spawn(move || { 
        thread::sleep(Duration::from_secs(5));
        println!("This is the vector = {:?}", v1);
        transmitter.send(v1).unwrap();
    });
    
    let data_received = receiver.recv().unwrap();
    println!("{:?}", data_received);
}

