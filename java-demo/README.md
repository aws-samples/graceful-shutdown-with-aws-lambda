# Java demo

This folder contains a simple java function with CloudWatch Lambda Insight enabled. CloudWatch Lambda Insight is monitoring and troubleshooting solution for serverless applicaiton. Its agent is an external extension. Any external extension will work. We use Lambda Insight extension simply because it is readily available.

```yaml
    Properties:
      Layers:
        - !Sub "arn:aws:lambda:${AWS::Region}:580247275435:layer:LambdaInsightsExtension:14" # Add Lambda Insight Extension
      Policies:
        - CloudWatchLambdaInsightsExecutionRolePolicy # Add IAM Permission for Lambda Insight Extension
```

In the function, a simple SIGTERM signal handler is added. It will be executed when the lambda runtime receives a SIGTERM signal.

```java
    Runtime.getRuntime().addShutdownHook(new Thread() {
        @Override
        public void run() {
            System.out.println("[runtime] ShutdownHook triggered");

            System.out.println("[runtime] Cleaning up");
            // perform actual clean up work here.
            try {
                Thread.sleep(200);
              } catch (Exception e) {
                System.out.println(e);
              }

            System.out.println("[runtime] exiting");
            System.exit(0);
        }
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

Waite for serveral minutes, check the function's log messages in CloudWatch. If you see a log line containing "ShutdownHook triggered", it works!

```
2021/07/28/[$LATEST]6385a4c95a9546b0a4fb4487ca7ac607 2021-07-28T14:51:53.214000 START RequestId: f1693677-4742-41b0-89c3-434dc5fe884c Version: $LATEST
2021/07/28/[$LATEST]6385a4c95a9546b0a4fb4487ca7ac607 2021-07-28T14:51:53.793000 LOGS    Name: cloudwatch_lambda_agent   State: Subscribed       Types: [platform]
2021/07/28/[$LATEST]6385a4c95a9546b0a4fb4487ca7ac607 2021-07-28T14:51:53.793000 EXTENSION       Name: cloudwatch_lambda_agent   State: Ready    Events: [INVOKE,SHUTDOWN]
2021/07/28/[$LATEST]6385a4c95a9546b0a4fb4487ca7ac607 2021-07-28T14:51:58.802000 END RequestId: f1693677-4742-41b0-89c3-434dc5fe884c
2021/07/28/[$LATEST]6385a4c95a9546b0a4fb4487ca7ac607 2021-07-28T14:51:58.802000 REPORT RequestId: f1693677-4742-41b0-89c3-434dc5fe884c  Duration: 5006.06 ms    Billed Duration: 5007 ms        Memory Size: 512 MB     Max Memory Used: 127 MB Init Duration: 662.16 ms
2021/07/28/[$LATEST]6385a4c95a9546b0a4fb4487ca7ac607 2021-07-28T14:57:53.182000 [runtime] ShutdownHook triggered
2021/07/28/[$LATEST]6385a4c95a9546b0a4fb4487ca7ac607 2021-07-28T14:57:53.182000 [runtime] Cleaning up
2021/07/28/[$LATEST]6385a4c95a9546b0a4fb4487ca7ac607 2021-07-28T14:57:53.383000 [runtime] exiting
```
