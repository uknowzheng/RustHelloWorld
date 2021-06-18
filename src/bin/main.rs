use hello::ThreadPool;
use std::io::prelude::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{fs, thread};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let pool = ThreadPool::new(4);
  for stream in listener.incoming().take(2) {
    let _stream = stream.unwrap();
    pool.execute(|| {
      handel_connection(_stream);
    });
  }

  println!("Shutting down.");
}

fn handel_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();
  println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
  let get = b"GET / HTTP/1.1\r\n";
  let (status_line, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK", "index.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND", "404.html")
  };
  println!("{}", filename);
  let contents = fs::read_to_string(filename).unwrap();
  let response = format!("{}\r\n\r\n{}", status_line, contents);
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
