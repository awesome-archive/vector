---
description: Full Vector config specification
---

<!--
     THIS FILE IS AUTOOGENERATED!

     To make changes please edit the template located at:

     scripts/generate/templates/docs/usage/configuration/specification.md.erb
-->

# Config Specification

Below is a full config specification. Note, this file is included with
Vector package installs, generally located at `/etc/vector/vector.spec.yml`:

{% code-tabs %}
{% code-tabs-item title="/etc/vector/vector.spec.toml" %}
```coffeescript
#                                    __   __  __  
#                                    \ \ / / / /
#                                     \ V / / /
#                                      \_/  \/
#
#                                    V E C T O R
#                            Configuration Specification
#
# ------------------------------------------------------------------------------
# Website: https://vector.dev
# Docs: https://docs.vector.dev
# Community: https://vector.dev/community
# ------------------------------------------------------------------------------
# The file contains a full specification for the `vector.toml` configuration
# file. It follows the TOML format and includes all options, types, and
# possible values.
#
# More info on Vector's configuration can be found at:
# https://docs.vector.dev/usage/configuration

# ------------------------------------------------------------------------------
# Global
# ------------------------------------------------------------------------------
# Global options are relevant to Vector as a whole and apply to global behavior.
#
# Documentation: https://docs.vector.dev/usage/configuration
# The directory used for persisting Vector state, such as on-disk buffers, file
  # checkpoints, and more. Please make sure the Vector project has write
  # permissions to this dir.
  # 
  # * optional
  # * no default
  data_dir = "/var/lib/vector"

# ------------------------------------------------------------------------------
# Sources
# ------------------------------------------------------------------------------
# Sources specify data sources and are responsible for ingesting data into
# Vector.
#
# Documentation: https://docs.vector.dev/usage/configuration/sources

[sources.file]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "file"
  type = "file"

  # Array of file patterns to exclude. Globbing is supported. *Takes precedence
  # over the `include` option.*
  # 
  # * required
  # * no default
  exclude = ["/var/log/nginx/access.log"]

  # Array of file patterns to include. Globbing is supported.
  # 
  # * required
  # * no default
  include = ["/var/log/nginx/*.log"]

  # Ignore files with a data modification date that does not exceed this age.
  # 
  # * optional
  # * no default
  # * unit: seconds
  ignore_older = 86400

  # The maximum number of a bytes a line can contain before being discarded. This
  # protects against malformed lines or tailing incorrect files.
  # 
  # * optional
  # * default: 102400
  # * unit: bytes
  max_line_bytes = 102400

  # When `true` Vector will read from the beginning of new files, when `false`
  # Vector will only read new data added to the file.
  # 
  # * optional
  # * default: false
  start_at_beginning = false

  #
  # Context
  #

  # The key name added to each event with the full path of the file.
  # 
  # * optional
  # * default: "file"
  file_key = "file"

  # The key name added to each event representing the current host.
  # 
  # * optional
  # * default: "host"
  host_key = "host"

[sources.statsd]
  # The component type
  # 
  # * required
  # * no default
  # * must be: "statsd"
  type = "statsd"

  # UDP socket address to bind to.
  # 
  # * required
  # * no default
  address = "127.0.0.1:8126"

[sources.stdin]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "stdin"
  type = "stdin"

  # The maxiumum bytes size of a message before it is discarded.
  # 
  # * optional
  # * default: 102400
  # * unit: bytes
  max_length = 102400

  #
  # Context
  #

  # The key name added to each event representing the current host.
  # 
  # * optional
  # * default: "host"
  host_key = "host"

[sources.syslog]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "syslog"
  type = "syslog"

  # The input mode.
  # 
  # * required
  # * no default
  # * enum: "tcp", "udp", "unix"
  mode = "tcp"
  mode = "udp"
  mode = "unix"

  # The TCP or UDP address to listen on.
  # 
  # * optional
  # * no default
  address = "0.0.0.0:9000"

  # The maximum bytes size of incoming messages before they are discarded.
  # 
  # * optional
  # * default: 102400
  # * unit: bytes
  max_length = 102400

  # The unix socket path. *This should be absolute path.* Only relevant when
  # `mode` is `unix`.
  # 
  # * optional
  # * no default
  path = "/path/to/socket"

  #
  # Context
  #

  # The key name added to each event representing the current host.
  # 
  # * optional
  # * default: "host"
  host_key = "host"

[sources.tcp]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "tcp"
  type = "tcp"

  # The address to bind the socket to.
  # 
  # * required
  # * no default
  address = "0.0.0.0:9000"

  # The maximum bytes size of incoming messages before they are discarded.
  # 
  # * optional
  # * default: 102400
  # * unit: bytes
  max_length = 102400

  # The timeout before a connection is forcefully closed during shutdown.
  # 
  # * optional
  # * default: 30
  # * unit: seconds
  shutdown_timeout_secs = 30

  #
  # Context
  #

  # The key name added to each event representing the current host.
  # 
  # * optional
  # * default: "host"
  host_key = "host"

[sources.vector]
  # The component type
  # 
  # * required
  # * no default
  # * must be: "vector"
  type = "vector"

  # The TCP address to bind to.
  # 
  # * required
  # * no default
  address = "0.0.0.0:9000"

  # The timeout before a connection is forcefully closed during shutdown.
  # 
  # * optional
  # * default: 30
  # * unit: seconds
  shutdown_timeout_secs = 30


# ------------------------------------------------------------------------------
# Transforms
# ------------------------------------------------------------------------------
# Transforms parse, structure, and enrich events.
#
# Documentation: https://docs.vector.dev/usage/configuration/transforms

[transforms.add_fields]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "add_fields"
  type = "add_fields"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  #
  # Fields
  #

  [transforms.add_fields.fields]
    # A key/value pair representing the new field to be added. Accepts all
    # supported types. Use `.` for adding nested fields.
    # 
    # * required
    # * no default
    my_string_field = "string value"
    my_env_var_field = "${ENV_VAR}"
    my_int_field = 1
    my_float_field = 1.2
    my_bool_field = true
    my_timestamp_field = 1979-05-27T00:32:00.999998-07:00
    my_nested_fields = {key1 = "value1", key2 = "value2"}
    my_list = ["first", "second", "third"]

[transforms.field_filter]
  # The component type
  # 
  # * required
  # * no default
  # * must be: "field_filter"
  type = "field_filter"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The target field to compare against the `value`.
  # 
  # * required
  # * no default
  field = "file"

  # If the value of the specified `field` matches this value then the event will
  # be permitted, otherwise it is dropped.
  # 
  # * required
  # * no default
  value = "/var/log/nginx.log"

[transforms.grok_parser]
  # The component type
  # 
  # * required
  # * no default
  # * must be: "grok_parser"
  type = "grok_parser"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The Grok pattern
  # 
  # * required
  # * no default
  pattern = "%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"

  # If `true` will drop the `field` after parsing.
  # 
  # * optional
  # * default: true
  drop_field = true

  # The field to execute the `pattern` against. Must be a `string` value.
  # 
  # * optional
  # * default: "message"
  field = "message"

[transforms.json_parser]
  # The component type
  # 
  # * required
  # * no default
  # * must be: "json_parser"
  type = "json_parser"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # If `true` events with invalid JSON will be dropped, otherwise the event will
  # be kept and passed through.
  # 
  # * required
  # * no default
  drop_invalid = true

  # The field decode as JSON. Must be a `string` value.
  # 
  # * optional
  # * default: "message"
  field = "message"

[transforms.log_to_metric]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "log_to_metric"
  type = "log_to_metric"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  #
  # Metrics
  #

  [[transforms.log_to_metric.metrics]]
    # The metric type.
    # 
    # * required
    # * no default
    # * enum: "counter", "gauge"
    type = "counter"
    type = "gauge"

    # The log field to use as the metric.
    # 
    # * required
    # * no default
    field = "duration"

    # If `true` the metric will be incremented by the `field` value. If `false` the
    # metric will be incremented by 1 regardless of the `field` value.
    # 
    # * optional
    # * default: false
    increment_by_value = false

    # The name of the metric. Defaults to `<field>_total` for `counter` and
    # `<field>` for `gauge`.
    # 
    # * optional
    # * default: "dynamic"
    name = "duration_total"

    [transforms.log_to_metric.metrics.labels]
      # Key/value pairs representing the metric labels.
      # 
      # * required
      # * no default
      host = "${HOSTNAME}"
      region = "us-east-1"

[transforms.lua]
  # The component type
  # 
  # * required
  # * no default
  # * must be: "lua"
  type = "lua"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The inline Lua source to evaluate.
  # 
  # * required
  # * no default
  source = """
require("script") # a `script.lua` file must be in your `search_dirs`

if event["host"] == nil then
  local f = io.popen ("/bin/hostname")
  local hostname = f:read("*a") or ""
  f:close()
  hostname = string.gsub(hostname, "\n$", "")
  event["host"] = hostname
end
"""


  # A list of directories search when loading a Lua file via the `require`
  # function.
  # 
  # * optional
  # * no default
  search_dirs = ["/etc/vector/lua"]

[transforms.regex_parser]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "regex_parser"
  type = "regex_parser"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The Regular Expression to apply. Do not inlcude the leading or trailing `/`.
  # 
  # * required
  # * no default
  regex = "^(?P<host>[\\w\\.]+) - (?P<user>[\\w]+) (?P<bytes_in>[\\d]+) \\[(?P<timestamp>.*)\\] \"(?P<method>[\\w]+) (?P<path>.*)\" (?P<status>[\\d]+) (?P<bytes_out>[\\d]+)$"

  # If the `field` should be dropped (removed) after parsing.
  # 
  # * optional
  # * default: true
  drop_field = true

  # The field to parse.
  # 
  # * optional
  # * default: "message"
  field = "message"

  #
  # Types
  #

  [transforms.regex_parser.types]
    # A definition of mapped field types. They key is the field name and the value
    # is the type. `strftime` specifiers are supported for the `timestamp` type.
    # 
    # * required
    # * no default
    # * enum: "string", "int", "float", "bool", "timestamp|strftime"
    status = "int"
    duration = "float"
    success = "bool"
    timestamp = "timestamp|%s"
    timestamp = "timestamp|%+"
    timestamp = "timestamp|%F"
    timestamp = "timestamp|%a %b %e %T %Y"

[transforms.remove_fields]
  # The component type
  # 
  # * required
  # * no default
  # * must be: "remove_fields"
  type = "remove_fields"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The field names to drop.
  # 
  # * required
  # * no default
  fields = ["field1", "field2"]

[transforms.sampler]
  # The component type
  # 
  # * required
  # * no default
  # * must be: "sampler"
  type = "sampler"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The maximum number of events allowed per second.
  # 
  # * required
  # * no default
  rate = ["field1", "field2"]

  # A list of regular expression patterns to exclude events from sampling. If an
  # event's `"message"` key matches _any_ of these patterns it will _not_ be
  # sampled.
  # 
  # * optional
  # * no default
  pass_list = ["[error]", "field2"]

[transforms.tokenizer]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "tokenizer"
  type = "tokenizer"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The field names assigned to the resulting tokens, in order.
  # 
  # * required
  # * no default
  field_names = ["timestamp", "level", "message"]

  # If `true` the `field` will be dropped after parsing.
  # 
  # * optional
  # * default: true
  drop_field = true

  # The field to tokenize.
  # 
  # * optional
  # * default: "message"
  field = "message"

  #
  # Types
  #

  [transforms.tokenizer.types]
    # A definition of mapped field types. They key is the field name and the value
    # is the type. `strftime` specifiers are supported for the `timestamp` type.
    # 
    # * required
    # * no default
    # * enum: "string", "int", "float", "bool", "timestamp|strftime"
    status = "int"
    duration = "float"
    success = "bool"
    timestamp = "timestamp|%s"
    timestamp = "timestamp|%+"
    timestamp = "timestamp|%F"
    timestamp = "timestamp|%a %b %e %T %Y"


# ------------------------------------------------------------------------------
# Sinks
# ------------------------------------------------------------------------------
# Sinks batch or stream data out of Vector.
#
# Documentation: https://docs.vector.dev/usage/configuration/sinks

[sinks.aws_cloudwatch_logs]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "aws_cloudwatch_logs"
  type = "aws_cloudwatch_logs"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The group name of the target CloudWatch Logs stream.
  # 
  # * required
  # * no default
  group_name = "/var/log/my-log.log"

  # The AWS region of the target CloudWatch Logs stream resides.
  # 
  # * required
  # * no default
  region = "us-east-1"

  # The stream name of the target CloudWatch Logs stream.
  # 
  # * required
  # * no default
  stream_name = "my-stream"

  #
  # Batching
  #

  # The maximum size of a batch before it is flushed.
  # 
  # * optional
  # * default: 1049000
  # * unit: bytes
  batch_size = 1049000

  # The maximum age of a batch before it is flushed.
  # 
  # * optional
  # * default: 1
  # * unit: bytes
  batch_timeout = 1

  #
  # Requests
  #

  # The encoding format used to serialize the events before flushing.
  # 
  # * optional
  # * default: "dynamic"
  # * enum: "json", "text"
  encoding = "json"
  encoding = "text"

  # The window used for the `request_rate_limit_num` option
  # 
  # * optional
  # * default: 1
  # * unit: seconds
  rate_limit_duration = 1

  # The maximum number of requests allowed within the `rate_limit_duration`
  # window.
  # 
  # * optional
  # * default: 5
  rate_limit_num = 5

  # The maximum number of in-flight requests allowed at any given time.
  # 
  # * optional
  # * default: 5
  request_in_flight_limit = 5

  # The maximum time a request can take before being aborted.
  # 
  # * optional
  # * default: 30
  # * unit: seconds
  request_timeout_secs = 30

  # The maximum number of retries to make for failed requests.
  # 
  # * optional
  # * default: 5
  retry_attempts = 5

  # The amount of time to wait before attempting a failed request again.
  # 
  # * optional
  # * default: 5
  # * unit: seconds
  retry_backoff_secs = 5

  #
  # Buffer
  #

  [sinks.aws_cloudwatch_logs.buffer]
    # The buffer's type / location. `disk` buffers are persistent and will be
    # retained between restarts.
    # 
    # * optional
    # * default: "memory"
    # * enum: "memory", "disk"
    type = "memory"
    type = "disk"

    # The behavior when the buffer becomes full.
    # 
    # * optional
    # * default: "block"
    # * enum: "block", "drop_newest"
    when_full = "block"
    when_full = "drop_newest"

    # The maximum size of the buffer on the disk.
    # 
    # * optional
    # * no default
    # * unit: bytes
    max_size = 104900000

    # The maximum number of events allowed in the buffer.
    # 
    # * optional
    # * default: 500
    # * unit: events
    num_items = 500

[sinks.aws_kinesis_streams]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "aws_kinesis_streams"
  type = "aws_kinesis_streams"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The AWS region of the target CloudWatch Logs stream resides.
  # 
  # * required
  # * no default
  region = "us-east-1"

  # The stream name of the target CloudWatch Logs stream.
  # 
  # * required
  # * no default
  stream_name = "my-stream"

  #
  # Batching
  #

  # The maximum size of a batch before it is flushed.
  # 
  # * optional
  # * default: 1049000
  # * unit: bytes
  batch_size = 1049000

  # The maximum age of a batch before it is flushed.
  # 
  # * optional
  # * default: 1
  # * unit: bytes
  batch_timeout = 1

  #
  # Requests
  #

  # The encoding format used to serialize the events before flushing.
  # 
  # * optional
  # * default: "dynamic"
  # * enum: "json", "text"
  encoding = "json"
  encoding = "text"

  # The window used for the `request_rate_limit_num` option
  # 
  # * optional
  # * default: 1
  # * unit: seconds
  rate_limit_duration = 1

  # The maximum number of requests allowed within the `rate_limit_duration`
  # window.
  # 
  # * optional
  # * default: 5
  rate_limit_num = 5

  # The maximum number of in-flight requests allowed at any given time.
  # 
  # * optional
  # * default: 5
  request_in_flight_limit = 5

  # The maximum time a request can take before being aborted.
  # 
  # * optional
  # * default: 30
  # * unit: seconds
  request_timeout_secs = 30

  # The maximum number of retries to make for failed requests.
  # 
  # * optional
  # * default: 5
  retry_attempts = 5

  # The amount of time to wait before attempting a failed request again.
  # 
  # * optional
  # * default: 5
  # * unit: seconds
  retry_backoff_secs = 5

  #
  # Buffer
  #

  [sinks.aws_kinesis_streams.buffer]
    # The buffer's type / location. `disk` buffers are persistent and will be
    # retained between restarts.
    # 
    # * optional
    # * default: "memory"
    # * enum: "memory", "disk"
    type = "memory"
    type = "disk"

    # The behavior when the buffer becomes full.
    # 
    # * optional
    # * default: "block"
    # * enum: "block", "drop_newest"
    when_full = "block"
    when_full = "drop_newest"

    # The maximum size of the buffer on the disk.
    # 
    # * optional
    # * no default
    # * unit: bytes
    max_size = 104900000

    # The maximum number of events allowed in the buffer.
    # 
    # * optional
    # * default: 500
    # * unit: events
    num_items = 500

[sinks.aws_s3]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "aws_s3"
  type = "aws_s3"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The S3 bucket name. Do not include a leading `s3://` or a trailing `/`.
  # 
  # * required
  # * no default
  bucket = "my-bucket"

  # The AWS region of the target S3 bucket.
  # 
  # * required
  # * no default
  region = "us-east-1"

  #
  # Requests
  #

  # The encoding format used to serialize the events before flushing.
  # 
  # * required
  # * no default
  # * enum: "ndjson", "text"
  encoding = "ndjson"
  encoding = "text"

  # The compression type to use before writing data.
  # 
  # * optional
  # * no default
  # * must be: "gzip" (if supplied)
  compression = "gzip"

  # Whether to Gzip the content before writing or not. Please note, enabling this
  # has a slight performance cost but significantly reduces bandwidth.
  # 
  # * optional
  # * default: false
  gzip = false

  # The window used for the `request_rate_limit_num` option
  # 
  # * optional
  # * default: 1
  # * unit: seconds
  rate_limit_duration = 1

  # The maximum number of requests allowed within the `rate_limit_duration`
  # window.
  # 
  # * optional
  # * default: 5
  rate_limit_num = 5

  # The maximum number of in-flight requests allowed at any given time.
  # 
  # * optional
  # * default: 5
  request_in_flight_limit = 5

  # The maximum time a request can take before being aborted.
  # 
  # * optional
  # * default: 30
  # * unit: seconds
  request_timeout_secs = 30

  # The maximum number of retries to make for failed requests.
  # 
  # * optional
  # * default: 5
  retry_attempts = 5

  # The amount of time to wait before attempting a failed request again.
  # 
  # * optional
  # * default: 5
  # * unit: seconds
  retry_backoff_secs = 5

  #
  # Batching
  #

  # The maximum size of a batch before it is flushed.
  # 
  # * optional
  # * default: 10490000
  # * unit: bytes
  batch_size = 10490000

  # The maximum age of a batch before it is flushed.
  # 
  # * optional
  # * default: 300
  # * unit: bytes
  batch_timeout = 300

  #
  # Object Names
  #

  # Whether or not to append a UUID v4 token to the end of the file. This ensures
  # there are no name collisions high volume use cases.
  # 
  # * optional
  # * default: true
  filename_append_uuid = true

  # The extension to use in the object name.
  # 
  # * optional
  # * default: "log"
  filename_extension = "log"

  # The format of the resulting object file name. `strftime` specifiers are
  # supported.
  # 
  # * optional
  # * default: "%s"
  filename_time_format = "%s"

  # A prefix to apply to all object key names. This should be used to partition
  # your objects, and it's important to end this value with a `/` if you want
  # this to be the root S3 "folder". `strftime` specifiers are supported.
  # 
  # * optional
  # * default: "date=%F"
  key_prefix = "date=%F/"
  key_prefix = "date=%F/hour=%H/"
  key_prefix = "year=%Y/month=%m/day=%d/"

  #
  # Buffer
  #

  [sinks.aws_s3.buffer]
    # The buffer's type / location. `disk` buffers are persistent and will be
    # retained between restarts.
    # 
    # * optional
    # * default: "memory"
    # * enum: "memory", "disk"
    type = "memory"
    type = "disk"

    # The behavior when the buffer becomes full.
    # 
    # * optional
    # * default: "block"
    # * enum: "block", "drop_newest"
    when_full = "block"
    when_full = "drop_newest"

    # The maximum size of the buffer on the disk.
    # 
    # * optional
    # * no default
    # * unit: bytes
    max_size = 104900000

    # The maximum number of events allowed in the buffer.
    # 
    # * optional
    # * default: 500
    # * unit: events
    num_items = 500

