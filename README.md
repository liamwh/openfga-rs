# OpenFGA Rust SDK

Automatically generated Rust SDK for the OpenFGA API, using the protobufs from [OpenFGA API](https://github.com/openfga/api).

## Getting Started

You can instantiate the SDK like so:

```rust
use openfga_rs::open_fga_service_client::OpenFgaServiceClient;

#[tokio::main]
async fn main() -> Result<()> {
    let _openfga_client = OpenFgaServiceClient::connect("http://[::1]:8081")
      .await
      .expect("Expected to connect to the OpenFGA gRPC endpoint successfully");

    println!("Connected to OpenFGA service");
    Ok(())
}
```
