# Spin example components

This repository contains a list of Spin HTTP components in various
programming languages.
To explore a component, read the accompanying section in this readme,
and the corresponding component in `spin.toml`, and the code directory
associated with the component.

pre-requisites:
1. [tinygo](https://tinygo.org/getting-started/install/)
2. [zig](https://ziglang.org/learn/getting-started/#installing-zig)

To build and run:

```
$ make build serve
```

## `rust-hello`: A simple "hello world" Spin component in Rust

This is the simplest Spin component you can write in Rust — a single function that
takes an HTTP request and returns an HTTP response.

## `rust-static-assets`: A Rust component that reads static assets

This is an example of how to mount use assets with a Spin component, and how
to build a Rust component that opens and returns a static asset.

This is just an example. For a more complete implementation of a file server as
a Spin component, check out [the Spin static file server that the Spin documentation
website uses](https://github.com/fermyon/spin-fileserver).

This showcases how to mount static assets into the component:

```toml
[[component]]
id = "rust-static-assets"
source = "rust-static-assets/target/wasm32-wasi/release/rust_static_assets.wasm"
# map all files from the `static/` directory into the WebAssembly module, at `/`.
files = [{ source = "static-assets/", destination = "/" }]
[component.trigger]
# this component will be invoked for requests on any route starting with `/static/`
route = "/static/..."
```

## `rust-outbound-http`: A Rust component that sends an HTTP request

This is an example of a Rust component that sends sends an HTTP request. A
requirement for allowing a component so send outbound HTTP requests is adding
the domain of the request to the list of `allowed_http_hosts`:

```toml
[[component]]
id = "rust-outbound-http"
source = "rust-outbound-http/target/wasm32-wasi/release/rust_outbound_http.wasm"
# set the service URL as an environment variable pointing to the previous component
environment = { SERVICE_URL = "http://localhost:3000/static/important.txt" }
# add the domain of the service URL to the list of allowed hosts, so the component 
# is allowed to send a request to it
allowed_http_hosts = [ "http://localhost:3000" ]
[component.trigger]
route = "/outbound"
```

This example sends the request to the `rust-static-assets` component, so this
also showcases how to use multiple Spin components.
Finally, the full URL this component should send the request to is passed as an
environment variable, showing how to set and retrieve them from Rust.

```rust
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
```

When a request to `/outbound` is received, Spin will instantiate and invoke the
entry point of the `rust-outbound-http` component, which sends an HTTP request to
`/static/important.txt` — Spin will instantiate the `rust-static-assets`
component, send the response to the `rust-outbound-http` component, which will
add a custom header, then send the response back to the client.

## `go-hello`: A simple "hello world" Spin component written in Go

This is the simplest Spin component you can write in Go — a `main` function that
calls the Spin handler which takes the Go standard HTTP request and response
writer as parameters:

```go
func main() {
 spin.HandleRequest(func(w http.ResponseWriter, r *http.Request) {
  fmt.Fprintln(w, "Hello, Fermyon from a Spin component written in Go!!")
 })
}
```

## `go-static-assets`: A Go component that reads static assets

This is an example of building a Spin component in Go that opens and returns
static assets.

The component will attempt to open and read a file based on the request path.
Notice that the code to open and read a file uses the Go standard library:

```go
func main() {
 spin.HandleRequest(func(w http.ResponseWriter, r *http.Request) {
  path := os.Getenv(pathInfoEnv)
  buf, err := os.Open(path)
  if err != nil {
   fmt.Fprintf(os.Stderr, "Cannot read file %v: %v", path, err)
  }

  if _, err = io.Copy(w, buf); err != nil {
   fmt.Fprintf(os.Stderr, "Error writing file %s: %s\n", path, err)
  }
 })
}
```

## `go-outbound-http`: A Go component that sends an HTTP request

This is an example of a Go component that sends an HTTP request.
A requirement for allowing a component so send outbound HTTP requests is adding
the domain of the request to the list of `allowed_http_hosts`:

```toml
[[component]]
id = "go-outbound-http"
source = "go-outbound-http/main.wasm"
environment = { SERVICE_URL = "http://localhost:3000/go-static/important.txt" }
allowed_http_hosts = [ "http://localhost:3000" ]
[component.trigger]
route = "/go-outbound"
# Go components use the Wagi HTTP executor
executor = { type = "wagi" }
```

This example sends the request to the `go-static-assets  component, so
this also showcases how to use multiple Spin components:

```go
const serviceURLEnv = "SERVICE_URL"

func main() {
 spin.HandleRequest(func(w http.ResponseWriter, r *http.Request) {
  url := os.Getenv(serviceURLEnv)
  resp, err := spin.Get(url)
  if err != nil {
   fmt.Fprintf(os.Stderr, "Cannot send HTTP request to %v: %v", url, err)
   send404(w)
  }

  fmt.Fprintln(w, resp.Body)
 })
}
```

## `python-hello`: An experimental Python component

This is a very early experiment for running Python as a Spin component.
It is based on the work done in <https://github.com/fermyon/wagi-python> and
<https://github.com/singlestore-labs/python-wasi>:

```python
import cgi

print('Content-Type: text/plain; charset=UTF-8')
print('Status: 200')
print()

print('Hello, from Python!')

params = cgi.parse()
print(params)
```

Sending a request to the Python component, we can see the result of the script
above being executed:

```
$ curl -i 'localhost:3000/python-hello?abc=def'
HTTP/1.1 200 OK
content-type: text/plain; charset=UTF-8
content-length: 37
date: Wed, 30 Mar 2022 13:44:24 GMT

Hello, from Python!
{'abc': ['def']}
```
