use std::{
    error::Error as StdError,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum MailFmtError {
    UnknownLocale(crate::Locales),
    UnknownTemplate(String),
    FmtError
}

impl Display for MailFmtError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::UnknownLocale(locale) => f.write_str(&format!("unknown locale: {locale:?}")[..]),
            Self::UnknownTemplate(template) => f.write_str(&format!("unknown template: {template:?}")[..]),
            Self::FmtError => f.write_str("format error")
        }
    }
}

impl StdError for MailFmtError {}
