use std::io::{Error, Read};
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct Server {
  ip_address: String,
  port: String,
}

impl Server {
  pub fn new(ip_address: String, port: String) -> Server {
    Server { ip_address, port }
  }

  pub fn run_server(&self) -> Result<(), Error> {
    let listener = TcpListener::bind(format!("{}:{}", self.ip_address, self.port))?;

    println!("Server is listening on {}:{}", self.ip_address, self.port);

    for stream in listener.incoming() {
      match stream {
        Ok(mut _stream) => {
          println!("reading stream input !!!");
          let worker_request: String = self.read_worker_request(&mut _stream)?;

          // TODO: sending fragment task //
          println!("Received worker request: {}", worker_request);
        }
        Err(e) => {
          eprintln!("Error accepting connection: {}", e);
        }
      }
    }
    Ok(())
  }

  fn read_worker_request(&self, stream: &mut TcpStream) -> Result<String, Error> {
    // reading the message size
    let mut size_buffer = [0; 4];
    stream.read_exact(&mut size_buffer)?;

    let size = u32::from_be_bytes(size_buffer) as usize;

    // reading the message data //
    let mut buffer = vec![0; size];
    stream.read_exact(&mut buffer)?;

    let request_str = String::from_utf8_lossy(&buffer).to_string();

    Ok(request_str)
  }
}
