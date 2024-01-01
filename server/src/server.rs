use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct Server {
  ip_addresse: String,
  connexion_port: String,
}

impl Server {
  pub fn new(ip_addresse: String, connexion_port: String) -> Server {
    Server {
      ip_addresse,
      connexion_port,
    }
  }

  pub fn run_server(&self) -> io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", self.ip_addresse, self.connexion_port))?;

    println!(
      "Server is listening on {}:{}",
      self.ip_addresse, self.connexion_port
    );

    for stream in listener.incoming() {
      match stream {
        Ok(mut _stream) => {
          println!("reading stream input !!!");
          let worker_request: String = self.read_worker_request(&mut _stream).unwrap();
          print!("{}", worker_request);

          // TODO: sending fragment task //
        }
        Err(e) => {
          eprintln!("Error accepting connection: {}", e);
        }
      }
    }
    Ok(())
  }

  fn read_worker_request(&self, stream: &mut TcpStream) -> io::Result<String> {
    // reading the message size
    let mut size_buffer = [0; 4];
    stream.read_exact(&mut size_buffer).unwrap();
    let size = u32::from_be_bytes(size_buffer) as usize;

    // reading the message data //
    let mut buffer = vec![0; size];
    stream.read_exact(&mut buffer).unwrap();
    let request_str = String::from_utf8_lossy(&buffer).to_string();

    Ok(request_str)
  }
}
