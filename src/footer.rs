use crate::Error;
use async_std::sync::{RwLock, RwLockReadGuard};
use async_std::task;
use std::mem::swap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tide::log;

/// The content for the KTH footer.
///
/// The footer is automatically fetched from the cortina block api and
/// kept up to date.
/// The `Footer` struct derives `Clone` so that it can be included in
/// the application state, which tide clones for each request.
/// This only clones the reference counted pointer.  All the clones
/// points to the same lock, containing the same string, which is kept
/// up-to-date.
#[derive(Clone)]
pub struct Footer {
    content: Arc<RwLock<String>>,
}

/// Where to fetch the KTH page footer
const FOOTER_URL: &str = "https://www.kth.se/cm/1.202278";

/// How long to keep the footer.
///
/// Some odd number, slightly more than an hour.
/// After this duration, the footer is fetched.  If fetch fails, old
/// data is kept.
const A_WHILE: Duration = Duration::from_secs(63 * 63);

impl Footer {
    pub fn new() -> Self {
        let content = Arc::new(RwLock::new("<!-- not loaded yet -->".to_string()));
        task::spawn(reload(content.clone()));
        Footer { content }
    }
    pub async fn get(&self) -> RwLockReadGuard<'_, String> {
        self.content.read().await
    }
}

async fn reload(data: Arc<RwLock<String>>) {
    loop {
        let start = Instant::now();
        match load_data(FOOTER_URL).await {
            Ok(mut new_data) => {
                log::info!(
                    "Loaded {} bytes of footer in {:?}",
                    new_data.len(),
                    start.elapsed()
                );
                let mut w = data.write().await;
                swap(&mut *w, &mut new_data);
            }
            Err(e) => {
                log::warn!("Failed to load footer: {}", e);
            }
        }

        task::sleep(A_WHILE).await;
    }
}

async fn load_data(url: &str) -> Result<String, Error> {
    let mut response = surf::get(url).await?;
    let status = response.status();
    if status != 200 {
        return Err(surf::Error::from_str(status, format!("{} from {}", status, url)).into());
    }
    Ok(response.body_string().await?)
}
