# lambda-graceful-shutdown-demo

## Graceful shutdown with Lambda Extension

AWS Lambda allows developers to run their code without managing servers, automatic scaling and pay for value. Many developers use Lambda to connect with databases and Redis. But when a Lambda execution environment shuts down, the connections remains open and take up backend resources. Databases ususally can close those connections after a idle timeout. Developers want to gracefully clean up these connections during shutdown. This can be achieved by attached any external Lambda extension.

For a function with registered external extensions, Lambda service supports graceful shutdown. When Lambda service is about to shut down the runtime, it sends a Shutdown event to the runtime and then to each registered external extension. The Shutdown event sent to the runtime is a SIGTERM signal. Developers can catch this signal in their lambda functions and clean up database connections.

For a function with external extensions, Lambda reserves up to 300 ms (500 ms for a runtime with an internal extension) for the runtime process to perform a graceful shutdown. Lambda allocates the remainder of the 2,000 ms limit for external extensions to shut down.

If the runtime or an extension does not respond to the Shutdown event within the limit, Lambda ends the process using a SIGKILL signal.

![lambda extension shutdown phase](https://docs.aws.amazon.com/lambda/latest/dg/images/Shutdown-Phase.png)

## The demo

This repo contains a simple function with CloudWatch Lambda Insight enabled. CloudWatch Lambda Insight is monitoring and troubleshooting solution for serverless applicaiton. Its agent is an external extension.

In the function, a simple SIGTERM signal handler is added. It will be executed when the lambda runtime receives a SIGTERM signal.

```javascript
// SIGTERM Handler 
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
sam build 
sam deploy --guided 
```

Take note of the output value of HelloWorldApi. Use curl to invoke the api and trigger the lambda function once.

```bash
curl "replace this with value of HelloWorldApi"
```

Waite for serveral minutes, check the function's log messages in CloudWatch. You will see logs contains "SIGTERM received". It works!

```
2021/07/27/[$LATEST]0a35efaafbd24ecc9a5f4fad2dd94b49 2021-07-27T03:47:27.635000 START RequestId: 1aac889c-ccaf-4655-9ad1-018e464ab75d Version: $LATEST
2021/07/27/[$LATEST]0a35efaafbd24ecc9a5f4fad2dd94b49 2021-07-27T03:47:27.789000 LOGS    Name: cloudwatch_lambda_agent   State: Subscribed       Types: [platform]
2021/07/27/[$LATEST]0a35efaafbd24ecc9a5f4fad2dd94b49 2021-07-27T03:47:27.789000 EXTENSION       Name: cloudwatch_lambda_agent   State: Ready    Events: [INVOKE,SHUTDOWN]
2021/07/27/[$LATEST]0a35efaafbd24ecc9a5f4fad2dd94b49 2021-07-27T03:47:27.880000 END RequestId: 1aac889c-ccaf-4655-9ad1-018e464ab75d
2021/07/27/[$LATEST]0a35efaafbd24ecc9a5f4fad2dd94b49 2021-07-27T03:47:27.880000 REPORT RequestId: 1aac889c-ccaf-4655-9ad1-018e464ab75d  Duration: 90.73 ms      Billed Duration: 91 ms  Memory Size: 128 MB     Max Memory Used: 81 MB  Init Duration: 232.27 ms
2021/07/27/[$LATEST]0a35efaafbd24ecc9a5f4fad2dd94b49 2021-07-27T03:53:27.681000 2021-07-27T03:53:27.661Z        1aac889c-ccaf-4655-9ad1-018e464ab75d    INFO    [runtime] SIGTERM received
2021/07/27/[$LATEST]0a35efaafbd24ecc9a5f4fad2dd94b49 2021-07-27T03:53:27.681000 2021-07-27T03:53:27.681Z        1aac889c-ccaf-4655-9ad1-018e464ab75d    INFO    [runtime] cleaning up
2021/07/27/[$LATEST]0a35efaafbd24ecc9a5f4fad2dd94b49 2021-07-27T03:53:27.882000 2021-07-27T03:53:27.882Z        1aac889c-ccaf-4655-9ad1-018e464ab75d    INFO    [runtime] exiting
```