[sinks.blackhole]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "blackhole"
  type = "blackhole"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The number of events that must be received in order to print a summary of
  # activity.
  # 
  # * required
  # * no default
  print_amount = "1000"

  #
  # Buffer
  #

  [sinks.blackhole.buffer]
    # The buffer's type / location. `disk` buffers are persistent and will be
    # retained between restarts.
    # 
    # * optional
    # * default: "memory"
    # * enum: "memory", "disk"
    type = "memory"
    type = "disk"

    # The behavior when the buffer becomes full.
    # 
    # * optional
    # * default: "block"
    # * enum: "block", "drop_newest"
    when_full = "block"
    when_full = "drop_newest"

    # The maximum size of the buffer on the disk.
    # 
    # * optional
    # * no default
    # * unit: bytes
    max_size = 104900000

    # The maximum number of events allowed in the buffer.
    # 
    # * optional
    # * default: 500
    # * unit: events
    num_items = 500

[sinks.console]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "console"
  type = "console"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The standard stream to write to.
  # 
  # * required
  # * no default
  # * enum: "stdout", "stderr"
  target = "stdout"
  target = "stderr"

  # The encoding format used to serialize the events before writing.
  # 
  # * optional
  # * default: "dynamic"
  # * enum: "json", "text"
  encoding = "json"
  encoding = "text"

  #
  # Buffer
  #

  [sinks.console.buffer]
    # The buffer's type / location. `disk` buffers are persistent and will be
    # retained between restarts.
    # 
    # * optional
    # * default: "memory"
    # * enum: "memory", "disk"
    type = "memory"
    type = "disk"

    # The behavior when the buffer becomes full.
    # 
    # * optional
    # * default: "block"
    # * enum: "block", "drop_newest"
    when_full = "block"
    when_full = "drop_newest"

    # The maximum size of the buffer on the disk.
    # 
    # * optional
    # * no default
    # * unit: bytes
    max_size = 104900000

    # The maximum number of events allowed in the buffer.
    # 
    # * optional
    # * default: 500
    # * unit: events
    num_items = 500

