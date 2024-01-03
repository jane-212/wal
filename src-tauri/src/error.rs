#[derive(Debug)]
pub enum IError {
    Network(String),
    Selector(String),
}

impl From<reqwest::Error> for IError {
    fn from(value: reqwest::Error) -> Self {
        Self::Network(value.to_string())
    }
}

impl From<scraper::error::SelectorErrorKind<'_>> for IError {
    fn from(value: scraper::error::SelectorErrorKind) -> Self {
        Self::Selector(value.to_string())
    }
}

impl ToString for IError {
    fn to_string(&self) -> String {
        match self {
            Self::Network(err) => err.to_string(),
            Self::Selector(err) => err.to_string(),
        }
    }
}

pub type IResult<T> = std::result::Result<T, IError>;
