# Node.js demo

This folder contains a simple node.js function with CloudWatch Lambda Insight enabled. CloudWatch Lambda Insight is monitoring and troubleshooting solution for serverless applicaiton. Its agent is an external extension. Any external extension will work. We use Lambda Insight extension simply because it is readily available.

```yaml
    Properties:
      Layers:
        - !Sub "arn:aws:lambda:${AWS::Region}:580247275435:layer:LambdaInsightsExtension:14" # Add Lambda Insight Extension
      Policies:
        - CloudWatchLambdaInsightsExecutionRolePolicy # Add IAM Permission for Lambda Insight Extension
```

In the function, a simple SIGTERM signal handler is added. It will be executed when the lambda runtime receives a SIGTERM signal.

```javascript
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

Take note of the output value of HelloWorldApi. Use curl to invoke the api and trigger the lambda function once.

```bash
curl "replace this with value of HelloWorldApi"
```

Waite for serveral minutes, check the function's log messages in CloudWatch. If you see a log line containing "SIGTERM received", it works!

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
