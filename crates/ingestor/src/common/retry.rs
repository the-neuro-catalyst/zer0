use crate::error::{IngestorError, Result};

use std::future::Future;

use std::time::Duration;

use tracing::warn;

pub async fn execute_with_retry<F, Fut, T>(operation: F) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = std::result::Result<T, backoff::Error<IngestorError>>>,
{
    let mut attempt = 0u64;
    loop {
        match operation().await {
            Ok(value) => return Ok(value),
            Err(e) => match e {
                backoff::Error::Permanent(err) => return Err(err),
                backoff::Error::Transient { err, .. } => {
                    attempt += 1;
                    if attempt > 10 {
                        return Err(err);
                    }
                    let delay = Duration::from_millis(100 * 2_u64.pow(attempt.min(10) as u32));
                    warn!("Retry attempt {} after {:?}", attempt, delay);
                    tokio::time::sleep(delay).await;
                }
            },
        }
    }
}

pub fn wrap_error(err: IngestorError) -> backoff::Error<IngestorError> {
    if err.is_transient() { backoff::Error::transient(err) } else { backoff::Error::permanent(err) }
}

#[cfg(test)]
mod tests {

    use super::*;

    use std::sync::Arc;

    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_retry_success_eventually() {
        let attempts = Arc::new(Mutex::new(0));
        let attempts_clone = attempts.clone();

        let result = execute_with_retry(|| {
            let attempts_clone = attempts_clone.clone();
            async move {
                let mut count = attempts_clone.lock().await;
                *count += 1;
                if *count < 3 {
                    Err(backoff::Error::transient(IngestorError::ConnectionError(
                        "Transient".into(),
                    )))
                } else {
                    Ok("Success")
                }
            }
        })
        .await;

        assert_eq!(result.unwrap(), "Success");
        assert_eq!(*attempts.lock().await, 3);
    }

    #[tokio::test]
    async fn test_retry_permanent_failure() {
        let attempts = Arc::new(Mutex::new(0));
        let attempts_clone = attempts.clone();

        let result: Result<()> = execute_with_retry(|| {
            let attempts_clone = attempts_clone.clone();
            async move {
                let mut count = attempts_clone.lock().await;
                *count += 1;
                Err(backoff::Error::permanent(IngestorError::ConfigurationError(
                    "Permanent".into(),
                )))
            }
        })
        .await;

        assert!(result.is_err());
        assert_eq!(*attempts.lock().await, 1);
    }
}
