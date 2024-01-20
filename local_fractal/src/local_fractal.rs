use complex::ComplexTrait;
use image::{ImageBuffer, ImageError, Rgb};
use shared::{
  ComplexOld, FractalDescriptor, IteratedSinZDescriptor, JuliaDescriptor, MandelbrotDescriptor,
  NewtonRaphsonZ3Descriptor, NewtonRaphsonZ4Descriptor, NovaNewtonRaphsonZ3Descriptor,
  NovaNewtonRaphsonZ4Descriptor, PixelIntensity, Point, Range, Resolution,
};
use std::f32::consts::PI;

pub fn generate_all_fractal_models_locally(
  resolution: &Resolution,
  max_iterations: i32,
) -> Result<(), Box<dyn std::error::Error>> {
  generate_julia_fractal_with_defined_range_and_descriptor(&resolution, max_iterations)?;
  generate_iterated_sin_z_fractal_with_defined_range_and_descriptor(&resolution, max_iterations)?;
  generate_mandelbrot_fractal_with_defined_range(&resolution, max_iterations)?;

  generate_newton_z3_fractal_with_defined_range(&resolution, max_iterations)?;
  generate_newton_z4_fractal_with_defined_range(&resolution, max_iterations)?;

  generate_nova_newton_z3_fractal_with_defined_range(&resolution, max_iterations)?;
  generate_nova_newton_z4_fractal_with_defined_range(&resolution, max_iterations)?;

  Ok(())
}

//#region generation functions with defined range (and fractal descriptor)
pub fn generate_julia_fractal_with_defined_range_and_descriptor(
  resolution: &Resolution,
  max_iterations: i32,
) -> Result<(), Box<dyn std::error::Error>> {
  let range = Range {
    min: Point { x: -4.0, y: -3.0 },
    max: Point { x: 4.0, y: 3.0 },
  };

  let _julia_descriptor_1 = JuliaDescriptor {
    c: ComplexOld {
      re: 0.285,
      im: 0.013,
    },
    divergence_threshold_square: 4.0,
  };
  let _julia_descriptor_2 = JuliaDescriptor {
    c: ComplexOld {
      re: -0.9,
      im: 0.27015,
    },
    divergence_threshold_square: 4.0,
  };

  generate_fractal_locally(
    &resolution,
    &range,
    FractalDescriptor::Julia(_julia_descriptor_2),
    max_iterations,
  )?;

  Ok(())
}

pub fn generate_mandelbrot_fractal_with_defined_range(
  resolution: &Resolution,
  max_iterations: i32,
) -> Result<(), Box<dyn std::error::Error>> {
  let mandelbrot_range = Range {
    min: Point { x: -2.0, y: -1.25 },
    max: Point { x: 1.0, y: 1.25 },
  };

  generate_fractal_locally(
    &resolution,
    &mandelbrot_range,
    FractalDescriptor::Mandelbrot(MandelbrotDescriptor {}),
    max_iterations,
  )?;

  Ok(())
}

pub fn generate_iterated_sin_z_fractal_with_defined_range_and_descriptor(
  resolution: &Resolution,
  max_iterations: i32,
) -> Result<(), Box<dyn std::error::Error>> {
  let range = Range {
    min: Point { x: -4.0, y: -3.0 },
    max: Point { x: 4.0, y: 3.0 },
  };

  let _sin_z_descriptor_1 = IteratedSinZDescriptor {
    c: ComplexOld { re: 1.0, im: 0.3 },
  };
  let _sin_z_descriptor_2 = IteratedSinZDescriptor {
    c: ComplexOld { re: 0.2, im: 1.0 },
  };

  generate_fractal_locally(
    &resolution,
    &range,
    FractalDescriptor::IteratedSinZ(_sin_z_descriptor_2),
    max_iterations,
  )?;

  Ok(())
}

