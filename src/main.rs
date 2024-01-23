use std::{
  io::{self, Write},
  process::exit,
};

use shared::FragmentRequest;
use std::net::Ipv4Addr;
use worker::Worker;

fn read_user_input(prompt: &str) -> String {
  print!("{}", prompt);
  io::stdout().flush().expect("Failed to flush stdout");

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

  input.trim().to_string()
}

fn start(ip_address: &str, port: &str) -> Result<(), Box<dyn std::error::Error>> {
  let connexion_name = read_user_input("enter connexion name: ");
  let work_load = read_user_input("enter work load: ");

  let work_load_u32 = work_load
    .trim()
    .parse::<u32>()
    .expect("error while parssing");
  let mut worker = Worker::new(ip_address.to_string(), connexion_name, port.parse()?);
  let request = FragmentRequest::builder()
    .with_max_work_load(work_load_u32)
    .with_worker_name("worker".to_string())
    .build()?;

  let request_str = request.to_json().map_err(|err| {
    eprintln!("Error transforming to json string: {}", err);
    "Error transforming to json string"
  })?;

  worker.run_worker(request_str);

  Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  loop {
    let user_input = read_user_input("fract >>  ");
    let user_input = user_input.trim();

    match user_input {
      "exit" => {
        exit(0);
      }
      "worker --help" => {
        println!(" worker --help : all commands list");
        println!(" worker generate locally : generate selected fractal locally");
        println!(" worker connect <ip address> : connect to server");
      }
      input if input.starts_with("worker connect ") => {
        let address_port_parts: Vec<&str> = input
          .trim_start_matches("worker connect ")
          .trim()
          .split(':')
          .collect();

        if address_port_parts.len() == 2 {
          let ip_address = address_port_parts[0];
          let port = address_port_parts[1];

          if ip_address.eq("localhost") {
            start(ip_address, port)?;
          } else if let Ok(_ipv4) = ip_address.parse::<Ipv4Addr>() {
            start(ip_address, port)?;
          } else {
            println!("invalid IP address format");
          }
        } else {
          println!("Invalid address format. Use <ip address>:<port>");
        }
      }
      "worker generate locally" => {
        println!("1- all fractals");
        println!("2- Jullia fractal");
        println!("3- Mandelbrot fractal");
      }
      _ => {
        println!("Wrong input! Try --help for more info");
      }
    }
  }
}
