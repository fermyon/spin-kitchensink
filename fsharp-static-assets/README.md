# C# Hello World

To build this project yourself:

* You will need the preview .NET WASI SDK from https://github.com/SteveSandersonMS/dotnet-wasi-sdk.
* Because the preview bits are not globally installed, this project uses the environment variable
  `DOTNET_WASI_CHECKOUT_ROOT` to find the .NET WASI SDK.  Set this to the directory where you have
  the SDK repo checked out.  E.g. `DOTNET_WASI_CHECKOUT_ROOT=~/github/dotnet-wasi-sdk dotnet build`

Alternatively, pre-built Wasm modules are included in the `bin` directory.