[sinks.elasticsearch]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "elasticsearch"
  type = "elasticsearch"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The host of your Elasticsearch cluster. This should be the full URL as shown
  # in the example.
  # 
  # * required
  # * no default
  host = "http://10.24.32.122:9000"

  # The `doc_type` for your index data. This is only relevant for Elasticsearch
  # <= 6.X. If you are using >= 7.0 you do not need to set this option since
  # Elasticsearch has removed it.
  # 
  # * optional
  # * default: "_doc"
  doc_type = "_doc"

  # Index name to write events to. `strftime` specifiers are supported.
  # 
  # * optional
  # * default: "vector-%F"
  index = "vector-%F"

  #
  # Batching
  #

  # The maximum size of a batch before it is flushed.
  # 
  # * optional
  # * default: 10490000
  # * unit: bytes
  batch_size = 10490000

  # The maximum age of a batch before it is flushed.
  # 
  # * optional
  # * default: 1
  # * unit: bytes
  batch_timeout = 1

  #
  # Requests
  #

  # The window used for the `request_rate_limit_num` option
  # 
  # * optional
  # * default: 1
  # * unit: seconds
  rate_limit_duration = 1

  # The maximum number of requests allowed within the `rate_limit_duration`
  # window.
  # 
  # * optional
  # * default: 5
  rate_limit_num = 5

  # The maximum number of in-flight requests allowed at any given time.
  # 
  # * optional
  # * default: 5
  request_in_flight_limit = 5

  # The maximum time a request can take before being aborted.
  # 
  # * optional
  # * default: 60
  # * unit: seconds
  request_timeout_secs = 60

  # The maximum number of retries to make for failed requests.
  # 
  # * optional
  # * default: 5
  retry_attempts = 5

  # The amount of time to wait before attempting a failed request again.
  # 
  # * optional
  # * default: 5
  # * unit: seconds
  retry_backoff_secs = 5

  #
  # Buffer
  #

  [sinks.elasticsearch.buffer]
    # The buffer's type / location. `disk` buffers are persistent and will be
    # retained between restarts.
    # 
    # * optional
    # * default: "memory"
    # * enum: "memory", "disk"
    type = "memory"
    type = "disk"

    # The behavior when the buffer becomes full.
    # 
    # * optional
    # * default: "block"
    # * enum: "block", "drop_newest"
    when_full = "block"
    when_full = "drop_newest"

    # The maximum size of the buffer on the disk.
    # 
    # * optional
    # * no default
    # * unit: bytes
    max_size = 104900000

    # The maximum number of events allowed in the buffer.
    # 
    # * optional
    # * default: 500
    # * unit: events
    num_items = 500

