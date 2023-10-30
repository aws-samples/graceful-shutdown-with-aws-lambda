using Amazon.Lambda.APIGatewayEvents;
using Xunit;
using Amazon.Lambda.Core;
using HelloWorldFunction;
using NSubstitute;

namespace HelloWorld.Tests;

public class FunctionTest
{
    [Fact]
    public void TestGreeting()
    {
        var function = new Function();

        var request = new APIGatewayProxyRequest
        {
            Body = "Hello World",
            HttpMethod = "POST",
        };
        
        var response = function.FunctionHandler(request, Substitute.For<ILambdaContext>());

        Assert.Equal("HELLO WORLD", response);
       
    }
}