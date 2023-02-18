use std::{sync::{Arc, Mutex, MutexGuard}, net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, str};

use crate::{Router, Route, Templates, ServerInitOptions, HttpResponse};

use super::ThreadPool;
#[allow(dead_code)]

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
        let router = Arc::new(Mutex::new(Router::new()));
        

        Server {
            listener,
            router,
            options
        }
    }

    pub fn router(&self) -> MutexGuard<Router> {
        return self.router.lock().unwrap();
    }
    pub fn init(&mut self) {
        let mut router = self.router.lock().unwrap();
        
        let favicon = Route::new("/favicon.ico", || {
            let mut response = HttpResponse::new();
            response.content_type = "image/jpeg".to_owned();
            response.set_body(Templates::render_bytes("favicon.ico").unwrap());
        
            return response;
        });
        router.register(favicon);
    }

    pub fn start(&mut self) {
        let threadpool = Arc::new(ThreadPool::new(10));
        println!("Server started on http://{}:{}", self.options.ip, self.options.port);
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
        let route: &mut Route = router.find_route(path).unwrap();        

        let response = route.render();
        let body = response.body.unwrap().bytes;
        let header = format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n", &body.len());
        stream.write_all(header.as_bytes()).unwrap();
        stream.write_all(body.as_slice()).unwrap();
    }
}