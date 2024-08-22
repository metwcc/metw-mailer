use std::{
    error::Error as StdError,
    fmt::{self, Display, Formatter},
};

use lettre::address::AddressError;

use crate::mailfmt::error::MailFmtError;


#[derive(Debug)]
pub enum MailerError {
    MailFmtError(MailFmtError),
    AddressError(AddressError),
    Error(Box<dyn std::error::Error>),
}

impl Display for MailerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::MailFmtError(mailfmt_error) => Display::fmt(&mailfmt_error, f),
            Self::AddressError(address_error) => Display::fmt(&address_error, f),
            Self::Error(error) => Display::fmt(&error, f),
        }
    }
}

impl From<MailFmtError> for MailerError {
    fn from(mailfmt_error: MailFmtError) -> Self {
        Self::MailFmtError(mailfmt_error)
    }
}

impl From<AddressError> for MailerError {
    fn from(address_error: AddressError) -> Self {
        Self::AddressError(address_error)
    }
}

impl StdError for MailerError {}
