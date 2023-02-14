mod options;
mod threadpool;

use std::{sync::{Arc, Mutex}, net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}};

pub use options::ServerInitOptions as ServerInitOptions;
pub use threadpool::ThreadPool as ThreadPool;



use crate::Router;


pub struct Server {
    listener: Arc<TcpListener>,
    router: Arc<Mutex<Router>>,
    options: Arc<ServerInitOptions>
}

impl Server {
    pub fn new(options: ServerInitOptions) -> Self {
        let options = Arc::new(options);
        let listener = Arc::new(
            match TcpListener::bind(format!("{}:{}", options.ip, options.port)) {
                Ok(res) => res,
                Err(err) => { panic!("{:?}, options: {:?}", err, options) }
            }
        );
        let router = Arc::new(Mutex::new(Router {}));
        

        Server {
            listener,
            router,
            options
        }
    }

    pub fn start(&mut self) {
        let threadpool = Arc::new(ThreadPool::new(10));
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let router = self.router.clone();
            threadpool.execute(|| {
                Server::handle_connection(router, stream);
            });
        }
    }

    fn handle_connection(router: Arc<Mutex<Router>>, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);   
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {:#?}", http_request);
        

        let status = "HTTP/1.1 200 OK";
        let contents = "haha nerd";
        let length = contents.len();
        let response = format!(
            "{status}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        stream.write_all(response.as_bytes());
    }
}