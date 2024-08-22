use std::sync::Arc;

use axum::{
    extract::{Json, Request, Extension},
    http::{HeaderMap, StatusCode},
    response::Response,
    middleware::{self, Next},
    routing, Router
};

use crate::mailer::{ Mailer, MailerOptions };
use super::response;

/// Initializes a [Router] object which requires authentication
pub fn router(mailer: Arc<Mailer>, token: String) -> Router {
    let mailer = Arc::clone(&mailer);
    let uptime = std::time::Instant::now();

    Router::new()
        .route("/mail", routing::post(move |Json(options): Json<MailerOptions>| {
            async move {
                Json(
                    if let Err(e) = mailer.send_mail(&options).await {
                        response::SendMail {
                            success: false,
                            message: format!("{e:?}")
                        }
                    } else {
                        response::SendMail {
                            success: true,
                            message: String::from("OK")
                        }
                    }
                )
            }
        }))
        .route("/", routing::get(move || {
            async move {
                Json(response::Uptime {
                    uptime: uptime.elapsed().as_secs(),
                    message: String::from("OK")
                })
            }
        }))
        .layer(middleware::from_fn(auth))
        .layer(Extension(token))
}

// Token authentication with headers
async fn auth(
    headers: HeaderMap,
    Extension(token): Extension<String>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(token2) = headers.get("token") {
        if *token2 == token { 
            return Ok(next.run(request).await)
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
