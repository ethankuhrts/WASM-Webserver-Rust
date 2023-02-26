
#[derive(Debug)]
pub struct URI {
    url: String,
    path: String,
    segments: Vec<String>,
}