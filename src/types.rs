use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TlsConfig {
    pub certs: Value,
    pub trusted_certs: Value,
    pub enabled: bool,
    pub loose: bool,
    pub trust_all: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Backend {
    pub id: String,
    pub hostname: String,
    pub port: u32,
    pub tls: bool,
    pub weight: u32,
    pub protocol: String,
    pub ip_address: Option<String>,
    pub predicate: Value,
    pub tls_config: Option<TlsConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Apikey {
    #[serde(alias = "clientId")]
    pub client_id: String,
    #[serde(alias = "clientName")]
    pub client_name: String,
    pub metadata: HashMap<String, String>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub profile: Value,
    pub metadata: HashMap<String, String>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawRequest {
    pub id: u32,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub cookies: Value,
    pub tls: bool,
    pub uri: String,
    pub path: String,
    pub query: HashMap<String, Vec<String>>,
    pub version: String,
    pub has_body: bool,
    pub remote: String,
    pub client_cert_chain: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Frontend {
    pub domains: Vec<String>,
    pub strict_path: Option<String>,
    pub exact: bool,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub methods: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthCheck {
    pub enabled: bool,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub retries: Number,
    pub max_errors: Number,
    pub retry_initial_delay: Number,
    pub backoff_factor: Number,
    pub call_timeout: Number,
    pub call_and_stream_timeout: Number,
    pub connection_timeout: Number,
    pub idle_timeout: Number,
    pub global_timeout: Number,
    pub sample_interval: Number,
    pub proxy: Value,
    pub custom_timeouts: Value,
    pub cache_connection_settings: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RouteBackend {
    pub targets: Vec<Backend>,
    pub root: String,
    pub rewrite: bool,
    pub load_balancing: Value,
    pub client: Client,
    pub health_check: Option<HealthCheck>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub tenant: String,
    pub teams: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
    pub _loc: Location,
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub enabled: bool,
    pub debug_flow: bool,
    pub export_reporting: bool,
    pub capture: bool,
    pub groups: Vec<String>,
    pub frontend: Frontend,
    pub backend: RouteBackend,
    pub backend_ref: Option<String>,
    pub plugins: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtoroshiResponse {
    pub status: u32,
    pub headers: HashMap<String, String>,
    pub cookies: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: String,
    #[serde(alias = "maxAge")]
    pub max_age: Option<Number>,
    pub secure: bool,
    #[serde(alias = "httpOnly")]
    pub http_only: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtoroshiRequest {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, Vec<String>>,
    pub version: String,
    pub client_cert_chain: Value,
    pub backend: Option<Backend>,
    pub cookies: Vec<Cookie>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmBackendContext {
    pub snowflake: Option<String>,
    pub backend: Backend,
    pub apikey: Option<Apikey>,
    pub user: Option<User>,
    pub raw_request: RawRequest,
    pub config: Config,
    pub global_config: Value,
    pub attrs: Value,
    pub route: Route,
    pub request_body_bytes: Option<Vec<u8>>,
    pub request: OtoroshiRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    kind: String,
    path: String,
    opts: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizationRights {
    pub read: bool,
    pub write: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Authorizations {
    #[serde(alias = "httpAccess")]
    pub http_access: bool,
    #[serde(alias = "proxyHttpCallTimeout")]
    pub proxy_http_call_timeout: Number,
    #[serde(alias = "globalDataStoreAccess")]
    pub global_data_store_access: AuthorizationRights,
    #[serde(alias = "pluginDataStoreAccess")]
    pub plugin_data_store_access: AuthorizationRights,
    #[serde(alias = "globalMapAccess")]
    pub global_map_access: AuthorizationRights,
    #[serde(alias = "pluginMapAccess")]
    pub plugin_map_access: AuthorizationRights,
    #[serde(alias = "proxyStateAccess")]
    pub proxy_state_access: bool,
    #[serde(alias = "configurationAccess")]
    pub configuration_access: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KillOptions {
    #[serde(alias = "immortal")]
    pub immortal: bool,
    pub max_calls: Number,
    pub max_memory_usage: Number,
    pub max_avg_call_duration: Number,
    pub max_unused_duration: Number,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub source: Source,
    #[serde(alias = "memoryPages")]
    pub memory_pages: Number,
    #[serde(alias = "functionName")]
    pub function_name: String,
    pub config: Value,
    #[serde(alias = "allowedHosts")]
    pub allowed_hosts: Vec<String>,
    #[serde(alias = "allowedPaths")]
    pub allowed_paths: Value,
    pub wasi: bool,
    pub opa: bool,
    pub authorizations: Authorizations,
    pub instances: Number,
    #[serde(alias = "killOptions")]
    pub kill_options: KillOptions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmAccessValidatorContext {
    pub snowflake: Option<String>,
    pub apikey: Option<Apikey>,
    pub user: Option<User>,
    pub request: RawRequest,
    pub config: Config,
    pub global_config: Value,
    pub attrs: Value,
    pub route: Route,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmRequestTransformerContext {
    pub snowflake: String,
    pub raw_request: OtoroshiRequest,
    pub otoroshi_request: OtoroshiRequest,
    pub apikey: Option<Apikey>,
    pub user: Option<User>,
    pub request: RawRequest,
    pub config: Value,
    pub global_config: Value,
    pub attrs: Value,
    pub route: Route,
    pub request_body_bytes: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmResponseTransformerContext {
    pub snowflake: Option<String>,
    pub raw_response: OtoroshiResponse,
    pub otoroshi_response: OtoroshiResponse,
    pub apikey: Option<Apikey>,
    pub user: Option<User>,
    pub request: RawRequest,
    pub config: Config,
    pub global_config: Value,
    pub attrs: Value,
    pub route: Route,
    pub response_body_bytes: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmSinkContext {
    pub snowflake: Option<String>,
    pub request: RawRequest,
    pub config: Value,
    pub attrs: Value,
    pub origin: String,
    pub status: u32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtoroshiPluginResponse {
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmBackendResponse {
    pub headers: Option<HashMap<String, String>>,
    pub body_bytes: Option<Vec<u8>>,
    pub body_base64: Option<String>,
    pub body_json: Option<Value>,
    pub body_str: Option<String>,
    pub status: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmAccessValidatorError {
    pub message: String,
    pub status: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmAccessValidatorResponse {
    pub result: bool,
    pub error: Option<WasmAccessValidatorError>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmTransformerResponse {
    pub url: Option<String>,
    pub method: Option<String>,
    pub headers: HashMap<String, String>,
    pub cookies: Value,
    pub body_bytes: Option<Vec<u8>>,
    pub body_base64: Option<String>,
    pub body_json: Option<Value>,
    pub body_str: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmSinkMatchesResponse {
    pub result: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmSinkHandleResponse {
    pub status: u32,
    pub headers: HashMap<String, String>,
    pub body_bytes: Option<Vec<u8>>,
    pub body_base64: Option<String>,
    pub body_json: Option<Value>,
    pub body_str: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmJobContext {
    pub attrs: Value,
    pub global_config: Value,
    pub snowflake: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmJobResult {}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmMatchRouteContext {
    pub snowflake: Option<String>,
    pub route: Route,
    pub request: RawRequest,
    pub config: Value,
    pub attrs: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmMatchRouteResponse {
    pub result: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmPreRouteContext {
    pub snowflake: Option<String>,
    pub route: Route,
    pub request: RawRequest,
    pub config: Value,
    pub global_config: Value,
    pub attrs: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmPreRouteResponse {
    pub status: u32,
    pub error: Option<bool>,
    pub headers: HashMap<String, String>,
    pub body_bytes: Option<Vec<u8>>,
    pub body_base64: Option<String>,
    pub body_json: Option<Value>,
    pub body_str: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmRequestHandlerContext {
    pub request: RawRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmRequestHandlerResponse {
    pub status: u32,
    pub headers: HashMap<String, String>,
    pub body_bytes: Option<Vec<u8>>,
    pub body_base64: Option<String>,
    pub body_json: Option<Value>,
    pub body_str: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct WasmResponse {
    pub error: Option<bool>,
    pub status: Option<u32>,
    pub headers: HashMap<String, String>,
    pub cookies: Value,
    pub body_bytes: Option<Vec<u8>>,
    pub body_base64: Option<String>,
    pub body_json: Option<Value>,
    pub body_str: Option<String>,
}
