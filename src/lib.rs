//! ## metw-mailer
//! Standalone mailer client for metw.cc. A SMTP relay is required to send emails.
//! Can send emails in several languages. Uses custom email formatting syntax to format in several
//! languages.

#[cfg(test)]
mod test;

/// Send formatted emails with [mailer::Mailer].
pub mod mailer;
/// UNSTABLE: the [mailfmt::MailFmt] object is onsidered unstable. Do not use this function if you are not willing to have changes forced on you!
pub mod mailfmt;

/// You can create [Locales] from `&str`.
/// ```rust,ignore
/// let locale = Locales::from("en_US");
/// ```
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Locales {
    en_US,
    tr
}

impl Locales {
    fn from(locale_str: &str) -> Result<Self, String> {
        match locale_str {
            "en_US" => Ok(Locales::en_US),
            "tr" => Ok(Locales::tr),
            locale => Err(format!("unknown locale: {locale}"))
        }
    }
}
