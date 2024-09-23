# frozen_string_literal: true

require "json"

# SIGTERM Handler: https://docs.aws.amazon.com/lambda/latest/operatorguide/static-initialization.html
# Listening for OS signals that can be handled, reference: https://docs.aws.amazon.com/lambda/latest/dg/runtimes-extensions-api.html
# Termination Signals: https://www.gnu.org/software/libc/manual/html_node/Termination-Signals.html
def exit_gracefully
  puts "[runtime] SIGTERM received"
  puts "[runtime] cleaning up"

  # perform actual clean up work here.
  sleep 0.2

  puts "[runtime] exiting"
end

Signal.trap("TERM") do |signal|
  Thread.new { exit_gracefully }.join
end

def lambda_handler(event:, context:)
  {
    statusCode: 200,
    body: {
      "message" => "hello ruby",
      "source ip" => event["requestContext"]["http"]["sourceIp"],
      "architecture" => RbConfig::CONFIG["host_cpu"],
      "operating system" => RbConfig::CONFIG["host_os"],
      "ruby version " => RUBY_VERSION
    }.to_json
  }
end