[sinks.http]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "http"
  type = "http"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The encoding format used to serialize the events before flushing.
  # 
  # * required
  # * no default
  # * enum: "ndjson", "text"
  encoding = "ndjson"
  encoding = "text"

  # The full URI to make HTTP requests to. This should include the protocol and
  # host, but can also include the port, path, and any other valid part of a URI.
  # 
  # * required
  # * no default
  uri = "https://10.22.212.22:9000/endpoint"

  # The compression strategy used to compress the payload before sending.
  # 
  # * optional
  # * no default
  # * must be: "gzip" (if supplied)
  compression = "gzip"

  # A URI that Vector can request in order to determine the service health.
  # 
  # * optional
  # * no default
  healthcheck_uri = "https://10.22.212.22:9000/_health"

  #
  # Batching
  #

  # The maximum size of a batch before it is flushed.
  # 
  # * optional
  # * default: 1049000
  # * unit: bytes
  batch_size = 1049000

  # The maximum age of a batch before it is flushed.
  # 
  # * optional
  # * default: 5
  # * unit: bytes
  batch_timeout = 5

  #
  # Requests
  #

  # The window used for the `request_rate_limit_num` option
  # 
  # * optional
  # * default: 1
  # * unit: seconds
  rate_limit_duration = 1

  # The maximum number of requests allowed within the `rate_limit_duration`
  # window.
  # 
  # * optional
  # * default: 10
  rate_limit_num = 10

  # The maximum number of in-flight requests allowed at any given time.
  # 
  # * optional
  # * default: 10
  request_in_flight_limit = 10

  # The maximum time a request can take before being aborted.
  # 
  # * optional
  # * default: 30
  # * unit: seconds
  request_timeout_secs = 30

  # The maximum number of retries to make for failed requests.
  # 
  # * optional
  # * default: 10
  retry_attempts = 10

  # The amount of time to wait before attempting a failed request again.
  # 
  # * optional
  # * default: 10
  # * unit: seconds
  retry_backoff_secs = 10

  #
  # Basic auth
  #

  [sinks.http.basic_auth]
    # The basic authentication password.
    # 
    # * required
    # * no default
    password = "password"

    # The basic authentication user name.
    # 
    # * required
    # * no default
    user = "username"

  #
  # Buffer
  #

  [sinks.http.buffer]
    # The buffer's type / location. `disk` buffers are persistent and will be
    # retained between restarts.
    # 
    # * optional
    # * default: "memory"
    # * enum: "memory", "disk"
    type = "memory"
    type = "disk"

    # The behavior when the buffer becomes full.
    # 
    # * optional
    # * default: "block"
    # * enum: "block", "drop_newest"
    when_full = "block"
    when_full = "drop_newest"

    # The maximum size of the buffer on the disk.
    # 
    # * optional
    # * no default
    # * unit: bytes
    max_size = 104900000

    # The maximum number of events allowed in the buffer.
    # 
    # * optional
    # * default: 500
    # * unit: events
    num_items = 500

  #
  # Headers
  #

  [sinks.http.headers]
    # A custom header to be added to each outgoing HTTP request.
    # 
    # * required
    # * no default
    X-Powered-By = "Vector"

