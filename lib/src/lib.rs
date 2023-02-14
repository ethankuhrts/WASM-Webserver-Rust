use std::{sync::Arc, net::TcpListener};

pub mod server;
pub use server::{
    Server as Server,
    ServerInitOptions as ServerInitOptions,
};
pub mod router;
pub use router::{
    Router as Router,
    Route as Route,
};











#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
        assert_eq!(0, 4);
    }
}


