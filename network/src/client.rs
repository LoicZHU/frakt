use crate::{FragmentRequest, FragmentResult, FragmentTask, FragmentTaskId, FragmentTaskMessage};
use fractal::generator::PixelIntensity;
use std::io;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Client {
  server_ip_address: String,
  connection_name: String,
  port: u16,
  stream: Option<TcpStream>,
}

const DEFAULT_PORT: u16 = 8787;

impl Client {
  pub fn new(server_ip_address: String, connection_name: String, maybe_port: Option<u16>) -> Self {
    Client {
      server_ip_address,
      connection_name,
      port: if let Some(port) = maybe_port {
        port
      } else {
        DEFAULT_PORT
      },
      stream: None,
    }
  }

  pub fn connect_to_server(&mut self) -> io::Result<()> {
    let stream = TcpStream::connect((self.server_ip_address.as_ref(), self.port));
    match stream {
      Ok(stream) => {
        self.stream = Some(stream);
        println!("Connected to server at address {}", self.server_ip_address)
      }
      Err(_) => eprintln!("Could not connect to server"),
    }

    Ok(())
  }

  pub fn send_fragment_request(
    &mut self,
    worker_name: String,
    max_workload: u32,
  ) -> Result<(), io::Error> {
    let fragment_request = FragmentRequest::builder()
      .with_worker_name(worker_name)
      .with_max_work_load(max_workload)
      .build()
      .unwrap();

    match &mut self.stream {
      None => Err(io::Error::new(
        ErrorKind::NotConnected,
        "Cannot send a Fragment Request if the connection is not established",
      )),
      Some(stream) => {
        if let Ok(json_formatted_request) = fragment_request.to_json() {
          let json_request_size = json_formatted_request.len() as u32;
          let total_request_size = json_request_size;

          stream.write(&total_request_size.to_be_bytes())?;
          stream.write(&total_request_size.to_be_bytes())?;
          stream.write_all(json_formatted_request.as_str().as_bytes())?;
        } else {
          return Err(io::Error::new(
            ErrorKind::InvalidData,
            "Could not serialize fragment request to JSON",
          ));
        }

        Ok(())
      }
    }
  }

  pub fn read_fragment_task(&mut self) -> Result<(FragmentTask, FragmentTaskId), io::Error> {
    let total_message_size: u32 = self.read_total_message_size()?;
    let json_message_size: u32 = self.read_json_message_size()?;

    let fragment_task = self.read_fragment_task_json_message(json_message_size as usize)?;

    let fragment_task_id_size = total_message_size - json_message_size;
    let fragment_task_id = self.read_fragment_task_id(fragment_task_id_size as usize)?;

    Ok((fragment_task, fragment_task_id))
  }

  pub fn send_fragment_result(
    &mut self,
    fragment_task: FragmentTask,
    fragment_task_id: FragmentTaskId,
    pixels: Vec<PixelIntensity>,
  ) -> Result<(), io::Error> {
    let pixels_count: u32 =
      fragment_task.resolution.width as u32 * fragment_task.resolution.height as u32;
    let pixels_bytes_count: u32 = pixels_count * (std::mem::size_of::<u32>() as u32 * 2);
    let fragment_task_id_size = fragment_task_id.len() as u32;

    let fragment_result = FragmentResult::builder()
      .with_id(0, fragment_task_id_size)
      .with_resolution(
        fragment_task.resolution.width,
        fragment_task.resolution.height,
      )
      .with_range(
        fragment_task.range.min.x,
        fragment_task.range.min.y,
        fragment_task.range.max.x,
        fragment_task.range.max.y,
      )
      .with_pixels(fragment_task_id_size, pixels_count)
      .build()
      .map_err(|_| {
        io::Error::new(
          ErrorKind::InvalidInput,
          "Could not build the fragment result before sending it",
        )
      })?;

    if let Ok(json_formatted_fragment_result) = fragment_result.to_json() {
      let json_message_bytes_size = json_formatted_fragment_result.len() as u32;
      let total_message_bytes_size =
        json_message_bytes_size + fragment_task_id_size + pixels_bytes_count;
      println!(
        "Total message size = {} and json message size = {}",
        &total_message_bytes_size, &json_message_bytes_size
      );
      self.send_message_sizes(total_message_bytes_size, json_message_bytes_size)?;
      println!("fragment result = {:?}", json_formatted_fragment_result);
      self.send_fragment_result_json_message(json_formatted_fragment_result)?;
      println!("fragment result id = {:?}", fragment_task_id);
      self.send_fragment_result_id(fragment_task_id)?;
      self.send_pixels(pixels)?;
      println!("PIXELS SENT");
    } else {
      return Err(io::Error::new(
        ErrorKind::InvalidInput,
        "Could not serialize fragment result to json",
      ));
    }

    Ok(())
  }

