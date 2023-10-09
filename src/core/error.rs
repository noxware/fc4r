use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// The loaded configuration is invalid
    InvalidConfig,
    /// Configurations are missing
    MissingConfig,
    /// An unexpected/unknown error occurred
    Unexpected,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub main_msg: String,
    pub detail_msg: Option<String>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.main_msg)?;

        if let Some(detail_msg) = &self.detail_msg {
            write!(f, ". {}", detail_msg)?;
        }

        Ok(())
    }
}

impl Error {
    pub fn invalid_config(detail_msg: String) -> Self {
        Error {
            kind: ErrorKind::InvalidConfig,
            main_msg: "the loaded configuration is invalid ".to_string(),
            detail_msg: Some(detail_msg),
        }
    }

    pub fn missing_config(detail_msg: String) -> Self {
        Error {
            kind: ErrorKind::MissingConfig,
            main_msg: "configurations are missing".to_string(),
            detail_msg: Some(detail_msg),
        }
    }

    pub fn unexpected(detail_msg: String) -> Self {
        Error {
            kind: ErrorKind::Unexpected,
            main_msg: "an unexpected error occurred".to_string(),
            detail_msg: Some(detail_msg),
        }
    }
}
