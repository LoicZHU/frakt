use server::Server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let server = Server::new("127.0.0.1".to_string(), "8080".to_string());
  server.run_server()?;
  Ok(())
}
