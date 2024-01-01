use server::Server;

fn main() {
  // creating and running the test server //
  let server: Server = Server::new("127.0.0.1".to_string(), "8999".to_string());
  server.run_server().unwrap();
}
