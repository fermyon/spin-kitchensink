use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

// The environment variable set in `spin.toml` that points to the
// URL that the component will send a request to.
//
// Note that the domain of the URL must also be present in the `allowed_http_hosts`
// list for the component to be allowed to connect to the host.
const SERVICE_URL_ENV: &str = "SERVICE_URL";

/// Send an HTTP request and return the response.
#[http_component]
fn send_outbound(_req: Request) -> Result<Response> {
    let service_url = std::env::var(SERVICE_URL_ENV)?;

    let mut res = spin_sdk::outbound_http::send_request(
        http::Request::builder()
            .method("GET")
            .uri(service_url)
            .body(None)?,
    )?;

    res.headers_mut()
        .insert("spin-component", "rust-outbound-http".try_into()?);

    Ok(res)
}
