use std::fmt::{Debug, Display};

use super::HttpBody;

pub enum ContentType {
    TextHtml,
    
    ImageJpeg,
    ImagePng,
    ImageGif,
    ImageSVGXML,
    ImageXIcon,
    
    MultipartForm,
}
impl Debug for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TextHtml => write!(f, "text/html"),
            Self::ImageJpeg => write!(f, "image/jpeg"),
            Self::ImagePng => write!(f, "image/png"),
            Self::ImageGif => write!(f, "image/gif"),
            Self::ImageSVGXML => write!(f, "image/svg+xml"),
            Self::ImageXIcon => write!(f, "image/x-icon"),
            Self::MultipartForm => write!(f, "multipart/form-data"),
        }
    }
}

pub struct ResponseBody {
    pub bytes: Vec<u8>,
}

impl ResponseBody {
    pub fn from<B>(body: B) -> Self where B: HttpBody + 'static {
        ResponseBody { bytes: body.to_bytes() }
    }
}

impl Display for ResponseBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.bytes);
        Result::Ok(())
    }
}


pub struct HttpResponse {
    pub version: f32,
    pub status: String,
    pub content_type: String,
    pub body: String,
    pub body_bytes: Option<Vec<u8>>,
}

impl HttpResponse {
    pub fn pack(&mut self) {
        match self.body_bytes {
            Some(_) => {},
            None => {
                self.body_bytes = Some(self.body.clone().into_bytes());
            }
        }
    }
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self { 
            version: 1.1, 
            status: String::from("200 OK"), 
            content_type: String::from("text/html"), 
            body: String::from(""),
            body_bytes: None,
        }
    }

    
}



// impl Into::<String> for HttpResponse {
//     fn into(self) -> String {
//         let version = self.version;
//         let bytes_str = match self.body {
//             Some(res) => res.bytes.into_iter().map(|x| {x.to_string()}).collect(),
//             None => vec![String::from("")],
//         };
//         let contents = bytes_str.join(" ");
//         format!(
//             "HTTP/{} {}\r\nContent-Length: {}\r\n\r\n{}",
//             self.version, self.status,
//             contents.len(),
//             contents.as_bytes(),
//         )
//     }
// }

unsafe impl Send for HttpResponse {}
unsafe impl Sync for HttpResponse {}

