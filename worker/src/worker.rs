use fractal_old::{Fractal, FractalJulia};
use image::EncodableLayout;
use local_fractal::{generate_all_fractal_models_locally, generate_fractal_locally};
use serde_json::Value;
use shared::{FractalDescriptor, FragmentResult, FragmentTask, PixelIntensity, Range, Resolution};
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
  // constructor
  pub fn new(server_addresse: String, connexion_name: String, default_port: u16) -> Worker {
    Worker {
      server_addresse,
      connexion_name,
      default_port,
      kill_connexion: false,
    }
  }

  // starting new connexion with the server
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
              // getting fragment_task and the task id
              let (mut fragment_task, mut id) = self
                .read_response(&mut _stream)
                .expect("error: unable to read server fragmentTask !!");

              // as long as fragmentResult is valide sending results and reciving tasks from the server
              loop {
                // creating a new connexion with the server (nominal loop)
                let mut inner_connexion = self.connect_to_server().unwrap();
                println!(" fragment task: {}", fragment_task.to_string());

                let fractal_type = fragment_task.fractal;

                // checking the fragment type
                match fractal_type {
                  FractalDescriptor::Julia(_) => {
                    println!("starting genration for julia fractal ...");

                    // calculating the pixles for the julia fractal and sending the results to server
                    let julia = FractalJulia::new();
                    let pixels = julia.generate(&fragment_task, &fragment_task.fractal);

                    // calculating data bloc offset and count
                    let data_id = fragment_task.id.offset + fragment_task.id.count;
                    let data_offset =
                      fragment_task.resolution.nx as u32 * fragment_task.resolution.ny as u32;

                    // bulding fragment result
                    let fragment_result = FragmentResult::builder()
                      .with_id(fragment_task.id.offset, fragment_task.id.count)
                      .with_resolution(fragment_task.resolution.nx, fragment_task.resolution.ny)
                      .with_range(
                        fragment_task.range.min.x,
                        fragment_task.range.min.y,
                        fragment_task.range.max.x,
                        fragment_task.range.max.y,
                      )
                      .with_pixels(data_id, data_offset)
                      .build()
                      .expect("error while building fragmentResult");

                    // sending fragment result to server
                    self
                      .send_fragment_result(&fragment_result, &mut inner_connexion, id, pixels)
                      .expect("error while sending");

                    // reading the server new task
                    let result = self.read_response(&mut inner_connexion).unwrap();
                    fragment_task = result.0;
                    id = result.1;
                  }
                  FractalDescriptor::Mandelbrot(_) => {
                    todo!("Mandelbrot is not implimented yet")
                  }
                  FractalDescriptor::IteratedSinZ(_) => {
                    todo!("IteratedSinZ not emplimented yet");
                  }
                  FractalDescriptor::NewtonRaphsonZ3(_) => {
                    todo!("NewtonRaphsonZ3 not emplimented yet");
                  }
                  FractalDescriptor::NewtonRaphsonZ4(_) => {
                    todo!("NewtonRaphsonZ4 not emplimented yet");
                  }

                  FractalDescriptor::NovaNewtonZ4(_) => {
                    todo!("NovaNewtonZ4 not emplimented yet");
                  }
                  FractalDescriptor::NovaNewtonZ3(_) => {
                    todo!("NovaNewtonZ3 not emplimented yet");
                  }
                }
              }
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

  fn send_fragment_result(
    &mut self,
    result: &FragmentResult,
    stream: &mut TcpStream,
    id: Vec<u8>,
    pixels: Vec<PixelIntensity>,
  ) -> Result<(), io::Error> {
    let serialized = result
      .to_json()
      .expect("error while serializing fragmentResult");
    let json_bytes = serialized.as_bytes();
    let msg_len: u32 = json_bytes.len() as u32;
    let total_msg_len: u32 = msg_len + (result.pixels.offset + result.pixels.count * (4 + 4));
    let a = total_msg_len.to_be_bytes();
    stream.write(&a).expect("Could not write to stream");
    let b = msg_len.to_be_bytes();
    stream.write(&b).expect("Could not write to stream");
    stream.write(json_bytes).expect("Could not write to stream");
    stream
      .write(&id.as_bytes())
      .expect("Could not write to stream");
    for pixel in pixels {
      let zn = pixel.zn;
      let count = pixel.count;

      stream
        .write(&zn.to_be_bytes())
        .expect("Could not write to stream");
      stream
        .write(&count.to_be_bytes())
        .expect("Could not write to stream");
    }
    println!("fragment result {:?} send succussfuly", result);
    Ok(())
  }

  fn send_request(&mut self, request: &String, stream: &mut TcpStream) -> Result<(), io::Error> {
    println!("--- sending fragementRequest ------ ");

    let json_message = request.as_str();
    let json_size = json_message.len() as u32;
    println!("json message size: {}", json_size);

    let total_size = json_size as usize;
    println!(" totl size : {}", total_size);
    println!("json message: {}", request);

    stream.write_all(&(total_size as u32).to_be_bytes())?;
    stream.write_all(&(json_size as u32).to_be_bytes())?;
    stream.write_all(json_message.as_bytes())?;

    Ok(())
  }

  fn read_response(&self, stream: &mut TcpStream) -> Result<(FragmentTask, Vec<u8>), String> {
    println!("--------- reading server response ---------");

    let mut total_size_buffer = [0; 4];
    stream
      .read_exact(&mut total_size_buffer)
      .map_err(|e| format!("failed to parse total message size: {}", e))?;
    let total_size = u32::from_be_bytes(total_size_buffer) as usize;

    let mut json_size_buffer = [0; 4];
    stream
      .read_exact(&mut json_size_buffer)
      .map_err(|e| format!("failed to parse json message size: {}", e))?;
    let json_size = u32::from_be_bytes(json_size_buffer) as usize;

    let mut json_buffer = vec![0; json_size];
    stream
      .read_exact(&mut json_buffer)
      .map_err(|e| format!("failed to parse json message: {}", e))?;
    let json_message = String::from_utf8_lossy(&json_buffer);

    let mut data_buffer = vec![0; total_size - json_size];
    stream
      .read_exact(&mut data_buffer)
      .map_err(|e| format!("failed to parse data buffer: {}", e))?;

    let json_value: Value = serde_json::from_str(&json_message)
      .map_err(|e| format!("Failed to parse json object : {}", e))?;

    if let Some(fragment_task_value) = json_value.get("FragmentTask") {
      let fragment_task: Result<FragmentTask, _> =
        serde_json::from_value(fragment_task_value.clone())
          .map_err(|e| format!("Failed to get JSON object: {}", e));

      match fragment_task {
        Ok(task) => Ok((task, data_buffer)),
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

  pub fn generate_all_fractal_models_locally(
    &self,
    resolution: &Resolution,
    max_iterations: i32,
  ) -> Result<(), Box<dyn std::error::Error>> {
    generate_all_fractal_models_locally(&resolution, max_iterations)?;
    Ok(())
  }

  pub fn generate_fractal_locally(
    &self,
    resolution: &Resolution,
    range: &Range,
    fractal_descriptor: FractalDescriptor,
    max_iterations: i32,
  ) -> Result<(), Box<dyn std::error::Error>> {
    generate_fractal_locally(&resolution, &range, fractal_descriptor, max_iterations)?;
    Ok(())
  }
}
