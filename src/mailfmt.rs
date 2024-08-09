use include_dir::{include_dir, Dir};

use std::collections::HashMap;

use strfmt::strfmt;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Locales {
    en_US,
    tr
}

#[derive(Debug)]
struct MailTemplate {
    subject: HashMap<Locales, String>,
    html: HashMap<Locales, String>,
    plain_text: HashMap<Locales, String>,
}

type MailTemplates = HashMap<String, MailTemplate>;

impl MailTemplate {
    fn new() -> Self {
        Self {
            subject: HashMap::new(),
            html: HashMap::new(),
            plain_text: HashMap::new(),
        }
    }
}

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

    pub fn mail(&self, mail: &String, locale: Locales, vars: HashMap<String, String>) -> (String, String, String) {
        let mail = self.templates.get(mail).unwrap();
        (
            strfmt(mail.subject.get(&locale).unwrap(), &vars).unwrap(),
            strfmt(mail.plain_text.get(&locale).unwrap(), &vars).unwrap(),
            strfmt(mail.html.get(&locale).unwrap(), &vars).unwrap()
        )
    }

    const MAILS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/mails/");

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
            let mut template = MailTemplate::new();
            let mut locales = HashMap::<Locales, HashMap<String, String>>::new();
            let mut plain_text = String::new();
            let mut html = String::new();

            for line in file.contents_utf8().unwrap().lines() {
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
                        let locale = match locale_str {
                            "en_US" => Locales::en_US,
                            "tr" => Locales::tr,
                            locale => panic!("unknown locale: {locale}")
                        };
                        locales.insert(locale.clone(), HashMap::new());
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
                                .insert(String::from(key.trim()), String::from(value.trim()).replace("{{", "{"). replace("}}", "}"));
                        }
                    },
                    ParseMode::LocaleSubjectInsert(ref locale) => {
                        if line.len() == 0 { continue }
                        template.subject.insert(locale.clone(), String::from(line.trim()).replace("{{", "{"). replace("}}", "}"));
                        state = ParseMode::LocaleInsert(locale.clone())
                    }
                    ParseMode::PlainTextInsert => {
                        plain_text.push_str(line);
                        plain_text.push_str("\n");
                    }
                    ParseMode::HtmlInsert => {
                        html.push_str(line);
                        html.push_str("\n");
                    }
                    _ => ()
                };
            }

            for (locale, map) in locales.iter() {
                template.plain_text.insert(
                    locale.clone(), 
                    strfmt(&plain_text, map).unwrap()
                );
                template.html.insert(
                    locale.clone(), 
                    strfmt(&html, map).unwrap()
                );
            }

            let file_stem = String::from(file.path().file_stem().unwrap().to_str().unwrap());
            templates.insert(file_stem, template);
        }
        templates
    }
}
