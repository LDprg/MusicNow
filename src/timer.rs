/// Abstraction layer to make all platforms work with time correctly
use std::time::Duration;

pub async fn sleep(dur: Duration) {
    #[cfg(feature = "web")]
    gloo_timers::future::sleep(dur).await;
    #[cfg(not(feature = "web"))]
    tokio::time::sleep(dur).await;
}
