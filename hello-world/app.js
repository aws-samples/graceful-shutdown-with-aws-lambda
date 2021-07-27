
// SIGTERM Handler 
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
                message: 'hello world',
            })
        }

    return response
};
