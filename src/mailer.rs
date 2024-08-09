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

use strfmt::strfmt;

type SmtpTransport = AsyncSmtpTransport<Tokio1Executor>;


pub struct Mailer {
    mailer: SmtpTransport,
}

impl Mailer {
    pub fn new() -> Self {
        let mailer = SmtpTransport::builder_dangerous("metw.dev").build();
        Self {
            mailer,
        }
    }

    pub async fn send_mail(self, text: &str, html: &str, vars: HashMap<String, String>) {
        let boundary = Self::generate_boundary();

        let text = strfmt(text, &vars).unwrap();
        let html = strfmt(html, &vars).unwrap();

        let multipart_headers = "\
Content-Transfer-Encoding: quoted-printable
Content-Type: text/plain; charset=utf-8
Mime-Version: 1.0";

        let body = format!("\
--{boundary}
{multipart_headers}

{text}


--{boundary}
{multipart_headers}

{html}

--{boundary}--
");

        let email = Message::builder()
            .from(Mailbox::new(Some(String::from("metw")), Address::new("metw", "metw.dev").unwrap()))
            .to(Mailbox::new(None, Address::new("me", "metw.cc").unwrap()))
            .subject("Metw Girişini Doğrula")
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
