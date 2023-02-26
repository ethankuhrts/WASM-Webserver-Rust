use std::{net::TcpStream, sync::Arc, fmt, collections::HashMap};
use regex::Regex;

use crate::{http::{HttpResponse, HttpRequest}};


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


/// Route Struct, defines a url path and a function to call when the page is entered
/// create variable paths like:
/// 
/// "api/posts/<post_id>/"
/// 
/// or for multiple:
/// 
/// "api/posts/<user_id>/<post_id"
/// 
/// Will create a Route with a path of "/api/posts"
/// and a 
pub struct Route {
    pub path: String,
    pub callback: Box<dyn FnMut(HttpRequest) -> HttpResponse + Sync + Send + 'static>,
}



impl Route {
    pub fn new<F>(path: &str, callback: F)  -> Self 
    where F: FnMut(HttpRequest) -> HttpResponse + Sync + Send + 'static {
        
        Route {
            path: String::from(path),
            callback: Box::new(callback),
        }
    }
    
    pub fn render(&mut self, request: HttpRequest) -> HttpResponse {
        (self.callback)(request)
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