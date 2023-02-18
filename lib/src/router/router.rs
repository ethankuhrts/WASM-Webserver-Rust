use std::{net::TcpStream, sync::Arc, fmt};

use crate::http::HttpResponse;


pub enum Error {
    NotFound
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotFound => write!(f, "Route not found")
        }
    }
}

pub struct Router {
    pub routes: Vec<Route>,
    
}
unsafe impl Send for Router {}
unsafe impl Sync for Router {}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: Vec::new(),
        }
    }

    pub fn handle_connection(&mut self, stream: TcpStream) {

    }
    pub fn register(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn find_route(&mut self, path: &str) -> Result<&mut Route, Error> {
        match self.routes.iter_mut().find(|route| {route.path == path}) {
            Some(res) => { Ok(res)},
            None => {println!("Route Not found, {}", path); Err(Error::NotFound)}
        }
    }   
}



pub struct Route {
    pub path: String,
    pub callback: Box<dyn FnMut() -> HttpResponse + Sync + Send + 'static>,
}

impl Route {
    pub fn new<F>(path: &str, callback: F)  -> Self 
    where F: FnMut() -> HttpResponse + Sync + Send + 'static {
        
        Route {
            path: String::from(path),
            callback: Box::new(callback),
        }
    }
    pub fn render(&mut self) -> HttpResponse {
        (self.callback)()
    }
}

// impl Clone for Route {
//     fn clone(&self) -> Self {
//         Route {
//             path: self.path.clone(),
//             callback: self.callback,
//         }
//     }
// }