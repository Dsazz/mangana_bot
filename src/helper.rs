use tokio::time::{Duration, interval_at, Instant};
use futures::Future;

pub async fn repeat_work<Fut>(work: impl FnOnce() -> Fut + Copy, interval: u64)
    where Fut: Future<Output = ()>,
{
    let mut interval = interval_at(Instant::now(), Duration::from_secs(interval));

    loop {
        interval.tick().await;
        work().await;
        tokio::task::yield_now().await;
    }
}