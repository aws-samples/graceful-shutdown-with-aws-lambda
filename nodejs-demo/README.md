# Node.js demo

This folder contains a simple node.js function with CloudWatch Lambda Insight enabled. CloudWatch Lambda Insight is monitoring and troubleshooting solution for serverless applicaiton. Its agent is an external extension. Any external extension will work. We use Lambda Insight extension simply because it is readily available.

*It is recommended to use the
latest [Lambda Insights extension](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Lambda-Insights-extension-versions.html)*
```yaml
    Properties:
      Layers:
        # Add Lambda Insight Extension: https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Lambda-Insights-extension-versions.html
        - !Sub "arn:aws:lambda:${AWS::Region}:580247275435:layer:LambdaInsightsExtension-Arm64:5"
      Policies:
        # Add IAM Permission for Lambda Insight Extension
        - CloudWatchLambdaInsightsExecutionRolePolicy
```

In the function, a simple `SIGTERM` signal handler is added. It will be executed when the lambda runtime receives a `SIGTERM` signal.

```javascript
// Static initialization
// SIGTERM Handler: https://docs.aws.amazon.com/lambda/latest/operatorguide/static-initialization.html
// Listening for os signals that can be handled,reference: https://docs.aws.amazon.com/lambda/latest/dg/runtimes-extensions-api.html
// Termination Signals: https://www.gnu.org/software/libc/manual/html_node/Termination-Signals.html
process.on('SIGTERM', async () => {
    console.info('[runtime] SIGTERM received');

    console.info('[runtime] cleaning up');
    // perform actual clean up work here. 
    await new Promise(resolve => setTimeout(resolve, 200));

    console.info('[runtime] exiting');
    process.exit(0)
});
```

Use the following AWS SAM CLI commands to build and deploy this demo.

```bash
sam build --use-container
sam deploy --guided 
```

Take note of the output value of HelloWorldApi. Use curl to invoke the api and trigger the lambda function at least once.

```bash
curl "replace this with value of HelloWorldApi"
```

Waite for serveral minutes, check the function's log messages in CloudWatch. If you see a log line containing "SIGTERM received", it works!


for example:
![](./docs/images/nodejs-2024-01-08.png)
```text
2023-12-15T14:03:59.046+08:00	INIT_START Runtime Version: provided:al2023.v10 Runtime Version ARN: arn:aws:lambda:us-east-1::runtime:389fcaae1b213b40d38ed791dfb615af1a71a32d6996ff7c4afdde3d5af4b6f2
2023-12-15T14:03:59.104+08:00	LOGS Name: cloudwatch_lambda_agent State: Subscribed Types: [Platform]
2023-12-15T14:03:59.173+08:00	EXTENSION Name: cloudwatch_lambda_agent State: Ready Events: [INVOKE, SHUTDOWN]
2023-12-15T14:03:59.175+08:00	START RequestId: 90d93089-6e10-45f2-81f2-3a640945976b Version: $LATEST
2023-12-15T14:03:59.230+08:00	END RequestId: 90d93089-6e10-45f2-81f2-3a640945976b
2023-12-15T14:03:59.230+08:00	REPORT RequestId: 90d93089-6e10-45f2-81f2-3a640945976b Duration: 55.01 ms Billed Duration: 183 ms Memory Size: 128 MB Max Memory Used: 31 MB Init Duration: 127.75 ms
2023-12-15T14:04:07.275+08:00	START RequestId: 89827818-1fdf-4626-a0eb-4cb509171c29 Version: $LATEST
2023-12-15T14:04:07.330+08:00	END RequestId: 89827818-1fdf-4626-a0eb-4cb509171c29
2023-12-15T14:04:07.330+08:00	REPORT RequestId: 89827818-1fdf-4626-a0eb-4cb509171c29 Duration: 55.80 ms Billed Duration: 56 ms Memory Size: 128 MB Max Memory Used: 31 MB
2023-12-15T14:09:35.620+08:00	[runtime] SIGTERM received
2023-12-15T14:09:35.620+08:00	[runtime] Graceful shutdown in progress ...
2023-12-15T14:09:35.620+08:00	[runtime] Graceful shutdown completed 
```

## Tested Runtimes

| language version | Identifier | Operating system  | Architectures    | Support status |
|------------------|------------|-------------------|------------------|----------------|
| Node.js 20       | nodejs20.x | Amazon Linux 2023 | arm64<br/>x86_64 | ✅Support       |
| Node.js 18       | nodejs18.x | Amazon Linux 2    | arm64<br/>x86_64 | ✅Support       |

## Reference:

- [Building Lambda functions with nodejs](https://docs.aws.amazon.com/lambda/latest/dg/lambda-nodejs.html)
- [AWS SAM Documentation](https://docs.aws.amazon.com/serverless-application-model/)
