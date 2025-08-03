# Rust API Gateway

![Architecture Diagram](docs/architecture.png)

A minimal, high-performance, and self-hosted API Gateway built in Rust. This project provides a lightweight yet powerful solution for managing access to your backend services, perfect for solo developers and small teams.

---

## ✨ Features

- **Dynamic Routing**  
  Configure all routes via a simple YAML file. No code changes or restarts needed to add, remove, or change routes.

- **Reverse Proxy**  
  Forwards client requests to the appropriate backend services seamlessly.

- **Robust Authentication**  
  - **JWT (JSON Web Tokens):** Secure stateless authentication for users.  
  - **API Keys:** Simple, effective authentication for server-to-server communication.  
  - **Role-Based Access Control (RBAC):** Restrict access to specific routes based on roles defined in the JWT or API key data.

- **Rate Limiting**  
  Protect your services from abuse with a configurable Token Bucket algorithm, applied per client IP address.

- **Configuration Hot-Reload**  
  Automatically detects and applies changes to `gateway_config.yaml` and `api_keys.yaml` without any downtime.

- **CLI-Driven**  
  Easy to run and configure via command-line arguments.

---

## 🚀 Getting Started

### Prerequisites

- Rust toolchain (latest stable version recommended)

### Installation

You can run the project directly from the source or install it as a command-line application.

```bash
# To run directly from source
cargo run

# To install the binary
cargo install --path .

⚙️ Configuration
The gateway is configured using three main files:

1. Environment Variables (.env)
This file holds the master secret for the entire gateway and should never be committed to version control.

# .env

# The master secret for signing and verifying all JWTs.
# Use a long, random string for production.
JWT_SECRET="a-very-long-and-random-string-that-is-hard-to-guess"

2. API Key Store (api_keys.yaml)
This file manages all valid API keys and their associated user data and roles.
# api_keys.yaml
keys:
  "user-key-for-alice":
    user_id: "alice@example.com"
    roles: ["user"]
    status: "active"

  "admin-key-for-carol":
    user_id: "carol@example.com"
    roles: ["admin", "user"]
    status: "active"

  "revoked-key-for-dave":
    user_id: "dave@example.com"
    roles: ["user"]
    status: "revoked" # Keys can be easily revoked

3. Main Gateway Config (gateway_config.yaml)
This is the central configuration file that defines the server, routes, and authentication requirements.

# Main server configuration
server:
  addr: "127.0.0.1:8080"

# Defines the location of the API key store
identity:
  api_key_store_path: "./api_keys.yaml"

# --- Route Definitions ---
routes:
  # A public route with no authentication
  - name: "public_service"
    path: "/api/public"
    destination: "http://localhost:9001/some/path"

  # A route protected by an API key requiring the 'user' role
  - name: "user_service"
    path: "/api/user"
    destination: "http://localhost:9002"
    auth:
      type: "apikey"
      roles: ["user"]
    rate_limit:
      requests: 10
      period: "1m" # 10 requests per minute

  # A route protected by a JWT requiring the 'admin' role
  - name: "admin_dashboard"
    path: "/api/admin"
    destination: "http://localhost:9003"
    auth:
      type: "jwt"
      roles: ["admin"]

▶️ Running the Gateway

Default (uses gateway_config.yaml in the current directory)
cargo run

With a Custom Config File
cargo run -- --config /path/to/your/custom_config.yaml
# OR
cargo run -- -c /path/to/your/custom_config.yaml

🧪 Testing
The project includes a comprehensive integration test suite.

# Run all tests
cargo test

# Run a specific test and show output
cargo test --test test_file_name -- --nocapture