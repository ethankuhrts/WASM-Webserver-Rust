use core::fmt;
use std::{collections::HashMap, hash::Hash};

use super::{HttpBody, URI};

#[derive(Debug)]
pub enum HttpMethod {
    GET, POST, PUT, UPDATE, DELETE
}

impl HttpMethod {
    pub fn to_string(self) -> String {
        match self {
            HttpMethod::POST => String::from("POST"),
            HttpMethod::GET => String::from("GET"),
            HttpMethod::PUT => String::from("PUT"),
            HttpMethod::UPDATE => String::from("UPDATE"),
            HttpMethod::DELETE => String::from("DELETE"),
            
        }
    }
    pub fn from_str(str: &str) -> Option<HttpMethod> {
        match str {
            "GET" => Some(HttpMethod::GET),
            "POST" => Some(HttpMethod::POST),
            "UPDATE" => Some(HttpMethod::UPDATE),
            "PUT" => Some(HttpMethod::PUT),
            "DELETE" => Some(HttpMethod::DELETE),
            _ => None,
        }
    }
}
#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub uri: String,
    pub version: f32,
    pub headers: HashMap<String, String>,
    pub host: String,
    pub connection: String,
    pub body: Vec<u8>,
}

impl HttpRequest {
    /// Function to take a Http request from a string and convert it into a request struct.
    /// Input must be a valid HTTP request and NOT an Http response.
    /// Will return an Error::InvalidRequest if str is not valid HTTP Request.
    /// Example HTTP Request
    /// * GET / HTTP/1.1
    /// * Host: 127.0.0.1:9500
    /// * Connection: keep-alive
    /// * Cache-Control: max-age=0
    /// * sec-ch-ua: "Chromium";v="110", "Not A(Brand";v="24", "Microsoft Edge";v="110"
    /// * sec-ch-ua-mobile: ?0
    /// * sec-ch-ua-platform: "Windows"
    /// * Upgrade-Insecure-Requests: 1
    /// * User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36 Edg/110.0.1587.50
    /// * Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
    /// * Sec-Fetch-Site: none
    /// * Sec-Fetch-Mode: navigate
    /// * Sec-Fetch-User: ?1
    /// * Sec-Fetch-Dest: document
    /// * Accept-Encoding: gzip, deflate, br
    /// * Accept-Language: en-US,en;q=0.9
    pub fn from_string(request: &str) -> Self {
        // String path to the resource not including website address e.g. https://website.com/blog/17814 has a path of /blog/17814
        let mut path: Option<String> = None;
        // HTTP Method e.g. GET, POST ... 
        let mut method: Option<HttpMethod> = None;
        // HTTP Version number e.g. 1.1
        let mut version: Option<f32> = None;
        // list of headers for the request, all headers are put into a hashmap with key and value as <String>'s
        let mut headers: HashMap<String, String> = HashMap::new();
        // host: the main part of the url, website address e.g. 127.0.0.1::8000 OR website.com
        let mut host: Option<String> = None;
        // connection: LEARN MORE ABOUT THIS
        let mut connection: Option<String> = None;
        // content of the request
        let mut body: Vec<u8> = vec![];
        
        // current index of the parser, used for taking slices of the request near the current char being proccesed 

        // current section of char's being considered, will be cleared after a str has been confirmed and will either be disregarded or added to a variable above.
        let mut buffer_str: Vec<&str> = vec![""];
        // a boolean representing whether the characters we are recording are important and should be kept or not
        let mut buffer_started: bool = false;
        let lines: Vec<&str> = request.split("\r\n").collect();
        let mut line_index = 0;
        for line in &lines {
            if line_index == 0 {
                let line: &str = &format!("{}%", line);
                // first line, contains Method, Path and HTTP Version
                let mut rec_buf: Vec<&str> = Vec::new();
                for char in line.split("") {
                
                    rec_buf.push(char);
                    if char == " " { // segment ended
                        if method.is_none() {
                            method = Some(HttpMethod::from_str(&rec_buf[0..rec_buf.len() - 1].join("")).unwrap());
                            rec_buf.clear();
                        } else if path.is_none() {
                            path = Some(rec_buf[0..rec_buf.len() - 1].join(""));
                        }
                    }
                    if char == "/" && method.is_some() && path.is_some() {
                        rec_buf.clear();
                    }
                    if char == "%" {
                        version = Some(rec_buf[0..rec_buf.len()-1].join("").parse::<f32>().unwrap());
                    }
                }
            } else {
                if line.is_empty() {
                    continue;
                }
                if line_index == lines.len() - 1 {
                    body = line.as_bytes().into();
                    continue;
                }
                let line_split: Vec<&str> = line.split(": ").collect();
                if line_split.len() != 2 {
                    println!("{:?}", line_split);
                }
                let (key, value) = (line_split[0], line_split[1]);
                match key {
                    "Host" => {host = Some(value.to_owned())},
                    "Connection" => {connection = Some(value.to_owned())},
                    _ => {headers.insert(key.to_owned(), value.to_owned());}
                }
                
            }
            line_index += 1;
        }
        HttpRequest {
            method: method.unwrap(),
            uri: path.unwrap(),
            version: version.unwrap(),
            host: host.unwrap(),
            connection: connection.unwrap(),
            headers: headers,
            body: body,

        }
    }
}