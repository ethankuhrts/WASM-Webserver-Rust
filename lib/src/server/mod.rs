mod options;
mod threadpool;

use std::{sync::{Arc, Mutex}, net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, cell::RefCell};

pub use options::ServerInitOptions as ServerInitOptions;
pub use threadpool::ThreadPool as ThreadPool;



use crate::{Router, Route};


pub struct Server {
    listener: Arc<TcpListener>,
    pub router: Arc<Mutex<Router>>,
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
        let router = Arc::new(Mutex::new(Router::new()));
        

        Server {
            listener,
            router,
            options
        }
    }

    pub fn init(&mut self) {
        let mut router = self.router.lock().unwrap();
        let favicon = Route::new("/favicon.ico", || {
            return "no".to_string();
        });
        router.register(favicon);
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
        let http_request = http_request.join("");
        
        let path = &http_request[4..http_request.find(" HTTP/1.1").unwrap_or(http_request.len())];
        
        println!("{}", &path);

        let mut router = router.lock().unwrap();
        let mut route: &mut Route = router.find_route(path).unwrap();
        println!("{:?}", route.path);

        println!("Request: {:#?}", http_request);
        

        let status = "HTTP/1.1 200 OK";
        let contents = route.render();
        let length = contents.len();
        let response = format!(
            "{status}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        stream.write_all(response.as_bytes());
    }
}