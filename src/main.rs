use dotenv;

use metw_mailer::web;
use metw_mailer::mailer;

// Checks for required environment variables
fn env(var: &'static str) -> &'static str { 
    let string = std::env::var(var).expect(&format!("environment variable {var} is required")[..]);
    Box::leak(string.into_boxed_str())
}

#[tokio::main]
#[allow(non_snake_case)]
async fn main() {
    dotenv::dotenv().ok();

    let HTTP_HOST = env("HTTP_HOST");
    let MAILER_RELAY = env("MAILER_RELAY");
    let MAILER_NOREPLY = env("MAILER_NOREPLY");
    let TOKEN = env("TOKEN");

    let mailer = mailer::Mailer::new(MAILER_RELAY, MAILER_NOREPLY);
    
    web::Web::new(&HTTP_HOST[..], mailer).await
        .route(String::from(TOKEN))
        .serve().await;
}
