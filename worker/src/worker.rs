use std::io::{self, Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Worker {
  pub server_addresse: String,
  pub connexion_name: String,
  pub default_port: u16,
  pub kill_connexion: bool,
}

impl Worker {
  pub fn new(server_addresse: String, connexion_name: String, default_port: u16) -> Worker {
    Worker {
      server_addresse,
      connexion_name,
      default_port,
      kill_connexion: false,
    }
  }

  pub fn connect_to_server(&mut self) -> io::Result<TcpStream> {
    let server_addr_str = format!("{}:{}", self.server_addresse, self.default_port);
    TcpStream::connect(server_addr_str)
  }

  pub fn run_worker(&mut self, request: String) {
    while !self.kill_connexion {
      match self.connect_to_server() {
        Ok(mut _stream) => {
          println!(
            "connected to server {}:{}",
            self.server_addresse, self.default_port
          );

          match self.send_request(&request, &mut _stream) {
            Ok(_) => {
              // TODO: reading fragement task response
              self.read_response(&mut _stream);

              self.stop_server();
              // TODO: creating new tcp connexion between fragementTask and result until task is valid
            }

            Err(_e) => {
              println!("error while sending request to server ...");
              println!("error: {}", _e);
              self.stop_server();
            }
          }
        }
        Err(e) => {
          print!("error while connectiong to server !");
          println!("error: {}", e);
          self.stop_server();
        }
      }
    }
  }

  fn send_request(&mut self, request: &String, stream: &mut TcpStream) -> Result<(), io::Error> {
    let json_message = request.as_str();
    let json_size = json_message.len() as u32;

    let total_size = json_size as usize;

    stream.write_all(&(total_size as u32).to_be_bytes())?;

    stream.write_all(&(json_size as u32).to_be_bytes())?;

    stream.write_all(json_message.as_bytes())?;

    Ok(())
  }

  fn read_response(&self, stream: &mut TcpStream) {
    let mut sbuf = vec![0_u8; 10000 as usize];
    stream.read(&mut sbuf).unwrap();
    println!("{sbuf:?}");
    let s = String::from_utf8_lossy(&sbuf);
    println!("{s}");
  }

  pub fn stop_server(&mut self) {
    self.kill_connexion = true;
  }
}
