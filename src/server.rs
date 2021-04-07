use std::net::TcpListener;

pub struct Server {
  addr: String,
}

// A reference to an array of arbitrary size, i.e. &[u8] is a pointer
fn arr(a: &[u8]) {}

impl Server {
  pub fn new(addr: String) -> Self {
    Self { addr }
  }

  pub fn run(self) {
    println!("Listening on {}", self.addr);

    let listener = TcpListener::bind(&self.addr).unwrap();

    match listener.accept() {
      Ok((mut stream, _)) => {
        let a = [1, 3, 4, 5];
        arr(&a); //This is called a slice
      }
      Err(e) => println!("Failed to establish a connection: {}", e),
    }
  }
}