  fn read_total_message_size(&mut self) -> Result<u32, io::Error> {
    let mut total_size_buffer = [0; std::mem::size_of::<u32>()];

    match &mut self.stream {
      None => Err(io::Error::new(
        ErrorKind::NotConnected,
        "Cannot read the total message size if the connection is not established",
      )),
      Some(stream) => {
        stream.read_exact(&mut total_size_buffer)?;

        Ok(u32::from_be_bytes(total_size_buffer))
      }
    }
  }

  fn read_json_message_size(&mut self) -> Result<u32, io::Error> {
    let mut json_message_size_buffer = [0; std::mem::size_of::<u32>()];

    match &mut self.stream {
      None => Err(io::Error::new(
        ErrorKind::NotConnected,
        "Cannot read the json message size if the connection is not established",
      )),
      Some(stream) => {
        stream.read_exact(&mut json_message_size_buffer)?;

        Ok(u32::from_be_bytes(json_message_size_buffer))
      }
    }
  }

  fn read_fragment_task_json_message(
    &mut self,
    json_message_size: usize,
  ) -> Result<FragmentTask, io::Error> {
    let mut json_message_buffer = vec![0; json_message_size];

    match &mut self.stream {
      None => Err(io::Error::new(
        ErrorKind::NotConnected,
        "Cannot read the json fragment task message if the connection is not established",
      )),
      Some(stream) => {
        stream.read_exact(&mut json_message_buffer)?;

        let fragment_task_message: FragmentTaskMessage =
          serde_json::from_str(&String::from_utf8_lossy(&json_message_buffer))?;

        Ok(fragment_task_message.fragment_task)
      }
    }
  }

  fn read_fragment_task_id(&mut self, id_size: usize) -> Result<FragmentTaskId, io::Error> {
    let mut id_buffer: FragmentTaskId = vec![0; id_size];

    match &mut self.stream {
      None => Err(io::Error::new(
        ErrorKind::NotConnected,
        "Cannot read the json fragment task message if the connection is not established",
      )),
      Some(stream) => {
        stream.read_exact(&mut id_buffer)?;

        Ok(id_buffer)
      }
    }
  }

  fn send_message_sizes(
    &mut self,
    total_message_size: u32,
    json_message_size: u32,
  ) -> Result<(), io::Error> {
    match &mut self.stream {
      None => Err(io::Error::new(
        ErrorKind::NotConnected,
        "Cannot send the fragment result json message sizes if the connection is not established",
      )),
      Some(stream) => {
        stream.write(&total_message_size.to_be_bytes())?;
        stream.write(&json_message_size.to_be_bytes())?;

        Ok(())
      }
    }
  }

  fn send_fragment_result_json_message(
    &mut self,
    json_formatted_fragment_result: String,
  ) -> Result<(), io::Error> {
    match &mut self.stream {
      None => Err(io::Error::new(
        ErrorKind::NotConnected,
        "Cannot send the fragment result json message if the connection is not established",
      )),
      Some(stream) => {
        stream.write(json_formatted_fragment_result.as_str().as_bytes())?;

        Ok(())
      }
    }
  }

  fn send_fragment_result_id(&mut self, fragment_task_id: FragmentTaskId) -> Result<(), io::Error> {
    match &mut self.stream {
      None => Err(io::Error::new(
        ErrorKind::NotConnected,
        "Cannot send the fragment result message id if the connection is not established",
      )),
      Some(stream) => {
        stream.write(&fragment_task_id)?;

        Ok(())
      }
    }
  }

  fn send_pixels(&mut self, pixels: Vec<PixelIntensity>) -> Result<(), io::Error> {
    match &mut self.stream {
      None => Err(io::Error::new(
        ErrorKind::NotConnected,
        "Cannot send pixels if the connection is not established",
      )),
      Some(stream) => {
        for pixel in pixels {
          stream.write(&pixel.zn.to_be_bytes())?;
          stream.write(&pixel.count.to_be_bytes())?;
        }

        Ok(())
      }
    }
  }
}