[sinks.kafka]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "kafka"
  type = "kafka"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # A comma-separated list of host and port pairs that are the addresses of the
  # Kafka brokers in a "bootstrap" Kafka cluster that a Kafka client connects to
  # initially to bootstrap itself
  # 
  # * required
  # * no default
  bootstrap_servers = "10.14.22.123:9092,10.14.23.332:9092"

  # The field name to use for the topic key. If unspecified, the key will be
  # randomly generated. If the field does not exist on the event, a blank value
  # will be used.
  # 
  # * required
  # * no default
  key_field = "partition_key"

  # The Kafka topic name to write events to.
  # 
  # * required
  # * no default
  topic = "topic-1234"

  # The encoding format used to serialize the events before flushing.
  # 
  # * optional
  # * default: "dynamic"
  # * enum: "json", "text"
  encoding = "json"
  encoding = "text"

  #
  # Buffer
  #

  [sinks.kafka.buffer]
    # The buffer's type / location. `disk` buffers are persistent and will be
    # retained between restarts.
    # 
    # * optional
    # * default: "memory"
    # * enum: "memory", "disk"
    type = "memory"
    type = "disk"

    # The behavior when the buffer becomes full.
    # 
    # * optional
    # * default: "block"
    # * enum: "block", "drop_newest"
    when_full = "block"
    when_full = "drop_newest"

    # The maximum size of the buffer on the disk.
    # 
    # * optional
    # * no default
    # * unit: bytes
    max_size = 104900000

    # The maximum number of events allowed in the buffer.
    # 
    # * optional
    # * default: 500
    # * unit: events
    num_items = 500

