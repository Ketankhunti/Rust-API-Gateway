use std::{error::Error, process::{Child, Command}};

struct ChildProcessGuard(Child);

impl Drop for ChildProcessGuard {
    fn drop(&mut self) {
        if let Err(e) = self.0.kill() {
            eprintln!("Failed to kill mock backend process: {}", e);
        }
    }
}

/// The main integration test function.
#[tokio::test]
async fn test_gateway_proxy_and_auth() -> Result<(),Box<dyn Error>> {

    println!("Spawning mock backend process...");
    let mock_backend_process = Command::new("cargo")
        .arg("run")
        .arg("--example")
        .arg("server")
        .spawn()?;
    
    let _backend_guard = ChildProcessGuard(mock_backend_process);

    let client = reqwest::Client::new();

    // 1. Test Health Endpoint
    println!("Testing /health...");
    let health_res = client.get("http://127.0.0.1:8080/health").send().await?;
    assert_eq!(health_res.status(), reqwest::StatusCode::OK);
    assert_eq!(health_res.text().await?, "OK");
    println!("... /health OK");

    // 2. Test Public Route (should always succeed)
    println!("Testing public route /public...");
    let public_res = client.get("http://127.0.0.1:8080/public").send().await?;
    assert_eq!(public_res.status(), reqwest::StatusCode::OK);
    println!("... Public route OK");

    // 3. Test Private Route - No Token (should fail)
    println!("Testing private route /private (no token)...");
    let private_no_token_res = client.get("http://127.0.0.1:8080/private").send().await?;
    assert_eq!(private_no_token_res.status(), reqwest::StatusCode::UNAUTHORIZED);
    println!("... Private route (no token) correctly failed with 401");

    // 4. Test Private Route - Wrong Token (should fail)
    println!("Testing private route /api/private (wrong token)...");
    let private_wrong_token_res = client
        .get("http://127.0.0.1:8081/api/private")
        .header("Authorization", "Bearer wrong-key")
        .send()
        .await?;
    assert_eq!(private_wrong_token_res.status(), reqwest::StatusCode::UNAUTHORIZED);
    println!("... Private route (wrong token) correctly failed with 401");

    // 5. Test Private Route - Correct Token (should succeed)
    println!("Testing private route /api/private (correct token)...");
    let private_correct_token_res = client
        .get("http://127.0.0.1:8081/api/private")
        .header("Authorization", "Bearer super-secret-key")
        .send()
        .await?;
    assert_eq!(private_correct_token_res.status(), reqwest::StatusCode::OK);
    println!("... Private route (correct token) OK");

    Ok(())
}
