import json
import signal
import sys
import time

def exit_gracefully(signum, frame): 
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
            "message": "hello world",
        }),
    }
