use std::{collections::HashMap, ops::Range};
use serde::{Deserialize, Serialize};


use webserver::{Server, ServerInitOptions, Route, macros::get, Templates};
use webserver::http::{HttpResponse, HttpBody, HttpRequest};

#[derive(Serialize, Deserialize)]
pub struct Message {
    sender: usize,
    content: String,
}
impl HttpBody for Message {
    fn to_bytes(self) -> Vec<u8> { 
        let text = serde_json::to_string(&self).unwrap();
        return text.to_bytes();
    }
}

fn main() {
    // Create a Server Instance using ServerInitOptions
    // Currently only options are IP and PORT
    // Must define as mutable
    let mut server = Server::new(ServerInitOptions {
        ip: String::from("127.0.0.1"),
        port: 9500, // Port can be any 16 bit int
        public_directory: String::from("./res"),
    });

    // Initialize backend server routes
    // e.g. /favicon.ico
    server.init();

    // Here we define the index route
    // Route::new(
    //     [route path e.g. "/" or "/contact"], 
    //     [callback a function returning a String]
    // )
    // callback must return a String

    // Get a mutable reference to the Router
    // The router is behind a Mutex so we must lock it to access it
    let mut router = server.router();
    
    let index = Route::new("/", |request: HttpRequest| {
        HttpResponse {
            content_type: String::from("text/html"),
            body: Templates::render("index.html").unwrap(),
            ..Default::default()
        }
    });

    // createa a dynamic route to add 2 numbers together
    let dynamic_route = Route::new("/sum/<num1>/<num2>", |request: HttpRequest| {
        // the route path variables always come in String format, you must parse them manually
        let num1 = request.path_var["num1"].parse::<f32>().unwrap_or(0.0);
        let num2 = request.path_var["num2"].parse::<f32>().unwrap_or(0.0);
        let sum = num1+num2;
        HttpResponse {
            content_type: String::from("text/html"),
            body: String::from(format!("{num1} + {num2} = {sum}")),
            ..Default::default()
        }
    });
    
    router.register(dynamic_route);
    

    // Register the index route we made
    router.register(index);

    // once all routes are registered, drop the router to free up use of it to run the server
    drop(router);

    // Finally start the server
    server.start()
}