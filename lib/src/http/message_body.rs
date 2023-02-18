use std::str;

pub trait MessageBody: Sized {
    fn to_bytes(self) -> Vec<u8> { vec![] }
}
impl MessageBody for String {
    fn to_bytes(self) -> Vec<u8> {
        return self.as_bytes().to_vec();
    }
}
impl MessageBody for &str {
    fn to_bytes(self) -> Vec<u8> {
        return self.as_bytes().to_vec();
    }
}
impl MessageBody for Vec<u8> {
    fn to_bytes(self) -> Vec<u8> {
        return self;
    }
}
impl MessageBody for &[u8] {
    fn to_bytes(self) -> Vec<u8> {
        return self.to_vec();
    }
}