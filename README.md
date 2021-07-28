# Graceful shutdown with Lambda Extension

AWS Lambda allows developers to run their code without managing servers, automatic scaling and pay for value. Many developers use Lambda to connect with databases and Redis. But when a Lambda execution environment shuts down, the connections remain open and hold up backend resources. Databases ususally can clean those connections after an idle timeout. However, developers want to gracefully clean up those connections when Lambda execution environment shuts down. Now, this can be achieved with Lambda Extensions.

For a function with registered external extensions, Lambda service supports graceful shutdown. When Lambda service is about to shut down the runtime, it sends a Shutdown event to the runtime and then to each registered external extension. The Shutdown event sent to the runtime is a SIGTERM signal. Developers can catch this signal in their lambda functions and clean up database connections.

For a function with external extensions, Lambda reserves up to 300 ms (500 ms for a runtime with an internal extension) for the runtime process to perform a graceful shutdown. Lambda allocates the remainder of the 2,000 ms limit for external extensions to shut down.

If the runtime or an extension does not respond to the Shutdown event within the limit, Lambda ends the process using a SIGKILL signal.

![lambda extension shutdown phase](https://docs.aws.amazon.com/lambda/latest/dg/images/Shutdown-Phase.png)


## Demos

This repo includes two examples. Please see the details in each demo folder. 

[Node.js Demo](nodejs-demo/)

[Python Demo](python-demo/)

