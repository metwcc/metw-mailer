use axum::Router;
use tokio::net::TcpListener;
use std::sync::Arc;

use crate::mailer::Mailer;

mod response;
mod routes;

/// HTTP REST API for mailer.
pub struct Web {
    mailer: Arc<Mailer>,
    app: Router,
    listener: TcpListener
}

impl Web {
    pub async fn new(host: &str, mailer: Mailer) -> Self {
        let app = Router::new();
        let listener = TcpListener::bind(host).await.expect(&format!("cannot bind host {host}")[..]);
        Self {
            app,
            listener,
            mailer: Arc::new(mailer)
        }
    }

    pub fn route(mut self, token: String) -> Self {
        let mailer = Arc::clone(&self.mailer);
        let router = routes::router(Arc::clone(&mailer), token);

        self.app = self.app.merge(router);

        self
    }

    pub async fn serve(self) {
        axum::serve(self.listener, self.app).await.expect("cannot serve app")
    }
}
