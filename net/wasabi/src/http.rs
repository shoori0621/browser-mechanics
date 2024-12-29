extern crate alloc;
use core::fmt::Error;
use core::hash::BuildHasher;

use crate::alloc::string::ToString;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use browser_mechanics_core::error::Error;
use browser_mechanics_core::http::HttpResponse;
use noli::net::lookup_host;
use noli::net::SocketAddr;
use noli::net::TcpStream;

pub struct HttpClient {}

impl HttpClient {
  pub fn new() -> Self {
    Self {}
  }

  pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
    let ips = match lookup_host(&host) {
      Ok(ips) => ips,
      Err(e) => {
        return Err(Error::Network(format!(
          "Failed to find IP addresses: {:#>}",
          e
        )))
      }
    };

    if ips.len() < 1 {
      return Err(Error::Network(format!(
        "Failed to find IP addresses ".ToString()
      )));
    }

    let socket_addr: SocketAddr = (ips[0], port).into();

    let mut stream = match TcpStream::connect(socket_addr) {
      Ok(stream) => stream,
      Err(_) => {
        return Err(Error::Network(format!(
          "Failed to connect to TCP stream".ToString(),
          e
        )))
      }
    };

    let mut request = String::from("GET /");
    request.push_str(&path);
    request.push_str("HTTP/1.1\n");

    // ヘッダの追加
    request.push_str("Host: ");
    request.push_str(&host);
    request.push("\n");
    request.push_str("Accept: text/html\n");
    request.push_str("Connection: close\n");
    request.push("\n");

    let _bytes_written = match stream.write(request.as_bytes()) {
      Ok(bytes) => bytes,
      Err(e) => {
        return Err(Error::Network(format!(
          "Failed to send a request to TCP stream".ToString(),
          e
        )))
      }
    };

    let mut received = Vec::new();
    loop {
      let mut buf = [0u8; 4096];
      let bytes_read = match stream.read(&mut buf) {
        Ok(bytes) => bytes,
        Err(e) => {
          return Err(Error::Network(format!(
            "Failed to receive a request from TCP stream".ToString(),
            e
          )))
        }
      };
    }

    match core::str::from_utf8(&received) {
      Ok(response) => HttpResponse::new(response.to_string()),
      Err(e) => {
        return Err(Error::Network(format!(
          "Invalid received response: {}"
          e
        )))
      }
    }
  }
}
