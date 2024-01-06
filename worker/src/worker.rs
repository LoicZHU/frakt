use image::{ImageBuffer, Rgb};
use serde_json::Value;
use shared::{
  Complex, FractalDescriptor, FragmentTask, IteratedSinZDescriptor, JuliaDescriptor,
  PixelIntensity, Range, Resolution,
};
use std::f32::consts::PI;
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
    resolution: &Resolution,
    range: &Range,
    fractal_descriptor: FractalDescriptor,
    max_iterations: i32,
  ) -> Result<(), image::ImageError> {
    match fractal_descriptor {
      FractalDescriptor::Julia(descriptor) => {
        self.generate_julia_fractal_locally(&resolution, &range, &descriptor, max_iterations)?
      }
      FractalDescriptor::Mandelbrot(_) => {
        self.generate_mandelbrot_fractal_locally(&resolution, &range, max_iterations)?
      }
      FractalDescriptor::IteratedSinZ(descriptor) => self.generate_iterated_sin_z_fractal_locally(
        &resolution,
        &range,
        &descriptor,
        max_iterations,
      )?,
      FractalDescriptor::NewtonRaphsonZ3(_) => {
        let epsilon = 1e-6;
        let polynomial = |z: &Complex| {
          let z_squared = z.mul(&z);
          let z_cubed = z_squared.mul(&z);
          z_cubed.sub(&Complex::new(1.0, 0.0))
        };

        let derivative = |z: &Complex| {
          let z_squared = z.mul(&z);
          Complex::new(3.0 * z_squared.re, 3.0 * z_squared.im)
        };

        self.generate_newton_fractal_locally(
          &resolution,
          &range,
          &polynomial,
          &derivative,
          max_iterations,
          epsilon,
          "newton_fractal_z3",
        )?
      }
      FractalDescriptor::NewtonRaphsonZ4(_) => {
        let epsilon = 1e-6;
        let polynomial = |z: &Complex| {
          let z_squared = z.mul(&z);
          let z_fourth = z_squared.mul(&z_squared);
          z_fourth.sub(&Complex::new(1.0, 0.0))
        };

        let derivative = |z: &Complex| {
          let z_squared = z.mul(&z);
          let z_cubed = z_squared.mul(&z);
          Complex::new(4.0 * z_cubed.re, 4.0 * z_cubed.im)
        };

        self.generate_newton_fractal_locally(
          &resolution,
          &range,
          &polynomial,
          &derivative,
          max_iterations,
          epsilon,
          "newton_fractal_z4",
        )?
      }
      FractalDescriptor::NovaNewtonZ3(_) => {
        let epsilon = 1e-6;
        let polynomial = |z: &Complex| {
          let z_squared = z.mul(&z);
          let z_cubed = z_squared.mul(&z);
          z_cubed.sub(&Complex::new(1.0, 0.0)) // z^3 - 1
        };

        let derivative = |z: &Complex| {
          let z_squared = z.mul(&z);
          Complex::new(3.0 * z_squared.re, 3.0 * z_squared.im) // 3z^2
        };

        self.generate_nova_newton_fractal_locally(
          &resolution,
          &range,
          &polynomial,
          &derivative,
          max_iterations,
          epsilon,
          "nova_newton_fractal_z3",
        )?
      }
      FractalDescriptor::NovaNewtonZ4(_) => {
        let epsilon = 1e-6;
        let polynomial = |z: &Complex| {
          let z_squared = z.mul(&z);
          let z_fourth = z_squared.mul(&z_squared);
          z_fourth.sub(&Complex::new(1.0, 0.0)) // z^4 - 1
        };

        let derivative = |z: &Complex| {
          let z_squared = z.mul(&z);
          let z_cubed = z_squared.mul(&z);
          Complex::new(4.0 * z_cubed.re, 4.0 * z_cubed.im) // 4z^3
        };

        self.generate_nova_newton_fractal_locally(
          &resolution,
          &range,
          &polynomial,
          &derivative,
          max_iterations,
          epsilon,
          "nova_newton_fractal_z4",
        )?
      } // _ => println!("Unknown fractal descriptor..."),
    }

    Ok(())
  }

  fn generate_julia_fractal_locally(
    &self,
    resolution: &Resolution,
    range: &Range,
    descriptor: &JuliaDescriptor,
    max_iterations: i32,
  ) -> Result<(), image::ImageError> {
    let (width, height) = (resolution.nx, resolution.ny);
    let mut image = ImageBuffer::new(width as u32, height as u32);
    let (center_re, center_im, divergence_threshold) = (
      descriptor.c.re,
      descriptor.c.im,
      descriptor.divergence_threshold_square,
    );

    let scale_x = (range.max.x - range.min.x) / width as f64;
    let scale_y = (range.max.y - range.min.y) / height as f64;

    for x in 0..width {
      for y in 0..height {
        let cx = range.min.x + x as f64 * scale_x;
        let cy = range.min.y + y as f64 * scale_y;
        let mut zx = cx;
        let mut zy = cy;
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

        let pixel_intensity = PixelIntensity {
          zn: (zx * zx + zy * zy / divergence_threshold) as f32,
          count: i as f32 / max_iterations as f32,
        };

        let pixel = Worker::map_color_julia_fractal_locally(&pixel_intensity);
        image.put_pixel(x as u32, y as u32, pixel);
      }
    }

    image.save("generated/images/julia.png")?;
    Ok(())
  }

  fn map_color_julia_fractal_locally(pixel_intensity: &PixelIntensity) -> Rgb<u8> {
    let scaled_count = (pixel_intensity.count * 255.0) as i32;
    let zn_effect = (pixel_intensity.zn * 10.0).sin().abs() * 5.0; // very minimal effect

    let r = (((scaled_count << 3) as u8) as f32 + zn_effect).min(255.0) as u8;
    let g = (((scaled_count << 4) as u8) as f32 + zn_effect).min(255.0) as u8;
    let b = (((scaled_count << 5) as u8) as f32 + zn_effect).min(255.0) as u8;

    Rgb([r, g, b])
  }

  fn generate_mandelbrot_fractal_locally(
    &self,
    resolution: &Resolution,
    range: &Range,
    max_iterations: i32,
  ) -> Result<(), image::ImageError> {
    let (width, height) = (resolution.nx, resolution.ny);
    let mut image = ImageBuffer::new(width as u32, height as u32);

    let scale_x = (range.max.x - range.min.x) / width as f64;
    let scale_y = (range.max.y - range.min.y) / height as f64;

    for x in 0..width {
      for y in 0..height {
        let cx = range.min.x + (x as f64) * scale_x;
        let cy = range.min.y + (y as f64) * scale_y;
        let c = Complex::new(cx, cy);

        let pixel_intensity = Worker::mandelbrot(&c, max_iterations);
        let pixel = Worker::map_color_mandelbrot_fractal_locally(&pixel_intensity);

        image.put_pixel(x as u32, y as u32, pixel);
      }
    }

    image.save("generated/images/mandelbrot.png")?;
    Ok(())
  }

  fn map_color_mandelbrot_fractal_locally(pixel_intensity: &PixelIntensity) -> Rgb<u8> {
    let (zn, count) = (pixel_intensity.zn, pixel_intensity.count);

    let hue = 0.7 + 0.3 * zn.cos();
    let saturation = 0.6 * count.cos();
    let value = 0.9 * count;

    let red_intensity = (255.0 * hue * saturation) as u8;
    let green_intensity = (255.0 * hue * value) as u8;
    let blue_intensity = (255.0 * value) as u8;

    Rgb([red_intensity, green_intensity, blue_intensity])
  }

  fn mandelbrot(c: &Complex, max_iterations: i32) -> PixelIntensity {
    let n = 4.0;
    let mut z = Complex { re: 0.0, im: 0.0 };
    let mut i = 0;

    while z.square_norm() <= n && i < max_iterations {
      z = z.square().add(&c);
      i += 1;
    }

    let zn = z.square_norm() as f32 / n as f32;
    let count = i as f32 / max_iterations as f32;

    PixelIntensity { zn, count }
  }

  fn generate_iterated_sin_z_fractal_locally(
    &self,
    resolution: &Resolution,
    range: &Range,
    descriptor: &IteratedSinZDescriptor,
    max_iterations: i32,
  ) -> Result<(), image::ImageError> {
    let (width, height) = (resolution.nx, resolution.ny);
    let mut image = ImageBuffer::new(width as u32, height as u32);
    let n = 50.0;

    let scale_re = (range.max.x - range.min.x) / width as f64;
    let scale_im = (range.max.y - range.min.y) / height as f64;

    for (x, y, pixel) in image.enumerate_pixels_mut() {
      let cx = range.min.x + x as f64 * scale_re;
      let cy = range.min.y + y as f64 * scale_im;

      let mut z = Complex::new(cx, cy);

      let mut i = 0;
      while z.square_norm() <= n && i < max_iterations {
        z = z.sin().mul(&descriptor.c);
        i += 1;
      }

      let pixel_intensity = PixelIntensity {
        zn: (z.square_norm() / n) as f32,
        count: i as f32 / max_iterations as f32,
      };

      *pixel = Worker::map_color_sin_z_fractal_locally(&pixel_intensity)
    }

    image.save("generated/images/iterated_sin_z_fractal.png")?;
    Ok(())
  }

  fn map_color_sin_z_fractal_locally(pixel_intensity: &PixelIntensity) -> Rgb<u8> {
    let (zn, count) = (pixel_intensity.zn, pixel_intensity.count);

    let hue = 0.5 + 0.5 * (zn * 2.0 * PI).cos();
    let saturation = 0.6 + 0.4 * (count * 2.0 * PI).cos();
    let value = 0.7 + 0.3 * (count * 2.0 * PI).sin();

    let r = (240.0 * hue) as u8;
    let g = (240.0 * saturation) as u8;
    let b = (240.0 * value) as u8;

    Rgb([r, g, b])
  }

  fn generate_nova_newton_fractal_locally(
    &self,
    resolution: &Resolution,
    range: &Range,
    polynomial: &dyn Fn(&Complex) -> Complex, // p(z)
    derivative: &dyn Fn(&Complex) -> Complex, // p'(z)
    max_iterations: i32,
    epsilon: f64,
    generated_image_name: &str,
  ) -> Result<(), image::ImageError> {
    let (width, height) = (resolution.nx, resolution.ny);
    let mut image = ImageBuffer::new(width as u32, height as u32);

    let scale_x = (range.max.x - range.min.x) / width as f64;
    let scale_y = (range.max.y - range.min.y) / height as f64;

    for x in 0..width {
      for y in 0..height {
        let cx = range.min.x + x as f64 * scale_x;
        let cy = range.min.y + y as f64 * scale_y;

        let mut z = Complex::new(1.0, 0.0);
        let c = Complex::new(cx, cy);

        let mut i = 0;
        while i < max_iterations {
          let next_z = z.sub(&polynomial(&z).div(derivative(&z))).add(&c);
          if (next_z.sub(&z)).square_norm() < epsilon {
            break;
          }

          z = next_z;
          i += 1;
        }

        let pixel_intensity = PixelIntensity {
          zn: 0.0,
          count: i as f32 / max_iterations as f32,
        };

        let pixel = Worker::map_color_nova_newton_z3_locally(&pixel_intensity);
        image.put_pixel(x as u32, y as u32, pixel);
      }
    }

    let generated_image_path = format!("generated/images/{}.png", &generated_image_name);
    image.save(generated_image_path)?;

    Ok(())
  }

  fn map_color_nova_newton_z3_locally(pixel_intensity: &PixelIntensity) -> Rgb<u8> {
    let normalized_count = pixel_intensity.count;
    let slight_variation = (pixel_intensity.zn * 5.0).sin().abs() * 0.1;

    let red_to_yellow = Rgb([
      (255.0 * normalized_count * 6.0) as u8,
      (255.0 * slight_variation) as u8,
      0,
    ]);
    let yellow_to_green = Rgb([
      255,
      (255.0 * (normalized_count * 6.0 - 1.0)) as u8,
      (255.0 * slight_variation) as u8,
    ]);
    let green_to_cyan = Rgb([
      (255.0 * (1.0 - (normalized_count * 6.0 - 2.0))) as u8,
      255,
      (255.0 * slight_variation) as u8,
    ]);
    let cyan_to_blue = Rgb([
      (255.0 * slight_variation) as u8,
      255,
      (255.0 * (normalized_count * 6.0 - 3.0)) as u8,
    ]);
    let blue_to_magenta = Rgb([
      (255.0 * slight_variation) as u8,
      (255.0 * (1.0 - (normalized_count * 6.0 - 4.0))) as u8,
      255,
    ]);
    let magenta_to_red = Rgb([
      (255.0 * (normalized_count * 6.0 - 5.0)) as u8,
      (255.0 * slight_variation) as u8,
      255,
    ]);

    let color = match (normalized_count * 6.0) as i32 {
      0 => red_to_yellow,
      1 => yellow_to_green,
      2 => green_to_cyan,
      3 => cyan_to_blue,
      4 => blue_to_magenta,
      _ => magenta_to_red,
    };

    color
  }

  fn generate_newton_fractal_locally(
    &self,
    resolution: &Resolution,
    range: &Range,
    polynomial: &dyn Fn(&Complex) -> Complex,
    derivative: &dyn Fn(&Complex) -> Complex,
    max_iterations: i32,
    epsilon: f64,
    generated_image_name: &str,
  ) -> Result<(), image::ImageError> {
    let (width, height) = (resolution.nx, resolution.ny);
    let mut image = ImageBuffer::new(width as u32, height as u32);

    let scale_x = (range.max.x - range.min.x) / width as f64;
    let scale_y = (range.max.y - range.min.y) / height as f64;

    for x in 0..width {
      for y in 0..height {
        let cx = range.min.x + x as f64 * scale_x;
        let cy = range.min.y + y as f64 * scale_y;

        let mut z = Complex::new(cx, cy);
        let mut i = 0;
        let mut pzn = polynomial(&z).square_norm();

        while i < max_iterations && pzn > epsilon {
          z = z.sub(&polynomial(&z).div(derivative(&z)));
          pzn = polynomial(&z).square_norm();
          i += 1;
        }

        let pixel_intensity = PixelIntensity {
          zn: (0.5 + z.arg() / (2.0 * std::f64::consts::PI)).fract() as f32,
          count: Worker::get_convergence_value(pzn as f32, epsilon, i, max_iterations),
        };

        let pixel = Worker::map_color_newton_locally(&pixel_intensity);
        image.put_pixel(x as u32, y as u32, pixel);
      }
    }

    let generated_image_path = format!("generated/images/{}.png", generated_image_name);
    image.save(generated_image_path)?;

    Ok(())
  }

  fn get_convergence_value(pzn: f32, threshold: f64, count: i32, nmax: i32) -> f32 {
    let accuracy = f32::log10(threshold as f32);

    if count >= nmax {
      1.0
    } else {
      0.5 - 0.5 * f32::cos(0.1 * (count as f32 - (f32::log10(pzn) / accuracy)))
    }
  }

  fn map_color_newton_locally(pixel_intensity: &PixelIntensity) -> Rgb<u8> {
    let start_red_color = Rgb([255, 0, 0]);
    let end_blue_color = Rgb([0, 0, 255]);
    let zn_effect = (pixel_intensity.zn * 10.0).sin().abs() * 0.05;

    let r = ((start_red_color[0] as f32 * (1.0 - pixel_intensity.count)
      + end_blue_color[0] as f32 * pixel_intensity.count)
      * (1.0 - zn_effect)) as u8;
    let g = ((start_red_color[1] as f32 * (1.0 - pixel_intensity.count)
      + end_blue_color[1] as f32 * pixel_intensity.count)
      * (1.0 - zn_effect)) as u8;
    let b = ((start_red_color[2] as f32 * (1.0 - pixel_intensity.count)
      + end_blue_color[2] as f32 * pixel_intensity.count)
      * (1.0 - zn_effect)) as u8;

    Rgb([r, g, b])
  }

  //#endregion
}
