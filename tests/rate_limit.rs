use std::process::{Child, Command};

use anyhow::Error;
use futures::future::join_all;

struct ChildProcessGuard(Child);

impl Drop for ChildProcessGuard {
    fn drop(&mut self) {
        if let Err(e) = self.0.kill() {
            eprintln!("Failed to kill mock backend process: {}", e);
        }
    }
}

async fn wait_for_server(url: &str, timeout_secs: u64) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let start = std::time::Instant::now();

    loop {
        if start.elapsed().as_secs() > timeout_secs {
            return Err(anyhow::anyhow!("Timeout waiting for server at {url}"));
        }

        match client.get(url).send().await {
            Ok(_) => return Ok(()), // Server is up
            Err(_) => {
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
        }
    }
}


#[tokio::test]
async fn test_gateway_flows() -> Result<(), Error> {
    // 1. Setup: Spawn servers
    println!("Spawning mock backend process...");
    let mock_backend_process = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("server")
        .spawn()?;
    let _backend_guard = ChildProcessGuard(mock_backend_process);
   
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    // 2. Test Rate Limiting
    println!("Testing rate limiting flow...");
    let client = reqwest::Client::new();
    
    let rate_limit_url = "http://127.0.0.1:8080/api/ratelimit";

    // Send 7 requests concurrently. The limit is 5.
    let mut tasks = Vec::new();
    for _ in 0..7 {
        let client = client.clone();
        let task = tokio::spawn(async move {
            client
            .get(rate_limit_url)
            .header("Authorization", "user-key-for-alice")
            .send().await
        });
        tasks.push(task);
    }

    let responses = join_all(tasks).await;

    let mut success_count = 0;
    let mut rate_limited_count = 0;

    for res in responses {
        let res = res??; // Handle JoinError and ReqwestError
        if res.status() == reqwest::StatusCode::OK {
            success_count += 1;
        } else if res.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            rate_limited_count += 1;
        }
    }

    // Assert that exactly 5 requests succeeded and 2 were rate-limited.
    assert_eq!(success_count, 5, "Expected 5 successful requests");
    assert_eq!(rate_limited_count, 2, "Expected 2 rate-limited requests");
    println!("... Rate limiting test OK (5 success, 2 throttled)");

    // 3. Test Health Endpoint
    println!("Testing /health...");
    let health_res = client.get("http://127.0.0.1:8080/health").send().await?;
    assert_eq!(health_res.status(), reqwest::StatusCode::OK);
    println!("... /health OK");

    Ok(())
}
