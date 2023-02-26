use std::str;

pub trait HttpBody: Sized {
    fn to_bytes(self) -> Vec<u8> { vec![] }
}
impl HttpBody for String {
    fn to_bytes(self) -> Vec<u8> {
        return self.as_bytes().to_vec();
    }
}
impl HttpBody for &str {
    fn to_bytes(self) -> Vec<u8> {
        return self.as_bytes().to_vec();
    }
}
impl HttpBody for Vec<u8> {
    fn to_bytes(self) -> Vec<u8> {
        return self;
    }
}
impl HttpBody for &[u8] {
    fn to_bytes(self) -> Vec<u8> {
        return self.to_vec();
    }
}