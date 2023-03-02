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

    pub fn find_request_route(&mut self, request: &mut HttpRequest) -> Result<&mut Route, Error>  {
        match self.routes.iter_mut().find(|route| {
            let path: Vec<&str> = request.path.split("/").filter(|x| {!x.is_empty()}).collect();
            if route.path.len() != path.len() { return false }
            for i in 0..route.path.len() {
                let p = &route.path[i];
                let pr = &path[i];
                if p.dynamic {
                    request.path_var.insert(p.name.to_owned(), pr.to_string());
                    continue;
                }
                if &p.name != pr {
                    return false;
                }
            }
            println!("{:?}", request);
            return true;
        }) {
            Some(res) => Ok(res),
            None => Err(Error::NotFound),
        }
    }

    pub fn find_route(&mut self, path: &str) -> Result<&mut Route, Error> {
        let path: Vec<&str> = path.split("/").filter(|c| { !c.is_empty() }).collect();
        match self.routes.iter_mut().find(|route| {
            for i in 0..route.path.len() {
                let p = &route.path[i];
                if !p.dynamic || &p.name != &path[i] {
                    return false
                }
            }
            return true;
        }) {
            Some(res) => { Ok(res)},
            None => {println!("Route Not found, {}", path[0]); Err(Error::NotFound)}
        }
    }   
}

#[derive(Debug, Clone)]
pub struct UrlPath {
    pub name: String,
    pub dynamic: bool,    
}

/// Route Struct, defines a url path and a function to call when the page is loaded
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
    pub path: Vec<UrlPath>,
    pub callback: Box<dyn FnMut(HttpRequest) -> HttpResponse + Sync + Send + 'static>,
}



impl Route {
    pub fn new<F>(path: &str, callback: F)  -> Self 
    where F: FnMut(HttpRequest) -> HttpResponse + Sync + Send + 'static {
        println!("{:?}", path);
        let ps : Vec<&str> = path.split("/").filter(|x| {!x.is_empty()}).collect(); // path split by '/'
    
        let mut paths: Vec<UrlPath> = Vec::new();
        for path in ps {   
            let dynamic: bool;
            let name: &str;
            
            if &path[0..1] == "<" && &path[path.len()-1..path.len()]== ">" {
                name = &path[1..path.len()-1];
                dynamic = true;
            } else {
                name = path;
                dynamic = false;
            }
            paths.push(UrlPath { name: name.to_owned(), dynamic: dynamic })
        }

        
        Route {
            path: paths,
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