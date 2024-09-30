//! Interceptors for the gRPC client to authenticate with `OpenFGA`.
use http::header::AUTHORIZATION;
use tonic::service::interceptor::Interceptor;

/// Create a simple gRPC `Interceptor` that attaches a given access token to any request
/// a client sends. The token is attached with the `Bearer` auth-scheme.
///
/// The access token is typically a token for shared key authentication with `OpenFGA`.
///
/// The interceptor does not insert the access token if the intercepted call
/// already has an `Authorization` header.
///
/// # Examples
/// ```no_run
/// use openfga_rs::open_fga_service_client::OpenFgaServiceClient;
/// use openfga_rs::authentication::BearerTokenInterceptor;
/// use tonic::transport::Endpoint;
///
/// #[tokio::main]
/// async fn main() {
///     let interceptor = BearerTokenInterceptor::new("my-token").unwrap();
///     let channel = Endpoint::from_static("http://[::1]:50051")
///         .connect()
///         .await
///         .unwrap();
///     let _client = OpenFgaServiceClient::with_interceptor(channel, interceptor);
///
///     println!("Connected to OpenFGA service");
/// }
/// ```
#[derive(Clone, veil::Redact)]
pub struct BearerTokenInterceptor {
    #[redact]
    pub token: tonic::metadata::MetadataValue<tonic::metadata::Ascii>,
}

impl BearerTokenInterceptor {
    /// Create a new interceptor with the given access token.
    ///
    /// # Errors
    /// Fails if "Bearer {token}" is not a valid ASCII string.
    pub fn new(token: &str) -> Result<Self, tonic::metadata::errors::InvalidMetadataValue> {
        let val = tonic::metadata::MetadataValue::<tonic::metadata::Ascii>::try_from(&format!(
            "Bearer {token}",
        ))?;
        Ok(Self { token: val })
    }
}

impl Interceptor for BearerTokenInterceptor {
    fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        let mut request = request;
        let metadata = request.metadata_mut();
        if !metadata.contains_key(AUTHORIZATION.as_str()) {
            metadata.insert(AUTHORIZATION.as_str(), self.token.clone());
        }
        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_token_added() {
        let mut interceptor = BearerTokenInterceptor::new("my-token").unwrap();

        let request = tonic::Request::new(());
        assert!(request.metadata().is_empty());
        let modified_request = interceptor.call(request).unwrap();

        let metadata = modified_request.metadata();
        assert!(metadata.contains_key("authorization"));
        assert_eq!(
            metadata.get("authorization").unwrap().to_str().unwrap(),
            "Bearer my-token"
        );
    }

    #[test]
    fn test_access_token_not_added_if_authorization_present() {
        let mut interceptor = BearerTokenInterceptor::new("my-token").unwrap();

        let mut request = tonic::Request::new(());
        assert!(request.metadata().is_empty());
        request
            .metadata_mut()
            .insert("authorization", "Bearer existing-token".parse().unwrap());

        let modified_request = interceptor.call(request).unwrap();
        assert_eq!(
            modified_request
                .metadata()
                .get("authorization")
                .unwrap()
                .to_str()
                .unwrap(),
            "Bearer existing-token"
        );
    }
}
