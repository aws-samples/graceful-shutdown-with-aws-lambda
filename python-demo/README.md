# Python demo

> ❗ **WARNING:** Graceful shutdown using SIGTERM is currently only compatible with python 3.7 and 3.12 runtimes. See #2

This folder contains a simple python function with CloudWatch Lambda Insight enabled. CloudWatch Lambda Insight is monitoring and troubleshooting solution for serverless applicaiton. Its agent is an external extension. Any external extension will work. We use Lambda Insight extension simply because it is readily available.

```yaml
    Properties:
      Layers:
        - !Sub "arn:aws:lambda:${AWS::Region}:580247275435:layer:LambdaInsightsExtension:14" # Add Lambda Insight Extension
      Policies:
        - CloudWatchLambdaInsightsExecutionRolePolicy # Add IAM Permission for Lambda Insight Extension
```

In the function, a simple SIGTERM signal handler is added. It will be executed when the lambda runtime receives a SIGTERM signal.

```python
def exit_gracefully(signum, frame): 
    print("[runtime] SIGTERM received")

    print("[runtime] cleaning up")
    # perform actual clean up work here. 
    time.sleep(0.2)

    print("[runtime] exiting")
    sys.exit(0)

signal.signal(signal.SIGTERM, exit_gracefully)

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
2021/07/28/[$LATEST]7b4ab412d2494617934d9cd408d8f8a8 2021-07-28T06:15:05.879000 START RequestId: abdd9973-487b-4293-93e5-ed230703cab0 Version: $LATEST
2021/07/28/[$LATEST]7b4ab412d2494617934d9cd408d8f8a8 2021-07-28T06:15:06.004000 LOGS    Name: cloudwatch_lambda_agent   State: Subscribed       Types: [platform]
2021/07/28/[$LATEST]7b4ab412d2494617934d9cd408d8f8a8 2021-07-28T06:15:06.004000 EXTENSION       Name: cloudwatch_lambda_agent   State: Ready    Events: [INVOKE,SHUTDOWN]
2021/07/28/[$LATEST]7b4ab412d2494617934d9cd408d8f8a8 2021-07-28T06:15:06.073000 END RequestId: abdd9973-487b-4293-93e5-ed230703cab0
2021/07/28/[$LATEST]7b4ab412d2494617934d9cd408d8f8a8 2021-07-28T06:15:06.073000 REPORT RequestId: abdd9973-487b-4293-93e5-ed230703cab0  Duration: 67.61 ms      Billed Duration: 68 ms  Memory Size: 128 MB     Max Memory Used: 64 MB  Init Duration: 201.85 ms
2021/07/28/[$LATEST]7b4ab412d2494617934d9cd408d8f8a8 2021-07-28T06:21:05.739000 [runtime] SIGTERM received
2021/07/28/[$LATEST]7b4ab412d2494617934d9cd408d8f8a8 2021-07-28T06:21:05.739000 [runtime] cleaning up
2021/07/28/[$LATEST]7b4ab412d2494617934d9cd408d8f8a8 2021-07-28T06:21:05.939000 [runtime] exiting
```
