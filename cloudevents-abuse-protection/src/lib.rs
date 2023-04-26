use anyhow::Result;
use cloudevents::{binding::http::*, AttributesReader};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A spin component that can be used to test the event-grid-validation.
/// For more info, see here: https://github.com/cloudevents/spec/blob/v1.0/http-webhook.md#42-validation-response
#[http_component]
fn validation(req: Request) -> Result<Response> {
    if req.method() == http::Method::OPTIONS {
        println!("received validation request");
        let mut origin: &str = "";
        let mut callback: &str = "";
        let rate = "120";
        req.headers().iter().for_each(|(k, v)| {
            if k == "webhook-request-origin" {
                origin = v.to_str().unwrap();
            }
            if k == "webhook-request-callback" {
                callback = v.to_str().unwrap();
            }
        });
        println!("callback: {}", callback);
        let req = http::Request::builder()
            .method("GET")
            .header("webhook-allowed-origin", origin)
            .header("webhook-allowed-rate", rate)
            .uri(callback)
            .body(None);

        let req = req.map_err(|err| {
            eprintln!("failed to build request: {:?}", err);
            anyhow::anyhow!("failed to build request")
        })?;

        println!("making request: {:?}", req);

        let res = spin_sdk::outbound_http::send_request(req);
        let mut res = res.map_err(|err| {
            eprintln!("failed to send request: {:?}", err);
            anyhow::anyhow!("failed to send request")
        })?;

        res.headers_mut()
            .insert("webhook-allowed-origin", origin.try_into()?);
        res.headers_mut()
            .insert("webhook-allowed-rate", rate.try_into()?);
        res.headers_mut()
            .insert("spin-component", "rust-outbound-http".try_into()?);
        Ok(res)
    } else {
        println!("received event");
        let msg = req.body().as_ref();
        if let Some(msg) = msg {
            let event = to_event(req.headers(), msg.to_vec())?;
            println!("event source: {}", event.source());
            println!("event id: {}", event.id());
            println!("event type: {}", event.ty());
            Ok(http::Response::builder()
                .status(200)
                .body(req.body().to_owned())?)
        } else {
            Ok(http::Response::builder()
                .status(400)
                .body(Some("cannot parse to cloudevents".into()))?)
        }
    }
}
