use webserver::{Server, ServerInitOptions, Route};

fn main() {
    let mut server: Server = Server::new(ServerInitOptions { 
        ip: "127.0.0.1".to_string(), 
        port: 7900, 
    });

    let index = Route::new("/", || -> String {
        return "AA".to_string();
    });
    
    let mut router = server.router.lock().unwrap();
    router.register(index);

    drop(router);
    server.start();
}
