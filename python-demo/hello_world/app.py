import json
import platform
import signal
import sys
import time


def exit_gracefully(signum, frame):
    r"""
    SIGTERM Handler: https://docs.aws.amazon.com/lambda/latest/operatorguide/static-initialization.html
    Listening for os signals that can be handled,reference: https://docs.aws.amazon.com/lambda/latest/dg/runtimes-extensions-api.html
    Termination Signals: https://www.gnu.org/software/libc/manual/html_node/Termination-Signals.html
    """
    print("[runtime] SIGTERM received")

    print("[runtime] cleaning up")
    # perform actual clean up work here.
    time.sleep(0.2)

    print("[runtime] exiting")
    sys.exit(0)


signal.signal(signal.SIGTERM, exit_gracefully)


def lambda_handler(event, context):
    return {
        "statusCode": 200,
        "body": json.dumps({
            "message": "hello python3",
            "source ip": event['requestContext']['identity']['sourceIp'],
            "architecture": platform.machine(),
            "operating system": platform.system(),
            "node version ": platform.python_version(),
        }),
    }
