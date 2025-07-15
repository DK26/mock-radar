use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use serde_json::Value;
use std::collections::HashMap;
use std::marker::PhantomData;
use tower::ServiceExt;

use mock_radar::SharedQRadarMock;

/// Standard header names for compile-time safety
pub mod headers {
    pub const CONTENT_TYPE: &str = "Content-Type";
    pub const ACCEPT: &str = "Accept";
    pub const VERSION: &str = "Version";
    pub const SEC: &str = "SEC";
    pub const AUTHORIZATION: &str = "Authorization";
}

/// QRadar API versions for compile-time safety
pub mod api_versions {
    pub const V12_0: &str = "12.0";
    pub const V11_0: &str = "11.0";
    pub const V10_0: &str = "10.0";
}

/// Type-safe builder for creating HTTP test requests with compile-time method enforcement.
///
/// This builder uses the type system to guide you through creating requests correctly,
/// preventing invalid combinations and ensuring you set required parameters.
///
/// **IMPORTANT**: NO DEFAULT HEADERS are set. Every header must be explicitly configured
/// in your tests to ensure complete visibility of what's being tested.
///
/// The builder uses the `mime` crate for type-safe MIME types and provides constants
/// for common headers and QRadar-specific values.
///
/// # Example Usage
///
/// ```rust
/// use crate::common::{TestRequest, headers, api_versions};
///
/// // Type-safe approach using mime crate
/// TestRequest::get("/api/reference_data/sets")
///     .content_type(mime::APPLICATION_JSON)    // Type-safe mime types
///     .accept(mime::APPLICATION_JSON)          // No typos possible
///     .version(api_versions::V12_0)         // Clear version specification
///     .sec_token(REGISTERED_SEC_TOKEN)
///     .send()
///     .await
///     .assert_status(StatusCode::OK);
///
/// // Explicit headers for complete transparency
/// TestRequest::post("/api/reference_data/sets")
///     .content_type(mime::APPLICATION_JSON)    // Explicit Content-Type
///     .accept(mime::APPLICATION_JSON)          // Explicit Accept header
///     .version(api_versions::V12_0)         // Clear version specification
///     .sec_token(REGISTERED_SEC_TOKEN)
///     .query_param("element_type", "IP")
///     .query_param("name", "test_set")
///     .send()
///     .await
///     .assert_status(StatusCode::CREATED);
///
/// // Testing different MIME types
/// TestRequest::post("/api/reference_data/sets")
///     .content_type(mime::APPLICATION_XML)     // Different content type
///     .accept(mime::APPLICATION_JSON)          // JSON response expected
///     .version(api_versions::V12_0)
///     .sec_token(REGISTERED_SEC_TOKEN)
///     .send()
///     .await
///     .assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
///
/// // Testing response structure
/// let response_data = TestRequest::post("/api/reference_data/sets")
///     .content_type(mime::APPLICATION_JSON)
///     .accept(mime::APPLICATION_JSON)
///     .version(api_versions::V12_0)
///     .sec_token(REGISTERED_SEC_TOKEN)
///     .query_param("element_type", "IP")
///     .send()
///     .await
///     .assert_status(StatusCode::CREATED)
///     .assert_deserializes_to::<TestPostResponse>();  // Assert structure matches
///
/// // Testing malformed queries (raw query string)
/// TestRequest::post("/api/reference_data/sets")
///     .json_content_and_accept()
///     .version(api_versions::V12_0)
///     .sec_token(REGISTERED_SEC_TOKEN)
///     .malformed_query("element_type=IP&invalid syntax here&more=bad")
///     .send()
///     .await
///     .assert_status(StatusCode::BAD_REQUEST);
/// ```

// Type markers for HTTP methods
pub struct Get;
pub struct Post;
pub struct Put;
pub struct Delete;

/// The main test request builder, parameterized by HTTP method type
pub struct TestRequest<Method> {
    method: http::Method,
    base_uri: String,
    query_string: Option<String>,
    headers: HashMap<String, String>,
    body: Body,
    shared_mock: Option<SharedQRadarMock>,
    _method: PhantomData<Method>,
}

/// Entry point for creating typed test requests
impl TestRequest<()> {
    /// Creates a new GET request builder
    pub fn get(uri: impl AsRef<str>) -> TestRequest<Get> {
        TestRequest::new(http::Method::GET, uri.as_ref())
    }

    /// Creates a new POST request builder
    pub fn post(uri: impl AsRef<str>) -> TestRequest<Post> {
        TestRequest::new(http::Method::POST, uri.as_ref())
    }

    /// Creates a new PUT request builder
    pub fn put(uri: impl AsRef<str>) -> TestRequest<Put> {
        TestRequest::new(http::Method::PUT, uri.as_ref())
    }

    /// Creates a new DELETE request builder
    pub fn delete(uri: impl AsRef<str>) -> TestRequest<Delete> {
        TestRequest::new(http::Method::DELETE, uri.as_ref())
    }
}

impl<Method> TestRequest<Method> {
    fn new(http_method: http::Method, uri: &str) -> Self {
        // NO DEFAULT HEADERS - everything must be explicit in tests
        let headers = HashMap::new();

        Self {
            method: http_method,
            base_uri: uri.to_string(),
            query_string: None,
            headers,
            body: Body::empty(),
            shared_mock: None,
            _method: PhantomData,
        }
    }

    /// Sets the shared QRadar mock instance for the request
    pub fn with_mock(mut self, mock: SharedQRadarMock) -> Self {
        self.shared_mock = Some(mock);
        self
    }

