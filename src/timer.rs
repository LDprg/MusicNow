/// Abstraction layer to make all platforms work with time correctly
use std::time::Duration;

pub async fn sleep(dur: Duration) {
    #[cfg(feature = "web")]
    gloo_timers::future::sleep(dur).await;
    #[cfg(not(feature = "web"))]
    tokio::time::sleep(dur).await;
}

pub struct Instant {
    #[cfg(not(feature = "web"))]
    value: tokio::time::Instant,
    #[cfg(feature = "web")]
    value: web_time::Instant,
}

impl Instant {
    pub fn now() -> Self {
        Self {
            #[cfg(not(feature = "web"))]
            value: tokio::time::Instant::now(),
            #[cfg(feature = "web")]
            value: web_time::Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.value.elapsed()
    }
}
