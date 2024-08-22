//! ## Mailer
//!
//! This module contains a wrapper for sending templated mails. [mailer::Mailer] implements a high-level API
//! for sending templated emails.
//!
//! ## Brief example
//!
//! This example shows how to send a templated email.
//!
//! ```rust,ignore
//! let mailer = Mailer::new("no-reply <no-reply@example.com>");
//! 
//! let mut vars = HashMap::new();
//! vars.insert(String::from("name"), String::from("Metw"));
//! vars.insert(String::from("verify"), String::from("https://example.com/site-api/verify-email/?key=000000"));
//! vars.insert(String::from("remove_email"), String::from("https://example.com/site-api/remove-email/?key=000000"));
//! 
//! let options = MailerOptions {
//!     mail_type: String::from("welcome"),
//!     locale: vec![Locales::en_US],
//!     vars,
//!     address: String::from("user@example.com"),
//! };
//! let _ = mailer.send_mail(&options).await.unwrap();
//! ```
//!

pub mod error;

use std::{
    collections::HashMap,
    time::Duration
};

use lettre::{
    message::{
        Message, header::ContentType,
        Mailbox
    },
    address::Address,
    AsyncSmtpTransport, AsyncTransport,
    Tokio1Executor,
};

use serde::Deserialize;

use rand::{thread_rng, Rng};

type SmtpTransport = AsyncSmtpTransport<Tokio1Executor>;

pub use crate::Locales;
use crate::mailfmt::{MailFmt, MailFmtError};
use error::MailerError;

/// Options to send email.
#[derive(Debug, Deserialize)]
pub struct MailerOptions {
    pub mail_type: String,
    pub locales: Vec<Locales>,
    pub vars: HashMap<String, String>,
    pub address: String,
}

/// Non-blocking email sender.
#[derive(Debug)]
pub struct Mailer {
    mailer: SmtpTransport,
    mailfmt: MailFmt,
    sender: Mailbox,
}

impl Mailer {
    /// Creates new mailer object.
    /// Caches mail templates by formatting with [MailFmt].
    pub fn new(relay: &str, sender: &str) -> Self {
        let mailer = SmtpTransport::builder_dangerous(relay)
            .timeout(Some(Duration::from_secs(8)))
            .build();
        let mailfmt = MailFmt::new();
        let sender = sender.parse::<Mailbox>().expect("invalid sender email");
        Self {
            mailer,
            mailfmt,
            sender,
        }
    }

    fn format_mail(&self, mail_type: &String, locales: Vec<Locales>, vars: &HashMap<String, String>) -> Result<(String, String, String), MailFmtError> {
        let boundary = Self::generate_boundary();

        let mail_template = self.mailfmt.fmt(mail_type, locales, vars)?;
        let multipart_headers = "Content-Transfer-Encoding: quoted-printable\nMime-Version: 1.0";

        let body = format!(
            "--{boundary}\n{multipart_headers}\nContent-Type: text/plain; charset=utf-8\n\n{}\n--{boundary}\n{multipart_headers}\nContent-Type: text/html; charset=utf-8\n\n{}\n--{boundary}--", 
            mail_template.plain_text,
            mail_template.html
        );

        Ok((boundary, mail_template.subject, body))
    }

    /// Sends the email
    pub async fn send_mail(&self, options: &MailerOptions) -> Result<(), MailerError> { 
        let (boundary, subject, body) = self.format_mail(&options.mail_type, options.locales.clone(), &options.vars)?;

        let email = Message::builder()
            .from(self.sender.clone())
            .to(Mailbox::new(None, options.address.parse::<Address>()?))
            .subject(subject)
            .header(ContentType::parse(&format!("multipart/alternative; boundary={boundary}")[..]).unwrap())
            .body(body)
            .unwrap();

        match self.mailer.send(email).await {
            Ok(_) => Ok(()),
            Err(e) => Err(MailerError::Error(Box::new(e)))
        }
    }

    // Generates random base16 string
    fn generate_boundary() -> String {
        static BASE16: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
        let mut rng = thread_rng();
        let mut random: u128 = rng.gen();

        let mut string = String::with_capacity(32);

        while random > 0 {
            let u4 = random & 15;
            random >>= 4;
            string.push(BASE16[u4 as usize]);
        }

        string
    }
}
