#[derive(Debug, Clone)]
pub struct Error(ErrorKind, String);

impl Error {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self(kind, message.into())
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    pub fn message(&self) -> &str {
        &self.1
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        format!("{}: {}", self.0.as_str(), self.1)
    }
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Dummy,
}

impl ErrorKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Dummy => "Dummy",
        }
    }
}

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
