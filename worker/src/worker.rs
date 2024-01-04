use image::{ImageBuffer, Pixel, Rgb};
use serde_json::Value;
use shared::{Complex, FractalDescriptor, FragmentTask, IteratedSinZDescriptor, JuliaDescriptor};
use std::f64::consts::PI;
use std::io::{self, Read, Write};
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

  //#region fractal generation locally
  pub fn generate_fractal_locally(
    &self,
    fractal_descriptor: FractalDescriptor,
    max_iterations: i32,
  ) -> Result<(), image::ImageError> {
    let WIDTH = 1280;
    let HEIGHT = 720;

    match fractal_descriptor {
      FractalDescriptor::Julia(descriptor) => {
        self.generate_julia_fractal_locally(WIDTH, HEIGHT, descriptor, max_iterations)?
      }
      FractalDescriptor::Mandelbrot(_) => {
        self.generate_mandelbot_fractal_locally(WIDTH, HEIGHT, max_iterations)?
      }
      FractalDescriptor::IteratedSinZ(descriptor) => {
        self.generate_sin_z_fractal_locally(WIDTH, HEIGHT, descriptor, max_iterations)?
      }
      _ => println!("Unknown fractal descriptor..."),
    }

    Ok(())
  }

  fn generate_julia_fractal_locally(
    &self,
    width: u32,
    height: u32,
    descriptor: JuliaDescriptor,
    max_iterations: i32,
  ) -> Result<(), image::ImageError> {
    let mut image = ImageBuffer::new(width, height);
    let scale_for_centering_fractal =
      Worker::calculate_scale_for_centering_fractal(width as f64, height as f64);
    let (center_re, center_im, divergence_threshold) = (
      descriptor.c.re,
      descriptor.c.im,
      descriptor.divergence_threshold_square,
    );

    for x in 0..width {
      for y in 0..height {
        let mut zx = scale_for_centering_fractal * (x as f64 - (width as f64 / 2.0));
        let mut zy = scale_for_centering_fractal * (y as f64 - (height as f64 / 2.0));
        let mut i = max_iterations;

        let mut is_within_escape_radius_and_not_max_iterations =
          zx * zx + zy * zy < divergence_threshold && i > 1;
        while is_within_escape_radius_and_not_max_iterations {
          let tmp = zx * zx - zy * zy + center_re;
          zy = 2.0 * zx * zy + center_im;
          zx = tmp;

          i -= 1;
          is_within_escape_radius_and_not_max_iterations =
            zx * zx + zy * zy < divergence_threshold && i > 1
        }

        let pixel = Worker::map_color_julia_fractal_locally(i);
        image.put_pixel(x, y, pixel);
      }
    }

    image.save("generated/images/julia.png")?;
    Ok(())
  }

  fn calculate_scale_for_centering_fractal(width: f64, height: f64) -> f64 {
    let scale_factor = 3.0;
    let min_dimension = width.min(height);

    scale_factor / min_dimension
  }

  fn map_color_julia_fractal_locally(iteration: i32) -> Rgb<u8> {
    let r = (iteration << 3) as u8;
    let g = (iteration << 4) as u8;
    let b = (iteration << 5) as u8;

    Rgb([r, g, b])
  }

  fn generate_mandelbot_fractal_locally(
    &self,
    width: u32,
    height: u32,
    max_iterations: i32,
  ) -> Result<(), image::ImageError> {
    let mut image = ImageBuffer::new(width, height);

    for x in 0..width {
      for y in 0..height {
        let c = Complex::new(
          (x as f64 / width as f64) * 3.5 - 2.5, // Ajustement pour l'échelle et la position
          (y as f64 / height as f64) * 2.0 - 1.0, // du fractal dans l'espace
        );

        let (zn, count) = Worker::mandelbrot(c, max_iterations);

        let pixel = Worker::map_color_mandelbrot_fractal_locally(zn, count);

        image.put_pixel(x, y, pixel);
      }
    }

    image.save("generated/images/mandelbrot.png")?;
    Ok(())
  }

  fn map_color_mandelbrot_fractal_locally(zn: f64, count: f64) -> Rgb<u8> {
    let hue = 0.7 + 0.3 * zn.cos();
    let saturation = 0.6 * count.cos();
    let value = 0.9 * count;

    let red_intensity = (255.0 * hue * saturation) as u8;
    let green_intensity = (255.0 * hue * value) as u8;
    let blue_intensity = (255.0 * value) as u8;

    Rgb([red_intensity, green_intensity, blue_intensity])
  }

  fn mandelbrot(c: Complex, max_iterations: i32) -> (f64, f64) {
    let n = 4.0;
    let mut z = Complex { re: 0.0, im: 0.0 };
    let mut i = 0;

    while z.square_norm() <= n && i < max_iterations {
      z = z.square().add(c);
      i += 1;
    }

    let zn = z.square_norm() / n;
    let count = i as f64 / max_iterations as f64;

    (zn, count)
  }

  fn generate_sin_z_fractal_locally(
    &self,
    width: u32,
    height: u32,
    descriptor: IteratedSinZDescriptor,
    max_iterations: i32,
  ) -> Result<(), image::ImageError> {
    let mut image = ImageBuffer::new(width, height);
    let n = 50.0;

    let scale_re = 8.0;
    let scale_im = scale_re * (height as f64 / width as f64);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
      let cx = (x as f64 / width as f64 - 0.5) * scale_re;
      let cy = (y as f64 / height as f64 - 0.5) * scale_im;

      let mut z = Complex::new(cx, cy);

      let mut i = 0;
      while z.square_norm() <= n && i < max_iterations {
        z = z.sin().mul(&descriptor.c);
        i += 1;
      }

      let zn = z.square_norm() / n;
      let count = i as f64 / max_iterations as f64;

      *pixel = Worker::map_color_sin_z_fractal_locally(zn, count)
    }

    image.save("generated/images/sin_z_fractal.png")?;
    Ok(())
  }

  fn map_color_sin_z_fractal_locally(zn: f64, count: f64) -> Rgb<u8> {
    let hue = 0.5 + 0.5 * (zn * 2.0 * PI).cos();
    let saturation = 0.6 + 0.4 * (count * 2.0 * PI).cos();
    let value = 0.7 + 0.3 * (count * 2.0 * PI).sin();

    let r = (240.0 * hue) as u8;
    let g = (240.0 * saturation) as u8;
    let b = (240.0 * value) as u8;

    Rgb([r, g, b])
  }
  //#endregion
}
