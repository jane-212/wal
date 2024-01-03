use super::state::porn::Porn;
use crate::error::IResult;

mod porn;

pub struct AppState {
    porn: Porn,
}

impl AppState {
    pub fn new() -> IResult<Self> {
        Ok(Self { porn: Porn::new()? })
    }

    pub fn porn(&self) -> &Porn {
        &self.porn
    }
}
