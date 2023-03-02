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

## Dynamic Routes
Routes can be made with dynamic path variables E.g. "/blog/<post_id>/"
To create a dynamic route write your route path with variables wrapped in <> brackets
```rust
let dynamic_route = Route::new("/users/<username>/<post_id>" |request: HttpRequest| {
    // ... handle route processing ...
    return HttpResponse { ... }
});
```

To access the path variables use ```request.path_var["variable name"]```
```rust 
let dynamic_route = Route::new("/users/<username>/<post_id>" |request: HttpRequest| {
    let username = request.path_var["username"];
    let post_id = request.path_var["username"];
    return HttpResponse { ... }
});
```

## URL Parameters
Url parameters ("?var1=someval&var2=otherval") are parsed and stored in a HashMap<String, String>
and can be accessed through request.parameters["paramater_name"]

```rust
// example request url is "https://website.com/someroute?query=do%20a%20thing
let route_with_parameters = Route::new("/someroute" |request: HttpRequest| {
    let query = request.parameters["query"];
    return HttpResponse { ... }
});
```
NOTE: trying to access a parameter variable may panic or return nothing as there is no guarantee that the parameter exists

## Currently in development
- Adding Http struct's for Response, Request and other parsing of Http data.
- Adding variable route paths
- Adding query variables 

## Would like suggestions or help on
- Security and ddos attack protection
- input validation 
