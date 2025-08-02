
use anyhow::Result;
use serde_json::{json, Value};
use std::process::{Child, Command};

struct ChildProcessGuard(Child);

impl Drop for ChildProcessGuard {
    fn drop(&mut self) {
        if let Err(e) = self.0.kill() {
            eprintln!("Failed to kill mock backend process: {}", e);
        }
    }
}

#[tokio::test]
async fn test_gateway_proxy_and_health() -> Result<()> {
    
    println!("Spawning mock backend process...");
    let mock_backend_process = Command::new("cargo")
        .arg("run")
        .arg("--example")
        .arg("server")
        .spawn()?;
    
    let _backend_guard = ChildProcessGuard(mock_backend_process);

    let client = reqwest::Client::new();

    // 4. Test the /health endpoint.
    let health_res = client.get("http://127.0.0.1:8080/health").send().await?;
    assert_eq!(health_res.status(), reqwest::StatusCode::OK);
    assert_eq!(health_res.text().await?, "OK");
    println!("/health endpoint... OK");

    // 5. Test the first proxied route.
    let service_one_res = client.get("http://127.0.0.1:8080/service-one").send().await?;
    assert_eq!(service_one_res.status(), reqwest::StatusCode::OK);
    println!("/service-one proxy... OK");

    // 6. Test the second proxied route.
    let service_two_res = client.get("http://127.0.0.1:8080/service-two").send().await?;
    assert_eq!(service_two_res.status(), reqwest::StatusCode::OK);

    println!("/service-two proxy... OK");

    Ok(())
}