pub fn generate_newton_z3_fractal_with_defined_range(
  resolution: &Resolution,
  max_iterations: i32,
) -> Result<(), Box<dyn std::error::Error>> {
  let range = Range {
    min: Point { x: -4.0, y: -3.0 },
    max: Point { x: 4.0, y: 3.0 },
  };

  generate_fractal_locally(
    &resolution,
    &range,
    FractalDescriptor::NewtonRaphsonZ3(NewtonRaphsonZ3Descriptor {}),
    max_iterations,
  )?;

  Ok(())
}

pub fn generate_newton_z4_fractal_with_defined_range(
  resolution: &Resolution,
  max_iterations: i32,
) -> Result<(), Box<dyn std::error::Error>> {
  let range = Range {
    min: Point { x: -4.0, y: -3.0 },
    max: Point { x: 4.0, y: 3.0 },
  };

  generate_fractal_locally(
    &resolution,
    &range,
    FractalDescriptor::NewtonRaphsonZ4(NewtonRaphsonZ4Descriptor {}),
    max_iterations,
  )?;

  Ok(())
}

pub fn generate_nova_newton_z3_fractal_with_defined_range(
  resolution: &Resolution,
  max_iterations: i32,
) -> Result<(), Box<dyn std::error::Error>> {
  let nova_newton_z3_range = Range {
    min: Point { x: -2.0, y: -1.5 },
    max: Point { x: 2.0, y: 1.5 },
  };

  generate_fractal_locally(
    &resolution,
    &nova_newton_z3_range,
    FractalDescriptor::NovaNewtonZ3(NovaNewtonRaphsonZ3Descriptor {}),
    max_iterations,
  )?;

  Ok(())
}

pub fn generate_nova_newton_z4_fractal_with_defined_range(
  resolution: &Resolution,
  max_iterations: i32,
) -> Result<(), Box<dyn std::error::Error>> {
  let nova_newton_z4_range = Range {
    min: Point { x: -2.5, y: -1.5 },
    max: Point { x: 2.0, y: 1.5 },
  };

  generate_fractal_locally(
    &resolution,
    &nova_newton_z4_range,
    FractalDescriptor::NovaNewtonZ4(NovaNewtonRaphsonZ4Descriptor {}),
    max_iterations,
  )?;

  Ok(())
}
//#endregion

