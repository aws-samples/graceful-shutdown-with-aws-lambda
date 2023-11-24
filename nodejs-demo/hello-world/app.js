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

exports.lambdaHandler = async (event, context) => {
    let response = {
        'statusCode': 200,
        'body': JSON.stringify({
            "message": 'hello nodejs',
            "source ip": event['requestContext']['identity']['sourceIp'],
            "architecture": process.arch,
            "operating system": process.platform,
            "node version ": process.version,
        })
    }

    return response
};
