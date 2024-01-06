use rand::distributions::Alphanumeric;
use rand::Rng;
use serde_json::Value;
use shared::FragmentTask;
use std::io::{self, Read, Write};
use std::iter;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct Worker {
  pub server_addresse: String,
  pub connexion_name: String,
  pub default_port: u16,
  pub kill_connexion: bool,
}

impl Worker {
  pub fn new(server_addresse: String, default_port: u16, connexion_name: String) -> Worker {
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

  pub fn generate_random_name_with_prefix(prefix: &str) -> String {
    let random_suffix: String = iter::repeat(())
      .map(|()| rand::thread_rng().sample(Alphanumeric) as char)
      .take(5)
      .collect();

    format!("{}{}", prefix, random_suffix)
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
              let fragment_task = self.read_response(&mut _stream).unwrap();
              println!("recived fragment task: {}", fragment_task.to_string());

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
    println!("--- sending fragementRequest ------ ");
    sleep(Duration::from_secs(1));

    let json_message = request.as_str();
    let json_size = json_message.len() as u32;
    println!("json message size: {}", json_size);
    sleep(Duration::from_secs(1));

    let total_size = json_size as usize;
    println!(" totl size : {}", total_size);
    println!("json message: {}", request);
    sleep(Duration::from_secs(1));

    stream.write_all(&(total_size as u32).to_be_bytes())?;
    stream.write_all(&(json_size as u32).to_be_bytes())?;
    stream.write_all(json_message.as_bytes())?;

    Ok(())
  }

  fn read_response(&self, stream: &mut TcpStream) -> Result<FragmentTask, String> {
    println!("--------- reading server response ---------");
    sleep(Duration::from_secs(1));

    let mut total_size_buffer = [0; 4];
    stream
      .read_exact(&mut total_size_buffer)
      .map_err(|e| format!("failed to parse message size : {}", e))?;

    let mut json_size_buffer = [0; 4];
    stream
      .read_exact(&mut json_size_buffer)
      .map_err(|e| format!("failed to parse size buffer : {}", e))?;
    let json_size = u32::from_be_bytes(json_size_buffer) as usize;

    let mut json_buffer = vec![0; json_size];
    stream
      .read_exact(&mut json_buffer)
      .map_err(|e| format!("failed to parse json message: {}", e))?;
    let json_message = String::from_utf8_lossy(&json_buffer);

    let json_value: Value = serde_json::from_str(&json_message)
      .map_err(|e| format!("Failed to parse json object : {}", e))?;

    if let Some(fragment_task_value) = json_value.get("FragmentTask") {
      let fragment_task: Result<FragmentTask, _> =
        serde_json::from_value(fragment_task_value.clone())
          .map_err(|e| format!("Failed to get JSON object: {}", e));

      match fragment_task {
        Ok(task) => Ok(task),
        Err(err) => {
          eprintln!("Failed to deserialize JSON: {}", err);
          Err(err.to_string())
        }
      }
    } else {
      Err("missing fields in json ".to_string())
    }
  }

  pub fn stop_server(&mut self) {
    self.kill_connexion = true;
  }
}
