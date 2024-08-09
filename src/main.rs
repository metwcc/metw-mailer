use std::collections::HashMap;

use metw_mailer::Mailer;


#[tokio::main]
async fn main() {
    let mailer = Mailer::new();
    
    let mut vars = HashMap::new();
    vars.insert(String::from("name"), String::from("metw"));

    mailer.send_mail("text: {name}", "html: {name}", vars).await;
}
