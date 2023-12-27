#[derive(Debug)]
pub enum IError {
    Network(String),
}

impl Into<String> for IError {
    fn into(self) -> String {
        match self {
            Self::Network(err) => err.to_string(),
        }
    }
}

pub type IResult<T> = std::result::Result<T, IError>;
