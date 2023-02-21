# WASM-Webserver
!! IN EARLY DEVELOPMENT, NOT IN A PRODUCTION STATE !!

This is a simple webserver intended to run WASM binaries.

## Documentation

[Documentation](https://linktodocumentation)

 Basic Server implementation
 
 Cargo:
 ```
 [dependencies]
 webserver = { path="../lib" }
 ```
 main.rs
```rust
use webserver::{Server, ServerInitOptions, Route};

fn main() {
    /// Create a Server Instance using ServerInitOptions
    /// Currently only options are IP and PORT
    /// Must define as mutable
    let mut server = Server::new(ServerInitOptions {
        IP: String::from("127.0.0.1"),
        PORT: 9500, // Port can be any 16 bit int
    });

    /// Initialize backend server routes
    /// e.g. /favicon.ico
    server.init();

    /// Here we define the index route
    /// Route::new(
    ///     [route path e.g. "/" or "/contact"], 
    ///     [callback a function returning a String]
    /// )
    /// callback must return a String
    let index = Route::new("/", || -> String {
        return "Put your HTML here".to_string();
    });

    /// Get a mutable reference to the Router
    /// The router is behind a Mutex so we must lock it to access it
    let mut router = server.router.lock().unwrap();
    /// Register the index route we made
    router.register(index);

    /// once all routes are registered, drop the router to free up use of it to run the server
    drop(router);

    /// Finally start the server
    server.start()
}
```

## Currently in development
- Adding Http struct's for Response, Request and other parsing of Http data.
- Adding variable route paths
- Adding query variables 

## Would like suggestions or help on
- Security and ddos attack protection
- input validation 
