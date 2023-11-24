# rust graceful shutdown demo

This folder contains a simple rust function with [CloudWatch Lambda Insight](https://docs.aws.amazon.com/lambda/latest/dg/monitoring-insights.html) enabled. CloudWatch Lambda Insight is
monitoring and troubleshooting solution for serverless application. Its agent is an external extension. Any external
extension will work. We use Lambda Insight extension simply because it is readily available.

*It is recommended to use the latest [Lambda Insights extension](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Lambda-Insights-extension-versions.html)*
```yaml
  Layers:
    # https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Lambda-Insights-extension-versions.html
    - !Sub "arn:aws:lambda:${AWS::Region}:580247275435:layer:LambdaInsightsExtension-Arm64:5" # Add Lambda Insight Extension
  Policies:
    # Add IAM Permission for Lambda Insight Extension
    - CloudWatchLambdaInsightsExecutionRolePolicy
```

In the function, a simple `SIGTERM` signal handler is added. It will be executed when the lambda runtime receives
a `SIGTERM` signal.

```rust
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
```

Use the following AWS SAM CLI commands to build and deploy this demo.

```bash
# https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/building-rust.html#building-rust-prerequisites
sam build --beta-features
sam deploy
```

Take note of the output value of HelloWorldApi. Use curl to invoke the api and trigger the lambda function once.

```bash
curl "replace this with value of RustHelloWorldApi"
```

Waite for several minutes, check the function's log messages in CloudWatch. If you see a log line containing "SIGTERM
received", it works!

```text
2023-11-24T10:24:42.205+08:00	INIT_START Runtime Version: provided:al2.v27 Runtime Version ARN: arn:aws:lambda:us-east-1::runtime:2314d913d88add4107e4119c38e7eff2379525a1b70c242c2fbbd5f44af167a2
2023-11-24T10:24:42.265+08:00	LOGS Name: cloudwatch_lambda_agent State: Subscribed Types: [Platform]
2023-11-24T10:24:42.296+08:00	EXTENSION Name: cloudwatch_lambda_agent State: Ready Events: [INVOKE, SHUTDOWN]
2023-11-24T10:24:42.299+08:00	START RequestId: 18045be4-4e12-4d7f-a24b-356669594272 Version: $LATEST
2023-11-24T10:24:42.355+08:00	END RequestId: 18045be4-4e12-4d7f-a24b-356669594272
2023-11-24T10:24:42.355+08:00	REPORT RequestId: 18045be4-4e12-4d7f-a24b-356669594272 Duration: 56.12 ms Billed Duration: 149 ms Memory Size: 128 MB Max Memory Used: 26 MB Init Duration: 92.32 ms
2023-11-24T10:25:41.020+08:00	START RequestId: bf4b4661-feed-4054-9e47-f007f21399bb Version: $LATEST
2023-11-24T10:25:41.035+08:00	END RequestId: bf4b4661-feed-4054-9e47-f007f21399bb
2023-11-24T10:25:41.035+08:00	REPORT RequestId: bf4b4661-feed-4054-9e47-f007f21399bb Duration: 15.18 ms Billed Duration: 16 ms Memory Size: 128 MB Max Memory Used: 26 MB
2023-11-24T10:25:43.584+08:00	START RequestId: 756e997d-d1f4-40a1-a198-7b5cbd75b8b3 Version: $LATEST
2023-11-24T10:25:43.595+08:00	END RequestId: 756e997d-d1f4-40a1-a198-7b5cbd75b8b3
2023-11-24T10:25:43.595+08:00	REPORT RequestId: 756e997d-d1f4-40a1-a198-7b5cbd75b8b3 Duration: 10.92 ms Billed Duration: 11 ms Memory Size: 128 MB Max Memory Used: 26 MB
2023-11-24T10:32:23.118+08:00	[runtime] SIGTERM received,signal number: 15
2023-11-24T10:32:23.118+08:00	[runtime] Graceful shutdown in progress ...
2023-11-24T10:32:23.118+08:00	[runtime] Graceful shutdown completed 
```

## Tested Runtimes

| language version | Identifier      | Operating system  | Architectures    | Support status |
|------------------|-----------------|-------------------|------------------|----------------|
| rust             | provided.al2023 | Amazon Linux 2023 | arm64<br/>x86_64 | ✅Support       |
| rust             | provided.al2    | Amazon Linux 2    | arm64<br/>x86_64 | ✅Support       |

> **Note**: The [Rust runtime client](https://github.com/awslabs/aws-lambda-rust-runtime)
is an experimental package. It is subject to change and intended only for evaluation purposes.

> **Note**: Make sure your [SAM CLI version](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/install-sam-cli.html) is the latest version,SAM CLI version 1.103.0 or newer is recommended.


## Reference:
- [Building Lambda functions with Rust](https://docs.aws.amazon.com/lambda/latest/dg/lambda-rust.html)
- [AWS SAM Documentation](https://docs.aws.amazon.com/serverless-application-model/)
  - [Building Rust Lambda functions with Cargo Lambda](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/building-rust.html)
- [cargo-lambda](https://www.cargo-lambda.info/)