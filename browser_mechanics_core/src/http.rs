use crate::error::Error;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct HttpResponse {
  version: String,
  status_code: u32,
  reason: String,
  headers: Vec<Header>,
  body: String,
}

impl HttpResponse {
  pub fn new(raw_response: String) -> Result<Self, Error> {
    let preproceed_response = raw_response.trim_start().replace("\r\n", "\n");

    let (status_line, remaining) = match preproceed_response.split_once("\n") {
      Some((s, r)) => (s, r),
      None => {
        return Err(Error::Network(format!(
          "invalid http response: {}",
          preproceed_response
        )))
      }
    };

    let (headers, body) = match remaining.split_once("\n\n") {
      Some((h, b)) => {
        let mut headers = Vec::new();
        for header in h.split('\n') {
          let splitted_header: Vec<&str> = header.splitn(2, ':').collect();
          headers.push(Header::new(
            String::from(splitted_header[0].trim()),
            String::from(splitted_header[1].trim()),
          ));
        }
        (headers, b)
      }
      None => (Vec::new(), remaining),
    };

    let statuses: Vec<&str> = status_line.split(' ').collect();

    Ok(Self {
      version: statuses[0].to_string(),
      status_code: statuses[1].parse().unwrap_or(404),
      reason: statuses[2].to_string(),
      headers,
      body: body.to_string(),
    })
  }

  pub fn version(&self) -> String {
    self.version.clone()
  }

  pub fn status_code(&self) -> u32 {
    self.status_code.clone()
  }

  pub fn reason(&self) -> String {
    self.reason.clone()
  }

  pub fn headers(&self) -> Vec<Header> {
    self.headers.clone()
  }

  pub fn body(&self) -> String {
    self.body.clone()
  }

  pub fn header_value(&self, name: &str) -> Result<String, String> {
    for h in &self.headers {
      if h.name == name {
        return Ok(h.value.clone());
      }
    }

    Err(format!("failed to find {} in headers", name))
  }
}

#[derive(Debug, Clone)]
pub struct Header {
  name: String,
  value: String,
}

impl Header {
  pub fn new(name: String, value: String) -> Self {
    Self { name, value }
  }
}
