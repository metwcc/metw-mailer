use std::collections::HashMap;

use metw_mailer::Mailer;
use metw_mailer::mailfmt::Locales;


#[tokio::main]
async fn main() {
    let mailer = Mailer::new();
    
  //let mut vars = HashMap::new();
  //vars.insert(String::from("name"), String::from("metw"));
  //vars.insert(String::from("verify"), String::from("https://metw.cc"));
  //vars.insert(String::from("remove_email"), String::from("https://metw.cc"));

  //mailer.send_mail(&String::from("welcome"), Locales::tr, vars).await;

  //let mut vars = HashMap::new();
  //vars.insert(String::from("code"), String::from("123213"));

  //mailer.send_mail(&String::from("verify"), Locales::tr, vars).await;
}
