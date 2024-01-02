package main

import (
	"context"
	"encoding/json"
	"fmt"
	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"os"
	"os/signal"
	"runtime"
	"syscall"
)

// Static initialization
// SIGTERM Handler: https://docs.aws.amazon.com/lambda/latest/operatorguide/static-initialization.html
func init() {
	// Create a chan to receive os signal
	var c = make(chan os.Signal)
	// Listening for os signals that can be handled,reference: https://docs.aws.amazon.com/lambda/latest/dg/runtimes-extensions-api.html
	// Termination Signals: https://www.gnu.org/software/libc/manual/html_node/Termination-Signals.html
	signal.Notify(c, syscall.SIGTERM, syscall.SIGINT, syscall.SIGQUIT, syscall.SIGHUP)
	// do something when os signal received
	go func() {
		for s := range c {
			switch s {
			// if lambda runtime received SIGTERM signal,perform actual clean up work here.
			case syscall.SIGTERM:
				fmt.Println("[runtime] SIGTERM received")
				fmt.Println("[runtime] Graceful shutdown in progress ...")
				fmt.Println("[runtime] Graceful shutdown completed")
				os.Exit(0)
				// else if lambda runtime received other signal
			default:
				fmt.Println("[runtime] Other signal received")
				fmt.Println("[runtime] Graceful shutdown in progress ...")
				fmt.Println("[runtime] Graceful shutdown completed")
				os.Exit(0)
			}
		}
	}()
}

// https://docs.aws.amazon.com/lambda/latest/dg/golang-handler.html
func handler(ctx context.Context, request events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {
	bodyContent := map[string]string{
		"message":          "hello golang",
		"source ip":        request.RequestContext.Identity.SourceIP,
		"architecture":     runtime.GOARCH,
		"operating system": runtime.GOOS,
		"go version":       runtime.Version(),
	}

	s, _ := json.Marshal(bodyContent)
	greeting := fmt.Sprintf(string(s))

	return events.APIGatewayProxyResponse{
		Body:       greeting,
		StatusCode: 200,
	}, nil
}

func main() {
	lambda.Start(handler)
}
