use std::fmt::{Debug};

pub enum Error {
    FileNotFound,
    PageNotFound,
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotFound => write!(f, "FileNotFound"),
            Self::PageNotFound => write!(f, "PageNotFound"),
        }
    }
}