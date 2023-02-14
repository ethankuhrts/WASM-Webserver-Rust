use webserver::*;

fn main() {
    let mut server: Server = Server::new(ServerInitOptions { 
        ip: "127.0.0.1".to_string(), 
        port: 7900, 
    });

    server.start();
}
