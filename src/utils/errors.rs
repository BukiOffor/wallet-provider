use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum KeyError{
    NotFound,
    PermissionDenied,
}

impl Display for KeyError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            KeyError::NotFound => write!(f, "File not found"),
            KeyError::PermissionDenied => write!(f, "Permission denied"),
        }
    }
}

impl std::error::Error for KeyError {}