pub fn generate_fractal_locally(
  resolution: &Resolution,
  range: &Range,
  fractal_descriptor: FractalDescriptor,
  max_iterations: i32,
) -> Result<(), ImageError> {
  match fractal_descriptor {
    FractalDescriptor::Julia(descriptor) => {
      generate_julia_fractal_locally(&resolution, &range, &descriptor, max_iterations)?
    }
    FractalDescriptor::Mandelbrot(_) => {
      generate_mandelbrot_fractal_locally(&resolution, &range, max_iterations)?
    }
    FractalDescriptor::IteratedSinZ(descriptor) => {
      generate_iterated_sin_z_fractal_locally(&resolution, &range, &descriptor, max_iterations)?
    }
    FractalDescriptor::NewtonRaphsonZ3(_) => {
      let epsilon = 1e-6;
      let polynomial = |z: &ComplexOld| {
        let z_squared = z.multiply(&z);
        let z_cubed = z_squared.multiply(&z);
        z_cubed.subtract(&ComplexOld::new(1.0, 0.0))
      };

      let derivative = |z: &ComplexOld| {
        let z_squared = z.multiply(&z);
        ComplexOld::new(3.0 * z_squared.re, 3.0 * z_squared.im)
      };

      generate_newton_fractal_locally(
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
      let polynomial = |z: &ComplexOld| {
        let z_squared = z.multiply(&z);
        let z_fourth = z_squared.multiply(&z_squared);
        z_fourth.subtract(&ComplexOld::new(1.0, 0.0))
      };

      let derivative = |z: &ComplexOld| {
        let z_squared = z.multiply(&z);
        let z_cubed = z_squared.multiply(&z);
        ComplexOld::new(4.0 * z_cubed.re, 4.0 * z_cubed.im)
      };

      generate_newton_fractal_locally(
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
      let polynomial = |z: &ComplexOld| {
        let z_squared = z.multiply(&z);
        let z_cubed = z_squared.multiply(&z);
        z_cubed.subtract(&ComplexOld::new(1.0, 0.0)) // z^3 - 1
      };

      let derivative = |z: &ComplexOld| {
        let z_squared = z.multiply(&z);
        ComplexOld::new(3.0 * z_squared.re, 3.0 * z_squared.im) // 3z^2
      };

      generate_nova_newton_fractal_locally(
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
      let polynomial = |z: &ComplexOld| {
        let z_squared = z.multiply(&z);
        let z_fourth = z_squared.multiply(&z_squared);
        z_fourth.subtract(&ComplexOld::new(1.0, 0.0)) // z^4 - 1
      };

      let derivative = |z: &ComplexOld| {
        let z_squared = z.multiply(&z);
        let z_cubed = z_squared.multiply(&z);
        ComplexOld::new(4.0 * z_cubed.re, 4.0 * z_cubed.im) // 4z^3
      };

      generate_nova_newton_fractal_locally(
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

//#region model: Julia
pub fn generate_julia_fractal_locally(
  resolution: &Resolution,
  range: &Range,
  descriptor: &JuliaDescriptor,
  max_iterations: i32,
) -> Result<(), ImageError> {
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

      let pixel = map_color_julia_fractal_locally(&pixel_intensity);
      image.put_pixel(x as u32, y as u32, pixel);
    }
  }

  image.save("worker/generated/images/julia.png")?;
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
//#endregion

//#region model: Mandelbrot
pub fn generate_mandelbrot_fractal_locally(
  resolution: &Resolution,
  range: &Range,
  max_iterations: i32,
) -> Result<(), ImageError> {
  let (width, height) = (resolution.nx, resolution.ny);
  let mut image = ImageBuffer::new(width as u32, height as u32);

  let scale_x = (range.max.x - range.min.x) / width as f64;
  let scale_y = (range.max.y - range.min.y) / height as f64;

  for x in 0..width {
    for y in 0..height {
      let cx = range.min.x + (x as f64) * scale_x;
      let cy = range.min.y + (y as f64) * scale_y;
      let c = ComplexOld::new(cx, cy);

      let pixel_intensity = mandelbrot(&c, max_iterations);
      let pixel = map_color_mandelbrot_fractal_locally(&pixel_intensity);

      image.put_pixel(x as u32, y as u32, pixel);
    }
  }

  image.save("worker/generated/images/mandelbrot.png")?;
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

fn mandelbrot(c: &ComplexOld, max_iterations: i32) -> PixelIntensity {
  let n = 4.0;
  let mut z = ComplexOld { re: 0.0, im: 0.0 };
  let mut i = 0;

  while z.square_norm() <= n && i < max_iterations {
    z = z.square().add(&c);
    i += 1;
  }

  let zn = z.square_norm() as f32 / n as f32;
  let count = i as f32 / max_iterations as f32;

  PixelIntensity { zn, count }
}
//#endregion

//#region model: Iterated sin Z
pub fn generate_iterated_sin_z_fractal_locally(
  resolution: &Resolution,
  range: &Range,
  descriptor: &IteratedSinZDescriptor,
  max_iterations: i32,
) -> Result<(), ImageError> {
  let (width, height) = (resolution.nx, resolution.ny);
  let mut image = ImageBuffer::new(width as u32, height as u32);
  let n = 50.0;

  let scale_re = (range.max.x - range.min.x) / width as f64;
  let scale_im = (range.max.y - range.min.y) / height as f64;

  for (x, y, pixel) in image.enumerate_pixels_mut() {
    let cx = range.min.x + x as f64 * scale_re;
    let cy = range.min.y + y as f64 * scale_im;

    let mut z = ComplexOld::new(cx, cy);

    let mut i = 0;
    while z.square_norm() <= n && i < max_iterations {
      z = z.sine().multiply(&descriptor.c);
      i += 1;
    }

    let pixel_intensity = PixelIntensity {
      zn: (z.square_norm() / n) as f32,
      count: i as f32 / max_iterations as f32,
    };

    *pixel = map_color_sin_z_fractal_locally(&pixel_intensity)
  }

  image.save("worker/generated/images/iterated_sin_z_fractal.png")?;
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
//#endregion

//#region model: Nova Newton
pub fn generate_nova_newton_fractal_locally(
  // &self,
  resolution: &Resolution,
  range: &Range,
  polynomial: &dyn Fn(&ComplexOld) -> ComplexOld, // p(z)
  derivative: &dyn Fn(&ComplexOld) -> ComplexOld, // p'(z)
  max_iterations: i32,
  epsilon: f64,
  generated_image_name: &str,
) -> Result<(), ImageError> {
  let (width, height) = (resolution.nx, resolution.ny);
  let mut image = ImageBuffer::new(width as u32, height as u32);

  let scale_x = (range.max.x - range.min.x) / width as f64;
  let scale_y = (range.max.y - range.min.y) / height as f64;

  for x in 0..width {
    for y in 0..height {
      let cx = range.min.x + x as f64 * scale_x;
      let cy = range.min.y + y as f64 * scale_y;

      let mut z = ComplexOld::new(1.0, 0.0);
      let c = ComplexOld::new(cx, cy);

      let mut i = 0;
      while i < max_iterations {
        let next_z = z.subtract(&polynomial(&z).divide(derivative(&z))).add(&c);
        if (next_z.subtract(&z)).square_norm() < epsilon {
          break;
        }

        z = next_z;
        i += 1;
      }

      let pixel_intensity = PixelIntensity {
        zn: 0.0,
        count: i as f32 / max_iterations as f32,
      };

      let pixel = map_color_nova_newton_z3_locally(&pixel_intensity);
      image.put_pixel(x as u32, y as u32, pixel);
    }
  }

  let generated_image_path = format!("worker/generated/images/{}.png", &generated_image_name);
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
//#endregion

//#region model: Newton
pub fn generate_newton_fractal_locally(
  // &self,
  resolution: &Resolution,
  range: &Range,
  polynomial: &dyn Fn(&ComplexOld) -> ComplexOld,
  derivative: &dyn Fn(&ComplexOld) -> ComplexOld,
  max_iterations: i32,
  epsilon: f64,
  generated_image_name: &str,
) -> Result<(), ImageError> {
  let (width, height) = (resolution.nx, resolution.ny);
  let mut image = ImageBuffer::new(width as u32, height as u32);

  let scale_x = (range.max.x - range.min.x) / width as f64;
  let scale_y = (range.max.y - range.min.y) / height as f64;

  for x in 0..width {
    for y in 0..height {
      let cx = range.min.x + x as f64 * scale_x;
      let cy = range.min.y + y as f64 * scale_y;

      let mut z = ComplexOld::new(cx, cy);
      let mut i = 0;
      let mut pzn = polynomial(&z).square_norm();

      while i < max_iterations && pzn > epsilon {
        z = z.subtract(&polynomial(&z).divide(derivative(&z)));
        pzn = polynomial(&z).square_norm();
        i += 1;
      }

      let pixel_intensity = PixelIntensity {
        zn: (0.5 + z.argument() / (2.0 * std::f64::consts::PI)).fract() as f32,
        count: get_convergence_value(pzn as f32, epsilon, i, max_iterations),
      };

      let pixel = map_color_newton_locally(&pixel_intensity);
      image.put_pixel(x as u32, y as u32, pixel);
    }
  }

  let generated_image_path = format!("worker/generated/images/{}.png", generated_image_name);
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
