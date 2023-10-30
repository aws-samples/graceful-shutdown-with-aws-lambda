using System.Runtime.InteropServices;
using System.Text.Json;
using System.Text.Json.Serialization;
using Amazon.Lambda.APIGatewayEvents;
using Amazon.Lambda.Core;

// Assembly attribute to enable the Lambda function's JSON input to be converted into a .NET class.
[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]

namespace HelloWorldFunction;

public class Function
{
    /// <summary>
    /// A simple function that takes a string and does a ToUpper
    /// </summary>
    /// <param name="input"></param>
    /// <param name="context"></param>
    /// <returns></returns>
    public string FunctionHandler(string input, ILambdaContext context)
    {
        var request = JsonSerializer.Deserialize<APIGatewayProxyRequest>(input);
        
        PosixSignalRegistration.Create(PosixSignal.SIGTERM, Handler);
        
        return request?.Body.ToUpper();
    }

    private void Handler(PosixSignalContext obj)
    {
       Console.WriteLine("[runtime] SIGTERM received stopping the application");
       Console.WriteLine("[runtime] Cleaning up");

       try
       {
           Task.Delay(200);
       }
       catch (Exception ex)
       {
           Console.WriteLine(ex);
       }
       
       Console.WriteLine("[runtime] Exiting");
    }
}