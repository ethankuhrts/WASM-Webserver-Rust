use std::{sync::{Arc, Mutex, MutexGuard}, net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, str, default};

use crate::{Router, Route, Templates, ServerInitOptions,
http::{HttpResponse, HttpRequest}, router};

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
        
        let favicon = Route::new("/favicon.ico", |request: HttpRequest| {
            let mut response = HttpResponse {
                content_type: String::from("image/jpeg"),
                body_bytes: Some(Templates::render_bytes("favicon.ico").unwrap()),
                ..Default::default()
            };
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
        println!("{:?}", buf_reader);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap() + "\r\n")
            .take_while(|x| x != "\r\n")
            .collect();
        let request = HttpRequest::from_string(&http_request.join(""));
        println!("{}", request.uri);
        

        let mut router = router.lock().unwrap();

        // try to find the requested route.
        // if not found try to find the "404" route.
        // if that isnt found return none, none will be handled later when response is generated
        let route: Option<&mut Route> = match router.find_route(&request.uri) {
            Ok(res) => Some(res),
            Err(err) => {
                match router.find_route("404") {
                    Ok(res) => Some(res),
                    Err(err) => None
                }
            },
        };        
        
        // generate response based on route, if route is Some(&mut Route) then we can render it normally
        // if None then we need to build a generic 404 text response instead.
        let mut response: HttpResponse = match route {
            Some(r) => r.render(request),
            None => {
                HttpResponse {
                    status: String::from("404 Not Found"),
                    body: String::from("404 Page Not Found"),
                    ..Default::default()
                }
            }
        };
        
        // parse the body of the response 
        // if response.body is some then we can just accept that (means its utf-8 compatible)
        // if its none it likely means its non utf-8 and is already in a bytes form
        // if both are empty then there just isn't a body and we need to make an empty filler one
        // TODO! possibly move this to the http_response struct impl OR to a future response trait.
        response.pack();
        let mut body = match response.body_bytes {
            Some(res) => res,
            None => vec![],
        };
        let bytes = body.as_slice();

        let header = format!(
            "HTTP/{} {}\r\nContent-Length:{}\r\nContent-Type:{}\r\n\r\n", 
            response.version, response.status, bytes.len(), response.content_type
        );
        stream.write_all(header.as_bytes()).unwrap();
        stream.write_all(bytes);
        
    }
}