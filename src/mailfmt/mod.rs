pub mod error;

use include_dir::{include_dir, Dir};

use html_minifier::HTMLMinifier;

use std::collections::HashMap;

use strfmt::strfmt;

pub use error::MailFmtError;

use crate::Locales;

#[derive(Debug, Clone)]
pub struct MailTemplate {
    pub subject: String,
    pub html: String,
    pub plain_text: String,
}

impl MailTemplate {
    fn new() -> Self {
        Self {
            subject: String::new(),
            html: String::new(),
            plain_text: String::new(),
        }
    }
}

type MailTemplates = HashMap<
        String,
        HashMap<
            Locales,
            MailTemplate
        >
    >;

/// Formats email subject and body.
#[derive(Debug)]
pub struct MailFmt {
    templates: MailTemplates,
}

impl MailFmt {
    pub fn new() -> Self {
        Self {
            templates: Self::load_templates(),
        }
    }

    pub fn fmt(&self, mail: &String, locales: Vec<Locales>, vars: &HashMap<String, String>) -> Result<MailTemplate, MailFmtError> {
        let mail = match self.templates.get(mail) {
            Some(mail) => mail,
            None => return Err(MailFmtError::UnknownTemplate(mail.clone()))
        };

        // Gets first available locale
        let mut mail2 = None;
        for locale in &locales {
            if let Some(mail) =  mail.get(&locale) { mail2 = Some(mail); break }
        }

        let mail = match mail2 {
            Some(mail) => mail,
            None => return Err(MailFmtError::UnknownLocale(locales.clone()))
        };


        Ok(MailTemplate {
            subject: strfmt(&mail.subject, &vars).unwrap(),
            plain_text: strfmt(&mail.plain_text, &vars).unwrap(),
            html: strfmt(&mail.html, &vars).unwrap(),
        })
    }

    const MAILS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/mails/");
    
    // Loads mail templates from mails/ directory.
    fn load_templates() -> MailTemplates {
        enum ParseMode {
            LocaleInsert (Locales),
            LocaleSubjectInsert (Locales),
            PlainTextInsert,
            HtmlInsert,
            None
        }

        let mut templates: MailTemplates = HashMap::new();

        for file in Self::MAILS_DIR.files() {
            let mut state = ParseMode::None;

            #[derive(Debug)]
            struct LocaleTemplate {
                subject: String,
                map: HashMap<String, String>
            }

            let mut locales = HashMap::<Locales, LocaleTemplate>::new();
            let mut plain_text = String::new();
            let mut html = HTMLMinifier::new();

            for line in file.contents_utf8().expect("Template file name contains invalid UTF-8").lines() {
                let mut cont = true;
                match line {
                    "#HTML" => {
                        state = ParseMode::HtmlInsert 
                    },
                    "#PLAINTEXT" => {
                        state = ParseMode::PlainTextInsert 
                    },
                    "#END" => {
                        state = ParseMode::None 
                    },
                    line if line.starts_with('@') => {
                        let locale_str = &line[1..line.len() - 1];
                        let locale = Locales::from(locale_str).unwrap();
                        locales.insert(
                            locale.clone(), 
                            LocaleTemplate {
                                subject: String::new(),
                                map: HashMap::new(),
                            }
                        );
                        state = ParseMode::LocaleSubjectInsert(locale);
                    },
                    _ => { cont = false }
                }

                if cont { continue }

                match state {
                    ParseMode::LocaleInsert(ref locale) => {
                        if line.len() == 0 { continue }
                        if let Some((key, value)) = line.split_once("=") {
                            locales.get_mut(locale).unwrap()
                                .map
                                .insert(
                                    String::from(key.trim()), 
                                    String::from(value.trim()).replace("{{", "{").replace("}}", "}")
                                );
                        }
                    },
                    ParseMode::LocaleSubjectInsert(ref locale) => {
                        if line.len() == 0 { continue }
                        locales.get_mut(locale).unwrap()
                            .subject.push_str(line.trim());
                        state = ParseMode::LocaleInsert(locale.clone())
                    }
                    ParseMode::PlainTextInsert => {
                        plain_text.push_str(line);
                        plain_text.push_str("\n");
                    }
                    ParseMode::HtmlInsert => {
                        html.digest(line).unwrap();
                        html.digest("\n").unwrap();
                    }
                    _ => ()
                };
            }

            let html = String::from_utf8(html.get_html().to_vec()).unwrap();

            let mut template: HashMap<Locales, MailTemplate> = HashMap::new();

            for (locale, locale_template) in locales.iter() {
                let mut mail_template = MailTemplate::new();
                let subject = &locale_template.subject;
                macro_rules! fmt {
                    ($field: ident) => { 
                        mail_template.$field = strfmt(&$field, &locale_template.map)
                            .expect(&format!("Cannot format locale {locale:?}")[..]) 
                    };
                }
                fmt!(subject);
                fmt!(plain_text);
                fmt!(html);

                template.insert(locale.clone(), mail_template);
            }

            let file_stem = String::from(file.path()
                .file_stem().expect("Cannot get file_stem of template")
                .to_str().expect("Template file_stem contains invalid unicode"));
            templates.insert(file_stem, template);
        }

        templates
    }
}