[sinks.prometheus]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "prometheus"
  type = "prometheus"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The address to expose for scraping.
  # 
  # * required
  # * no default
  address = "0.0.0.0:9598"

  #
  # Buffer
  #

  [sinks.prometheus.buffer]
    # The buffer's type / location. `disk` buffers are persistent and will be
    # retained between restarts.
    # 
    # * optional
    # * default: "memory"
    # * enum: "memory", "disk"
    type = "memory"
    type = "disk"

    # The behavior when the buffer becomes full.
    # 
    # * optional
    # * default: "block"
    # * enum: "block", "drop_newest"
    when_full = "block"
    when_full = "drop_newest"

    # The maximum size of the buffer on the disk.
    # 
    # * optional
    # * no default
    # * unit: bytes
    max_size = 104900000

    # The maximum number of events allowed in the buffer.
    # 
    # * optional
    # * default: 500
    # * unit: events
    num_items = 500

[sinks.splunk_hec]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "splunk_hec"
  type = "splunk_hec"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # Your Splunk HEC host.
  # 
  # * required
  # * no default
  host = "my-splunk-host.com"

  # Your Splunk HEC token.
  # 
  # * required
  # * no default
  token = "A94A8FE5CCB19BA61C4C08"

  #
  # Batching
  #

  # The maximum size of a batch before it is flushed.
  # 
  # * optional
  # * default: 1049000
  # * unit: bytes
  batch_size = 1049000

  # The maximum age of a batch before it is flushed.
  # 
  # * optional
  # * default: 1
  # * unit: bytes
  batch_timeout = 1

  #
  # Requests
  #

  # The encoding format used to serialize the events before flushing.
  # 
  # * optional
  # * default: "dynamic"
  # * enum: "ndjson", "text"
  encoding = "ndjson"
  encoding = "text"

  # The window used for the `request_rate_limit_num` option
  # 
  # * optional
  # * default: 1
  # * unit: seconds
  rate_limit_duration = 1

  # The maximum number of requests allowed within the `rate_limit_duration`
  # window.
  # 
  # * optional
  # * default: 10
  rate_limit_num = 10

  # The maximum number of in-flight requests allowed at any given time.
  # 
  # * optional
  # * default: 10
  request_in_flight_limit = 10

  # The maximum time a request can take before being aborted.
  # 
  # * optional
  # * default: 60
  # * unit: seconds
  request_timeout_secs = 60

  # The maximum number of retries to make for failed requests.
  # 
  # * optional
  # * default: 5
  retry_attempts = 5

  # The amount of time to wait before attempting a failed request again.
  # 
  # * optional
  # * default: 5
  # * unit: seconds
  retry_backoff_secs = 5

  #
  # Buffer
  #

  [sinks.splunk_hec.buffer]
    # The buffer's type / location. `disk` buffers are persistent and will be
    # retained between restarts.
    # 
    # * optional
    # * default: "memory"
    # * enum: "memory", "disk"
    type = "memory"
    type = "disk"

    # The behavior when the buffer becomes full.
    # 
    # * optional
    # * default: "block"
    # * enum: "block", "drop_newest"
    when_full = "block"
    when_full = "drop_newest"

    # The maximum size of the buffer on the disk.
    # 
    # * optional
    # * no default
    # * unit: bytes
    max_size = 104900000

    # The maximum number of events allowed in the buffer.
    # 
    # * optional
    # * default: 500
    # * unit: events
    num_items = 500

