use reqwest_middleware::{Middleware, Next, Result};
use tauri::http::Extensions;
use tauri_plugin_http::reqwest::{Request, Response};

pub struct RateLimiterMiddleware;

impl Default for RateLimiterMiddleware {
    fn default() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Middleware for RateLimiterMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        next.run(req, extensions).await
    }
}
