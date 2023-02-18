use std::{sync::Arc, net::TcpListener};

mod server;
pub use server::{
    Server as Server,
    ServerInitOptions as ServerInitOptions,
};
mod error;
pub use error::Error as Error;
mod router;
pub use router::{
    Router as Router,
    Route as Route,
};
mod templates;
pub use templates::{
    Templates as Templates
};
pub use macros;

pub mod http;
pub use http::HttpResponse as HttpResponse;











#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
        assert_eq!(0, 4);
    }
}


