use std::collections::HashMap;
use std::thread;

use aws_lambda_events::apigw::ApiGatewayProxyRequest;
use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use signal_hook::consts::SIGTERM;
use signal_hook::iterator::Signals;

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    statusCode: i32,
    body: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<Response, Error> {
    // Prepare the response payload
    let mut payload = HashMap::new();
    let source_ip = &*(event.payload.request_context.identity.source_ip.unwrap().to_string());
    payload.insert("message", "hello rust");
    payload.insert("source ip", source_ip);
    payload.insert("architecture", std::env::consts::ARCH);
    payload.insert("operating system", std::env::consts::OS);
    // Prepare the response
    let body_content = json!(payload).to_string();
    let resp = Response {
        statusCode: 200,
        body: body_content,
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    // Handle SIGTERM signal: https://rust-cli.github.io/book/in-depth/signals.html
    let mut signals = Signals::new(&[SIGTERM])?;
    thread::spawn(move || {
        for sig_num in signals.forever() {
            println!("[runtime] SIGTERM received,signal number: {:?}", sig_num);
            println!("[runtime] Graceful shutdown in progress ...");
            println!("[runtime] Graceful shutdown completed");
            std::process::exit(0);
        }
    });

    run(service_fn(function_handler)).await
}