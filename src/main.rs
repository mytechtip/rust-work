
// use rust libs
use std::net::*;
use std::io::*;

fn main() {
    // create a TCP listener bound to `0.0.0.0:9090`
    let listener = TcpListener::bind("0.0.0.0:9090").unwrap();
    // listen for the incoming message
    println!("Server listening on port 9090");
    for stream in listener.incoming() {
        // match result
        match stream {
            Ok(stream) => {
                handle_client(stream)
            }
            Err(e) => {
                // print error cause
                println!("Error: {}", e);
            }
        }
    }
    // close the server
    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    // define buffer variable
    let mut data = [0 as u8; 1024]; 
    // read up the bytes and the while loop will continue if read result is OK
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo  back
            stream.write(&data[0..size]).unwrap();
            // return true
            true
        },
        Err(err) => {
            println!("Error occurred, terminating connection with {} with error {}", stream.peer_addr().unwrap(), err);
            // shutdown stream connection
            stream.shutdown(Shutdown::Both).unwrap();
            // return false
            false
        }
    } {
        // no while body
    }
}