    /// Sets a header value - for custom headers not covered by typed methods
    pub fn header(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.headers
            .insert(key.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Sets the Content-Type header using mime types
    pub fn content_type(self, content_type: mime::Mime) -> Self {
        self.header(headers::CONTENT_TYPE, content_type.as_ref())
    }

    /// Sets the Accept header using mime types
    pub fn accept(self, accept: mime::Mime) -> Self {
        self.header(headers::ACCEPT, accept.as_ref())
    }

    /// Sets the Version header with compile-time safety
    pub fn version(self, version: &'static str) -> Self {
        self.header(headers::VERSION, version)
    }

    /// Sets the SEC token header
    pub fn sec_token(self, token: impl AsRef<str>) -> Self {
        self.header(headers::SEC, token)
    }

    /// Adds a single query parameter (URL-encoded automatically)
    /// Parameters preserve order for testing parameter order sensitivity
    pub fn query_param(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        let encoded_param = format!(
            "{}={}",
            urlencoding::encode(key.as_ref()),
            urlencoding::encode(value.as_ref())
        );

        self.query_string = Some(match self.query_string {
            Some(existing) => format!("{existing}&{encoded_param}"),
            None => encoded_param,
        });

        self
    }

    /// Adds multiple query parameters (URL-encoded automatically)
    pub fn query_params(
        mut self,
        params: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    ) -> Self {
        for (key, value) in params {
            self = self.query_param(key, value);
        }
        self
    }

    /// Sets a completely raw, malformed query string for testing error handling
    /// This bypasses all URL encoding and validation - use for testing broken queries
    ///
    /// # Examples
    /// ```rust
    /// // Test completely malformed syntax
    /// .malformed_query("invalid&&&syntax=here&=no_key&value_no_key")
    ///
    /// // Test edge cases the real API might receive
    /// .malformed_query("fields=name&fields=name&duplicate=param")
    ///
    /// // Test encoding issues
    /// .malformed_query("name=test with spaces&other=value%XX")
    /// ```
    pub fn malformed_query(mut self, raw_query: impl AsRef<str>) -> Self {
        self.query_string = Some(raw_query.as_ref().to_string());
        self
    }

    /// Sets the request body
    pub fn body(mut self, body: Body) -> Self {
        self.body = body;
        self
    }

    /// Sets a JSON body from a serde_json::Value
    pub fn json_body(mut self, json: Value) -> Self {
        let body_str = serde_json::to_string(&json).expect("Failed to serialize JSON body");
        self.body = Body::from(body_str);
        self
    }

    /// Builds the complete URI with query string
    fn build_uri(&self) -> String {
        match &self.query_string {
            Some(query) => format!("{}?{}", self.base_uri, query),
            None => self.base_uri.clone(),
        }
    }

    /// Executes the request and returns a TestResponse for assertions
    pub async fn send(self) -> TestResponse {
        let mock = self
            .shared_mock
            .clone()
            .unwrap_or_else(SharedQRadarMock::default);
        let router = mock_radar::create_routes();
        let uri = self.build_uri();

        let mut request_builder = Request::builder().method(self.method).uri(&uri);

        // Add all headers
        for (key, value) in &self.headers {
            request_builder = request_builder.header(key, value);
        }

        let request = request_builder
            .body(self.body)
            .expect("Failed to build request");

        let response = router
            .with_state(mock)
            .oneshot(request)
            .await
            .expect("Failed to get response");

        TestResponse::new(response).await
    }
}

/// Represents a test response with convenient assertion methods
pub struct TestResponse {
    pub status: StatusCode,
    pub body_bytes: Vec<u8>,
    pub body_json: Option<Value>,
    pub body_text: String,
}

impl TestResponse {
    async fn new(response: axum::response::Response) -> Self {
        let status = response.status();
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read response body");

        let body_text = String::from_utf8_lossy(&body_bytes).to_string();
        let body_json = serde_json::from_slice(&body_bytes).ok();

        Self {
            status,
            body_bytes: body_bytes.to_vec(),
            body_json,
            body_text,
        }
    }

    /// Asserts that the response has the expected status code
    pub fn assert_status(self, expected: StatusCode) -> Self {
        assert_eq!(self.status, expected, "Unexpected status code");
        self
    }

    /// Asserts that the response body matches the expected JSON value
    pub fn assert_json(self, expected: Value) -> Self {
        let actual = self
            .body_json
            .as_ref()
            .expect("Response body is not valid JSON");
        assert_eq!(actual, &expected, "Response JSON mismatch");
        self
    }

    /// Asserts that the response body contains the specified text
    pub fn assert_body_contains(self, text: &str) -> Self {
        assert!(
            self.body_text.contains(text),
            "Response body does not contain expected text: {text}"
        );
        self
    }

    /// Asserts that the response body matches exactly
    pub fn assert_body_text(self, expected: &str) -> Self {
        assert_eq!(self.body_text, expected, "Response body text mismatch");
        self
    }

    /// Returns the JSON body for custom assertions
    pub fn json(&self) -> &Value {
        self.body_json
            .as_ref()
            .expect("Response body is not valid JSON")
    }

    /// Returns the text body for custom assertions
    pub fn text(&self) -> &str {
        &self.body_text
    }

    /// Returns the raw response body bytes for custom parsing
    pub fn body_bytes(&self) -> &[u8] {
        &self.body_bytes
    }

    /// Asserts that the response can be deserialized as the expected type and returns it
    /// Use this when you want to assert the response structure matches a specific type
    pub fn assert_deserializes_to<T>(&self) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_slice(&self.body_bytes)
            .expect("Response body should deserialize to the expected type")
    }
}
