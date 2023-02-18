

#[derive(Debug, Clone)]
pub struct ServerInitOptions {
    pub ip: String,
    pub port: u16,
    pub public_directory: String,
}