[sinks.tcp]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "tcp"
  type = "tcp"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The TCP address.
  # 
  # * required
  # * no default
  address = "92.12.333.224:5000"

  #
  # Requests
  #

  # The encoding format used to serialize the events before flushing.
  # 
  # * optional
  # * default: "dynamic"
  # * enum: "json", "text"
  encoding = "json"
  encoding = "text"

  #
  # Buffer
  #

  [sinks.tcp.buffer]
    # The buffer's type / location. `disk` buffers are persistent and will be
    # retained between restarts.
    # 
    # * optional
    # * default: "memory"
    # * enum: "memory", "disk"
    type = "memory"
    type = "disk"

    # The behavior when the buffer becomes full.
    # 
    # * optional
    # * default: "block"
    # * enum: "block", "drop_newest"
    when_full = "block"
    when_full = "drop_newest"

    # The maximum size of the buffer on the disk.
    # 
    # * optional
    # * no default
    # * unit: bytes
    max_size = 104900000

    # The maximum number of events allowed in the buffer.
    # 
    # * optional
    # * default: 500
    # * unit: events
    num_items = 500

[sinks.vector]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "vector"
  type = "vector"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  # The downstream Vector address.
  # 
  # * required
  # * no default
  address = "92.12.333.224:5000"

  #
  # Buffer
  #

  [sinks.vector.buffer]
    # The buffer's type / location. `disk` buffers are persistent and will be
    # retained between restarts.
    # 
    # * optional
    # * default: "memory"
    # * enum: "memory", "disk"
    type = "memory"
    type = "disk"

    # The behavior when the buffer becomes full.
    # 
    # * optional
    # * default: "block"
    # * enum: "block", "drop_newest"
    when_full = "block"
    when_full = "drop_newest"

    # The maximum size of the buffer on the disk.
    # 
    # * optional
    # * no default
    # * unit: bytes
    max_size = 104900000

    # The maximum number of events allowed in the buffer.
    # 
    # * optional
    # * default: 500
    # * unit: events
    num_items = 500
```
{% endcode-tabs-item %}
{% endcode-tabs %}


