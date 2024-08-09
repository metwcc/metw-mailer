use std::collections::HashMap;

use lettre::{
    message::{
        Message, header::ContentType,
        Mailbox
    },
    address::Address,
    AsyncSmtpTransport, AsyncTransport,
    Tokio1Executor,
};

use rand::{thread_rng, Rng};

type SmtpTransport = AsyncSmtpTransport<Tokio1Executor>;

pub use crate::mailfmt::Locales;
use crate::mailfmt::MailFmt;


pub struct Mailer {
    mailer: SmtpTransport,
    mailfmt: MailFmt,
}

impl Mailer {
    pub fn new() -> Self {
        let mailer = SmtpTransport::builder_dangerous("metw.dev").build();
        let mailfmt = MailFmt::new();
        Self {
            mailer,
            mailfmt
        }
    }

    pub async fn send_mail(&self, mail_type: &String, locale: Locales, vars: HashMap<String, String>) {
        let boundary = Self::generate_boundary();
        
        let (subject, plain_text, html) = self.mailfmt.mail(mail_type, locale, vars);

        let multipart_headers = "\
Content-Transfer-Encoding: quoted-printable
Mime-Version: 1.0";

        let body = format!("\
--{boundary}
{multipart_headers}
Content-Type: text/plain; charset=utf-8

{plain_text}


--{boundary}
{multipart_headers}
Content-Type: text/html; charset=utf-8

{html}

--{boundary}--
");

        let email = Message::builder()
            .from(Mailbox::new(Some(String::from("metw")), Address::new("metw", "metw.dev").unwrap()))
            .to(Mailbox::new(None, Address::new("me", "metw.cc").unwrap()))
            .subject(subject)
            .header(ContentType::parse(&format!("multipart/alternative; boundary={boundary}")[..]).unwrap())
            .body(body)
            .unwrap();

        match self.mailer.send(email).await {
            Ok(_) => println!("Email sent succesfully!"),
            Err(e) => panic!("Could not send email: {e:?}")
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
