# RabbitMQ

Docs updated: May 2023

RabbitMQ ports:

* AMQP: 5672, 5671 TLS
* RabbitMQ Stream protocol: 5552, 5551 TLS
* MQTT: 1883, 8883 TLS
* STOMP: 61613, 61614 TLS
* Management UI: 15672, 15671 TLS
* Prometheus metrics: 15692, 15691 TLS















# Tutorials

RabbitMQ is a broker for binary messages.
*Producers* post message into a *queue*, *consumers* waits to receive the message.



## Tutorial: Task Queue

We'll use [pika](https://github.com/pika/pika), but see also [kombu](https://github.com/celery/kombu).

Let's send a message to the queue.
Establish a connection:

```python
import pika

# Establish a connection
# AMQP can multiplex:
#  * 0.9.1 will use multiple "channels" over a single connection.
#  * 1.0 calls them "sessions"
connection = pika.BlockingConnection(
    pika.ConnectionParameters(
        'localhost', 5672,
        credentials=pika.PlainCredentials('u', 'u'),
    ),
)
```

Now send a message to the default exchange of the queue:

```python
# Send a message
with connection.channel() as channel:
    # RabbitMQ will drop the message if a queue does not exist.
    # Create the queue
    channel.queue_declare(
        queue='tasks',
        # Persist the queue to disk. Otherwise it's lost when RabbitMQ restarts.
        durable=True
    )

    # Messages cannot be posted to queues directly: they need to go through an *exchange*.
    # Publish a message, using the default exchange: ''
    channel.basic_publish(
        # Default exchange
        exchange='',
        routing_key='tasks',   # queue name
        body='Hello world!'
    )
```

Use this command to see the list of queues:

```console
# rabbitmqctl list_queues
name    messages
hello   1
```

To receive a message, you subscribe a callback to a queue:

```python
# Receive the message
with connection.channel() as channel:
    # Ensure the queue exists
    # Idempotent: safe to run every time
    channel.queue_declare(queue='tasks', durable=True)

    # Subscribe a callback to a queue
    def on_message(ch, method, properties, body: bytes):
        print(f"{method.routing_key!r}: {body=}")

    channel.basic_consume(
        queue='tasks',
        # Turn off manual message acknowledgements
        auto_ack=True,
        on_message_callback=on_message
    )

    # Serve forever
    channel.start_consuming()

```

By default, RabbitMQ uses round-robin dispatch: it will send each message to the next consumer, in sequence.
To be more fair with heavy tasks, we can use `basic_qos()`:

```python
# Receive the message
with connection.channel() as channel:
    # Fair QoS: don't give more than one message to this worker at a time
    # Otherwise it's Rounb Robin, even if we're still busy
    channel.basic_qos(prefetch_count=1)

    ...
```

Messages must be ACKed: that is, tell RabbitMQ that the consumer didn't lose it.
By default, there's a 30 minute timeout: if not ACKed, it will be re-sent:

```python
    def on_message(ch, method, properties, body: bytes):
        # ... do long processing ....

        # Ack the message: show that we didn't lose it.
        # RabbitMQ can delete it now.4
        ch.basic_ack(delivery_tag = method.delivery_tag)

    # Subscribe a callback to a queue
    channel.basic_consume(
        queue='tasks',
        on_message_callback=on_message
    )
```

See the list of un-ACKed messages:

```console
# rabbitmqctl list_queues name messages_ready messages_unacknowledged
name    messages_ready  messages_unacknowledged
tasks   0               1
```

To make sure that a queue is not lost when RabbitMQ restarts, make it durable:

```python
# Define a durable queue. Idempotent call: safe to do multiple times.
# NOTE: RabbitMQ will not let you re-define a queue with different parameters!
# Define a new queue if you need.
channel.queue_declare(queue='tasks', durable=True)

# and now send messages as "persistent":

channel.basic_publish(
    exchange='',
    routing_key='tasks',  # queue name
    body=message,
    properties=pika.BasicProperties(
        # NOTE: persistence guarantees aren't strong: RabbitMQ does not fsync every message
        delivery_mode = pika.spec.PERSISTENT_DELIVERY_MODE,
    )
)
```

NOTE: If all the workers are busy, your queue can fill up.
You will want to keep an eye on that, and maybe add more workers, or use message TTL.


## Tutorial: Pub/Sub

In the work queue, a task is delivered to exactly one worker.
In pub/sub, a message is delivered to multiple consumers.

The messaging model:

* A *producer* sends messages
* A *consumer* receives messages
* A *queue* is a buffer that stores messages

A producer never sends any messages directly to the queue:
instead, the producer can only send messages to an *exchange*: it pushes messages to queues.

Available exchange types:

* "direct": use "routing_key" to pass the message to a queue by name
* "fanout": broadcast to all bound queues
* "topic": use `logs.<service>.<severity>` hierarchical names and subscribe like `logs.cron.*`
* "headers": ?

So here's what we do:

* We create an exchange
* Every time a consumer connects, they create a temporary queue, and bind it to the exchange

If no queue is bound to an exchange, the message will be lost: this way we only receive new messages.

Here's how you can see the list of exchanges and bindings:

```console
root@70df1f60e639:/# rabbitmqctl list_exchanges
name                    type
amq.rabbitmq.trace      topic
amq.direct              direct
amq.match               headers
amq.headers             headers
amq.fanout              fanout
amq.topic               topic
                        direct
logs                    fanout

root@70df1f60e639:/# rabbitmqctl list_bindings
source_name     source_kind     destination_name    destination_kind        routing_key      arguments
                exchange        tasks               queue                   tasks            []
logs            exchange        amq.gen-nhgyxf7     queue                   amq.gen-nhgyxf7  []
```


## Tutorial: Routing (direct)

Goal: subscribe only to a subset of the messages.
We will use a `routing_key` when binding an exchange to a queue.

First, declare a "direct" exchange:

```python
channel.exchange_declare(exchange='direct_logs', exchange_type='direct')
```

now bind your client queue to this exchange and start getting messages:

```python
res = channel.queue_declare(queue='', exclusive=True)
queue_name = res.method.queue

channel.queue_bind(
    exchange='direct_logs',
    queue=queue_name,
    # This is a "bind key". Its meaning depends on the exchange type:
    # "fanout" ignores it: mindless broadcasting
    # "direct" understands it as the name of the queue
    routing_key='black'
)
```

Don't forget to specify 'black' when publishing a message:

```python
channel.basic_publish(
    exchange='direct_logs',
    routing_key=severity,
    body=message
)
```

## Tutorial: Routing (topic)

The "direct" exchange has a limitation: it cannot do routing based on multiple criteria.
E.g. you might want to get all logs from "kern", and only critical errors from "cron".

We'll use the "topic" exchange. Its routing key is hierarchical:

```
logs.<service>.<severity>
```

Wildcards:

* `*` substitutes exactly one word
* `#` substitutes zero or more words

Example wildcards:

```
*.orange.*
*.*.rabbit
lazy.#
```

When no wildcards are used, "topic" exchange works exactly like "direct".

If a message does not match any of the bindings, it's lost.

Producer: create an exchange and publish a message with a topic:

```python
with connection.channel() as channel:
    # Declare an exchange: "topic"
    channel.exchange_declare(exchange='topic_logs', exchange_type='topic')

    # Publish
    # Routing key: topic
    channel.basic_publish(
        exchange='topic_logs',
        routing_key='logs.cron.critical',
        body='notification'
    )
```

Consumer: create a temp queue, bind to the exchange:

```python
# Receive messages
with connection.channel() as channel:
    # Fresh temp queue, random name
    res = channel.queue_declare(queue='', exclusive=True)
    queue_name = res.method.queue

    # Bind
    channel.queue_bind(exchange='topic_logs', queue=queue_name, routing_key='logs.kern.*')
    channel.queue_bind(exchange='topic_logs', queue=queue_name, routing_key='logs.cron.critical')

    # Receive
    def on_message(ch, method, properties, body: bytes):
        print(f"{method.routing_key!r}: {body=}")
    channel.basic_consume(queue=queue_name, auto_ack=True, on_message_callback=on_message)
    channel.start_consuming()

```

If you want message from multiple topics, it's perfectly fine to create multiple bindings!


## Tutorial: RPC

RPC pattern: run a function on a remote machine and wait for the result.

Word of advice:

* Make sure it's obvious which function call is local and which is remote!
* Document your system, make the dependencies between components clear
* Handle error cases. How should the client react when the RPC server is down for a long time?

When in doubt avoid RPC. If you can, you should use an asynchronous pipeline - instead of RPC-like blocking, results are asynchronously pushed to a next computation stage.

RPC is easy: send a message, specify a `reply_to` queue and wait to a result:


```python
result = channel.queue_declare(queue='', exclusive=True)
callback_queue = result.method.queue

channel.basic_publish(
    exchange='',
    routing_key='rpc_queue',
    properties=pika.BasicProperties(
        # The queue to send the response to
        reply_to = callback_queue,
    ),
    body=request
)
```

AMQP properties:

* `delivery_mode`: mark the message as persistent or transient
* `content_type`: MIME-type of the message, e.g. "application/json"
* `reply_to`: callback queue
* `correlation_id`: useful to correlate RPC responses with requests

Creating a callback queue for every RPC request is pretty inefficient.
Let's create a single callback queue per client.

To tell one response from another, we set a unique value to `correlation_id` for every request.

So in the end, this is how a server looks like:

```python
with connection.channel() as channel:
    channel.queue_declare(queue='rpc_queue')

    def on_request(ch, method, props, body):
        ...

        ch.basic_publish(
            # Send the response directly to the response queue
            exchange='',
            routing_key=props.reply_to,
            # Set correlation_id
            properties=pika.BasicProperties(correlation_id=props.correlation_id),
            body=f'Your message length: {len(body)}',
        )
        ch.basic_ack(delivery_tag=method.delivery_tag)

    channel.basic_qos(prefetch_count=1)  # spread the load equally over multiple servers
    channel.basic_consume(queue='rpc_queue', on_message_callback=on_request)
    channel.start_consuming()
```

and a client:

```python
with connection.channel() as channel:
    # Make a queue for responses
    res = channel.queue_declare(queue='', exclusive=True)
    results_queue = res.method.queue

    # Send request
    correlation_id = str(uuid.uuid4())
    channel.basic_publish(
        exchange='',
        routing_key='rpc_queue',
        properties=pika.BasicProperties(
            # Get response here
            reply_to=results_queue,
            correlation_id=correlation_id,
        ),
        body='whatever',
    )

    # Receive the response
    def on_result(ch, method, props, body):
        if props.correlation_id == correlation_id:
            print(f'result: {props.correlation_id=} {body=}')
        else:
            # Ignore messages with wrong ids.
            pass

    channel.basic_consume(queue=results_queue, on_message_callback=on_result, auto_ack=True)

    # Just once
    connection.process_data_events(time_limit=None)
```


## Tutorial: Publisher Confirms

"Publisher Confirms" is a RabbitMQ extension to implement reliable publishing.
When publisher confirms are enabled on a channel, messages the client publishes are confirmed
asynchronously by the broker, meaning they have been taken care of on the server side.

This extension to the AMQP protocol is not enabled by default. It has to be enabled on a channel:

Code in Java:

```java
channel.confirmSelect();
```

After publishing a message:

```java
channel.waitForConfirmsOrDie(5_000);
```

This technique is very straightforward but also has a major drawback: it significantly slows down publishing, as the confirmation of a message blocks the publishing of all subsequent messages. This approach is not going to deliver throughput of more than a few hundreds of published messages per second.

A faster strategy: publish a batch of messages and then wait for the whole batch to be confirmed.
This is ~20-30x times faster. One drawback: we do not know exactly what went wrong in case of failure, so we may have to keep a whole batch in memory to log something meaningful or to re-publish the messages:

```java
int batchSize = 100;
int outstandingMessageCount = 0;
while (thereAreMessagesToPublish()) {
    byte[] body = ...;
    BasicProperties properties = ...;
    channel.basicPublish(exchange, queue, properties, body);

    // When batch size exceeded, wait for confirmations
    outstandingMessageCount++;
    if (outstandingMessageCount == batchSize) {
        // Blocks execution :(
        channel.waitForConfirmsOrDie(5_000);
        outstandingMessageCount = 0;
    }
}
if (outstandingMessageCount > 0) {
    channel.waitForConfirmsOrDie(5_000);
}
```

Another strategy: handle publisher confirms asynchronously.
The client just needs to register a callback to be notified on these confirms:

```java
Channel channel = connection.createChannel();
channel.confirmSelect();

// Two callbacks:
// * for confirmed messages
// * for nack-ed messages
channel.addConfirmListener((sequenceNumber, multiple) -> {
    // code when message is confirmed
}, (sequenceNumber, multiple) -> {
    // code when message is nack-ed
});
```
















































# Definitions: export & import

Export definitions to a file, import definitions from a file:
use `rabbitmqctl` or `rabbitmqadmin`:

```console
$ rabbitmqctl export_definitions /tmp/definitions.json
$ rabbitmqctl import_definitions /tmp/definitions.json

$ rabbitmqadmin export /path/to/definitions.file.json
$ rabbitmqadmin import /path/to/definitions.file.json
```

Import definitions at boot time:

```ini
# Does not require management plugin to be enabled.
load_definitions = /path/to/definitions/file.json
definitions.skip_if_unchanged = true
```

Or this:

```ini
definitions.import_backend = local_filesystem
definitions.local.path = /path/to/definitions/defs.json
definitions.skip_if_unchanged = true
```

Definitions can be imported from a URL (HTTPS only):

```inefficient
# Does not require management plugin to be enabled.
definitions.import_backend = https
definitions.https.url = https://raw.githubusercontent.com/rabbitmq/sample-configs/main/queues/5k-queues.json
definitions.skip_if_unchanged = true

definitions.tls.verify     = verify_peer
definitions.tls.fail_if_no_peer_cert = true
definitions.tls.cacertfile = /path/to/ca_certificate.pem
definitions.tls.certfile   = /path/to/client_certificate.pem
definitions.tls.keyfile    = /path/to/client_key.pem


```

The definitions in the file will not overwrite anything already in the broker.

Example definitions:

```js
{
  "users": [
    {
      "hashing_algorithm": "rabbit_password_hashing_sha256",
      "limits": {},
      "name": "mark",
      "password_hash": "gU9PVFWLT7Z8tsp4D9nksdIRak99s57GSyy4welVHnKbbkCa",
      "tags": ["administrator", "management"]
    },
    //...
  ],
  "vhosts": [
    {
      "limits": [],
      "metadata": { "description": "Default virtual host", "tags": []},
      "name": "/"
    },
    //...
  ],
  "permissions": [
    {"user": "guest", "vhost": "mark", "read": ".*", "write": ".*" "configure": ".*"},
    //...
  ],
  "queues": [
    {
      "arguments": {},
      "auto_delete": true,
      "durable": true,
      "name": "mqtt-subscription-medthings-iot-dispenser-24904381311985673540357172qos1",
      "type": "classic",
      "vhost": "/"
    },
    //...
  ],
  "exchanges": [],
  "bindings": [
    {
      "arguments": {},
      "destination": "mqtt-subscription-38cc12d7161a31qos0",
      "destination_type": "queue",
      "routing_key": "medthings.device.dispenser.from.*",
      "source": "amq.topic",
      "vhost": "/"
    },
    //...
  ],
  "global_parameters": [],
  "parameters": [],
  "policies": [],
  "topic_permissions": [],
}
```

Get API definitions as JSON:
<http://localhost:15672/api/definitions>

from CLI:

```console
$ curl -u {username}:{password} -X GET http://{hostname}:15672/api/definitions

$ curl -u {username}:{password} -H "Content-Type: application/json" -X POST -T definitions.json http://{hostname}:15672/api/definitions
```













# CLI Tools

* `rabbitmqctl`: service management
* `rabbitmqadmin`: optional, additional tool that works over HTTP API
* `rabbitmq-diagnostics`: diagnostics and health checking
* `rabbitmq-plugins`: plugin management: list, enable, disable
* `rabbitmq-queues`: maintenance on queues: e.g. quorum queues
* `rabbitmq-streams`: maintenance on streams
* `rabbitmq-upgrage`: upgrade tasks
* `rabbitmq-collect-env`: collects cluster & environment information, logs

For a CLI tool and a node to be able to communicate they must have the same shared secret called the Erlang cookie.
Every cluster node must have the same cookie.
Usually: `/var/lib/rabbitmq/.erlang.cookie`. It is necessary to place a copy of the cookie file for each user that will be using CLI tools.

The Docker image uses a `RABBITMQ_ERLANG_COOKIE` environment variable to populate the cookie file.

### rabbitmqctl

Server status:

```console
$ rabbitmqctl status
Uptime (seconds): 548
Is under maintenance?: false
RabbitMQ version: 3.11.16

Enabled plugins: ...
Data directory: ...
Loaded config files: /etc/rabbitmq/conf.d/10-defaults.conf

Total memory used: 0.162 gb

Connection count: 0
Queue count: 1
Virtual host count: 1
```

Log level: debug, info, warning, error, critical, none. Default: info.

```console
$ rabbitmqctl set_log_level info
```

Virtual host: a logical group of entities in a multi-tenant system.
Resource permissions are scoped per virtual host: possible to set up isolation.

Add a virtual host:

```console
$ rabbitmqctl add_vhost "test"
$ rabbitmqctl list_vhosts
```


User management:

```console
$ rabbitmqctl list_users
user            tags
username        []
u               [administrator]

$ rabbitmqctl add_user "username" "password"
$ rabbitmqctl authenticate_user "username" "password"
$ rabbitmqctl change_password "username" "new-password"
$ rabbitmqctl set_user_tags "username" administrator
# rabbitmqctl set_permissions "username" '.*' '.*' '.*'

# rabbitmqctl list_permissions
user    configure       write   read
u       .*              .*      .*
# rabbitmqctl set_permissions "username" '.*' '.*' '.*'
# rabbitmqctl set_topic_permissions "username" amq.topic "^{username}-.*" "^{username}-.*"
```

See who's using it:

```console
$ rabbitmqctl list_connections
$ rabbitmqctl list_amqp10_connections
$ rabbitmqctl list_mqtt_connections
$ rabbitmqctl list_stomp_connections
$ rabbitmqctl list_channels
$ rabbitmqctl list_consumers
queue_name              channel_pid                             consumer_tag                    ack_required    prefetch_count  active  arguments
mqtt-subscription-...   <rabbit@rabbitmq.1685052428.7865.0>     amq.ctag-MHW7NRTdXzxKOzNPS-KWTQ true            10              true    []
```

See definitions:

```console
$ rabbitmqctl list_exchanges
$ rabbitmqctl list_queues
$ rabbitmqctl list_bindings
```


### rabbitmq-diagnostics

List alarms:

```console
$ rabbitmq-diagnostics alarms
```

Health-checks:

```console
$ rabbitmq-diagnostics check_alarms
$ rabbitmq-diagnostics check_running
```


















# Monitoring

Enable Prometheus: add it to `enabled_plugins`, or:

```console
$ rabbitmq-plugins enable rabbitmq_prometheus
```

This URL then exports key-value metrics for Prometheus: <http://localhost:15692/metrics>

Or sign into <http://localhost:15672/> to see the metrics in a web UI.
It also lists: connections, exchanges, queues, bindings, etc.

Or see information as JSON: <http://localhost:15672/api/overview>











# Configuration

To find where config files are located:

```console
$ rabbitmq-diagnostics status
Config files
 * /etc/rabbitmq/advanced.config
 * /etc/rabbitmq/rabbitmq.conf
```

Config file format: ini-like `rabbitmq.conf`.
The `advanced.config` uses Erlang-style config: good for deeply-nested structures.

The `rabbitmq.conf` file supports environment variable interpolation:

```ini
default_user = $(SEED_USERNAME)
```

Some important settings:

```ini
# User name to create a new database from scratch
default_user = guest
default_pass = guest

# Auth mechanisms to offer to clients
auth_mechanisms.1 = PLAIN
auth_mechanisms.2 = AMQPLAIN
auth_backends.1 = internal

# Logging
log.console = true
log.file = false
log.console.level = info  # debug, info, warning, error, critical
log.default.level = info

```

Alternatively, some settings can be provided with environment variables:

```env
RABBITMQ_DEFAULT_USER = guest
RABBITMQ_DEFAULT_PASS = guest
```


















# Parameters and Policies

## Parameters

*Parameters*: values shared across all nodes in a cluster that change at run time.

Two types of parameters: vhost-scoped parameters, and global parameters.

Parameters:

```console
$ rabbitmqctl set_parameter "component" "name" '"value"'
$ rabbitmqctl clear_parameter "component" "name"
$ rabbitmqctl list_parameters

$ rabbitmqctl set_global_parameter "name" '"value"'
$ rabbitmqctl clear_global_parameter "name"
$ rabbitmqctl list_global_parameters
```

or HTTP API:

```
PUT /api/parameters/{component_name}/{vhost}/{name}
GET /api/parameters

PUT /api/global-parameters/name
DELETE /api/global-parameters/name
GET /api/global-parameters
```

A parameter is a JSON document. You'll need to quote it.

## Policies

Policies is the recommended way of configuring *optional arguments* for queues, exchanges, and some plugins.
*Optional arguments*, aka "x-arguments", are a map of arbitrary key/value pairs used when a queue is declared.
So, in addition to `durable` and `exclusive`, one can have optional `x-arguments`.

Policies were introduced to eliminate pain-points: it's okay when the client defines parameters,
but changing them required re-deployment and queue re-declaration.

A policy matches queues and exchanges by name (usually a RegExp) and appends its definition to the x-arguments.
When a policy is updated, its effect on matching exchanges and queues is almost immediate.

Policies can be used to configure: federation, alternate exchanges, dead lettering, per-queue TTLs, queue length limit.

Define a policy:

```console
$ rabbitmqctl set_policy federate-me  "^federated\." '{"federation-upstream-set":"all"}' \
    --apply-to exchanges --priority 1
```

or HTTP API:

```js
// PUT /api/policies/%2f/federate-me
{
    // pattern: regexp that matches names
    "apply-to": "exchanges",  // exchanges or queues?
    "pattern": "^federated\.",
    "definition": {"federation-upstream-set":"all"},
    "priority": 1, // the one with the highest priority will take effect.
}
```

or in the Web UI: Admin -> Policies -> Add.




















# Authentication, Authorization, Access Control

Authentication: who the user is. Authorization: what the user is allowed to do.

Every connection has an associated user, and it targets a *virtual host*.

When a node starts and detects that it is uninitialized, it initializes a fresh database
with a virtual host `/` and a default user `guest:guest` granted full access to `/`.
This user is only allowed local connections: all remote connections are refused: only `localhost` loopback.

Two primary ways of authentication:

* username/password
* X.509 certificates

When a user connects, they choose a virtual host to operate in.
Then permissions kick in:

* 'configure': create, delete, alter resources or their behavior
* 'write': inject messages into a resource
* 'read': retrieve messages from a resource

Permissions: three RegExp: configure, write, read — on a per-vhost basis.

Note:

* For convenience, the default exchange is called "amq.default". Not "".
* Use RegExp `^$` to ban the user from all resources. Synonym is empty string: `''`.
* Use RegExp `.*` to allow all resources
* Built-in resources start with `amq.`
* Server-generated names are prefixed with `amq.gen`

Examples:

* `^(amq\.gen.*|amq\.default)$` gives a user access to server-generated names and the default exchange.

Users can have *tags* associated with them. Currently, only Management UI access is controlled by user tags.

## Topic Authorization

Topic exchanges support topic authorization. Topic authorization targets protocols like STOMP and MQTT
which are structured around topics and use topic exchanges under the hood.

Internal authorization back-end supports variable expansion in permission patterns:
`{username}`, `{vhost}` and `{client_id}` (MQTT) are supported.
For example, if `john` is the connected user, `^{username}-.*` is expanded to `^john-.*`.

## Backends

A backend may provide authentication ("authn") and/or authorization ("authz").
Examples:

* HTTP, LDAP: authn and authz
* Source IP range: only authn
* Cache backend: reduces the load

When several authentication backends are used then the first positive result returned
by a backend in the chain is considered to be final.

```ini
auth_backends.1 = internal
auth_backends.2 = ldap
auth_backends.3 = http
auth_backends.4 = amqp
auth_backends.5 = dummy
```

when using third-party plugins, provide a full module name: e.g. "rabbit_auth_backend_http".

Example: use the internal database for authentication, and the source IP range backend for authorization:

```ini
auth_backends.1.authn = internal
# uses module name because this backend is from a 3rd party
auth_backends.1.authz = rabbit_auth_backend_ip_range
```

Example: authorization is always internal; authentication is tried against LDAP first:

```ini
auth_backends.1.authn = ldap
auth_backends.1.authz = internal
auth_backends.2       = internal
```

Troubleshoot authentication and authorization:


```console
$ rabbitmqctl authenticate_user "a-username" "a/password"
$ rabbitmqctl list_permissions --vhost /
user        configure   write       read
user2       .*          .*          .*
guest       .*          .*          .*
temp-user   .*          .*          .*
```

and see server logs.

It is possible to have a `user-id` message property that tells which user published it.
The property has to be set manually, and be equal to the current user:

```java
AMQP.BasicProperties properties = new AMQP.BasicProperties();
properties.setUserId("guest");
channel.basicPublish("amq.fanout", "", properties, "test".getBytes());
```

If the user wants to publish a message on behalf of another, they need to have the "impersonator" tag.














# Firehose Tracer

Sometimes, during development, it's useful to be able to see every message published and delivered.

Turn on:

```console
$ rabbitmqctl trace_on -p /
$ rabbitmqctl trace_off -p /
```

This setting is not persistent.

It will publish messages to `amq.rabbitmq.trace`.
The routing key would be `publish.exchange_name` (on publish) or `deliver.{queuename}` (on deliver).

The `rabbitmq_tracing` plugin builds on top of the tracer and provides a GUI.
Enable it and see the "Admin/Tracing" tag in the GUI.











# Memory and Disk Alarms

## Memory Alarms

By default, when RabbitMQ uses >40% of the available RAM, it raises a memory alarm
and blocks all connections that are publishing messages. Once the memory alarm has cleared, normal service resumes.

Configure: either percent, or megabytes:

```console
$ rabbitmqctl set_vm_memory_high_watermark 0.4
$ rabbitmqctl set_vm_memory_high_watermark absolute "1G"
```

Commands are not persistent. Use config file for persistent configuration:

```ini
vm_memory_high_watermark.relative = 0.4
# or
vm_memory_high_watermark.absolute = 1G
```

Use `vm_memory_high_watermark=0` to deactivate publishing globally.

## Disk Alarms

By default, when free disk space drops below 50Mb, an alarm is triggered and all producers are blocked.

Configure:

```ini
disk_free_limit.absolute = 50M
```

or command to set a value temporarily:

```console
$ rabbitmqctl set_disk_free_limit "50M"
```






# Flow Control

If a client is publishing too quickly for queues to keep up, it's throttled:
it will look to the client like the network bandwidth to the server is lower than it actually is.

In the web UI, this connection will appear to be in the "flow" state.

Other components can be in the "flow" state as well: channels, queues, and other parts, can apply flow control that eventually propagates back to publishing connections.



















# Queues

Queue properties:

* Name: up to 255 characters
* Durable: will survive a broker restart, inc. messages
* Exclusive: used by only one connection and removed when it's closed
* Auto-delete: deleted when the last consumer unsubscribes
* Arguments: for plugins and broker-specific features

Queues can be declared and safely re-declared if its attributes remain the same.
If they are not the same, a "406 PRECONDITION_FAILED error will be raised.
Most optional arguments can be dynamically changed after queue declaration but there are exceptions:
`x-queue-type` and `x-max-priority` cannot be changed.

Optional arguments (called "x-arguments" because of their field name in the AMQP protocol)
is a map of arbitrary key/values:

* Queue type: e.g. "quorum", "stream", or classic
* Message TTL, Queue TTL
* Queue length limit
* Max number of priorities
* Consumer priorities
* ...

Optional arguments can be provided by a policy.
If both a policy and the client provide a value, the client's value takes precedence.
However, *operator policies* may override client's values and enforce a policy.

For numerical values (max queue length, ttl) the lower calue of the two is used.
That is, a policy provides the default value which is also the max allowed value. Clients can't go beyond it.

Three ways to make a queue temporary:

* Exclusive queue: removed when the client disconnects
* Auto-delete: removed when the last consumer quits
* TTL: removed after a timeout




















# Queue Length Limit

Max length of a queue can be set either to a number of messages, or to a number of bytes.
When the maximum size is reached, the default behavior is to drop or dead-letter oldest messages.

If `overflow=reject-publish` or `overflow=reject-publish-dlx` (dead letter), the newest messages will be discarded.
If publisher confirms are enabled, the publisher will be informed of the reject via a "basic.nack" message.

Define max queue length using a policy:

```console
$ rabbitmqctl set_policy my-pol "^one-meg$" '{"max-length-bytes":1048576}' --apply-to queues
$ rabbitmqctl set_policy my-pol "^two-messages$" '{"max-length":2,"overflow":"reject-publish"}' --apply-to queues
```










# TTL: Time-To-Live

Set `message-ttl` (ms) argument with a policy — and any message that has been in the queue for
longer than the configured TTL is *dead*. The server guarantees that such messages will not
be delivered using "basic.deliver" or included into a "basic.get-ok" response.

```console
$ rabbitmqctl set_policy TTL ".*" '{"message-ttl":60000}' --apply-to queues
```

A TTL can be specified on a per-message basis, by setting the `expiration` property when publishing a message.

Notes:
* The original expiry time of a message is preserved if it is requeued.
* Setting the TTL to 0 causes messages to be expired upon reaching a queue unless they can be delivered to a consumer immediately.
* When both a per-queue and a per-message TTL are specified, the lower value between the two will be chosen.

TTL can be set on queue itself: use with "auth-delete" queue property. Makes sense for temporary classic queues.
Streams do not support expiration.

```console
$ rabbitmqctl set_policy expiry ".*" '{"expires":1800000}' --apply-to queues
```















# Lazy Queues

Lazy queues are useful when they have millions of messages:
they would move their contents to disk as early as possible, and only load them in RAM when requested by consumers.
It's slower, but saves memory when consumers are slower than normal.

Tune using a policy:

```console
$ rabbitmqctl set_policy Lazy "^lazy-queue$" '{"queue-mode":"lazy"}' --apply-to queues
$ rabbitmqctl set_policy Lazy "^lazy-queue$" '{"queue-mode":"default"}' --apply-to queues
```

or at the time of declaration:

```python
channel.queue_declare(
    'events',
    arguments={'x-queue-mode', 'lazy'},
)
```

During a conversion from the regular mode to the lazy one, the queue will first page all messages kept in RAM to disk. It won't accept any more messages from publishing channels while that operation is in progress. After the initial pageout is done, the queue will start accepting publishes, acks, and other commands.

Lazy queues are appropriate when keeping node memory usage low is a priority and higher disk I/O and disk utilisation are acceptable.
















# Quorum Queues

Durable, replicated FIFO based on the Raft consensus algorithm.
Quorum: when the majority of replicas agree on the state of the queue and its contents.

Quorum Queues and Streams replace durable mirrored queues, which are now deprecated.

Quorum queues are by design replicated and durable.

(TODO: not read)


















# Dead Letter Exchange
Messages from a queue can be "dead-lettered": replublished to an exchange when:

* The message is NACKed: `basic.reject`, or `basic.nack` + `requeue=false`
* The message expires due to per-message TTL
* The message is dropped because its queue exceeded a length limit

Note that when the queue itself expires, it will not dead letter the messages in it.

DLX is a dead-letter exchange: normal exchange of a usual type.

Configure a DLX using a policy:

```console
$ rabbitmqctl set_policy DLX ".*" '{"dead-letter-exchange":"my-dlx"}' --apply-to queues
```

An  explicit routing key can be specified by adding the key `"dead-letter-routing-key"` to the policy.

It is possible to form a cycle of message dead-lettering.
But when a message go full circle (reaches the same queue twice), it will be dropped if there were no rejections in the entire cycle.

Each dead-lettered message gets an array in the header: `x-death`. It contains an entry for each dead lettering event,
identified by a pair of `{queue, reason}`.
Each entry is a table of:

* `queue`: name of the last queue
* `reason`: reason for dead lettering: `rejected`, `expired`, `maxlen`, `delivery_limit`
* `time`: when it was dead lettered
* `exchange`: name of the last exchange
* `routing-keys`: last routing key
* `count`: how many times this message was dead-lettered in this queue for this reason
* `original-expiration`: the original TTL value (if any)

Three top-level headers are added for the very first dead-lettering event:

* `x-first-death-reason`
* `x-first-death-queue`
* `x-first-death-exchange`















# Priority Queues

Any queue can be turned into a priority queue using client-provided arguments.
Declaration using policies is not supported by design (because policies are dynamic,
whereas it's not possible to change priorities of a queue).

To declare a priority queue, set `x-max-priority`: the max priority a queue can support:

> x-max-priority=10

Now publishers can publish prioritised messages using the `priority` field.
Larger numbers indicate higher priority.
Mesages without a priority implicitly have `priority=0`: lowest.

We recomment using `x-max-priority=10`. Using more priorities will consume more CPU resources
by using more Erlang processes.






# Consumer Priorities
If a consumer has a priority — then messages would go to other consumers when the high-priority consumer blocks.
Use this parameter for `basic_consume()`:

> x-priority: 10














# Exchange to Exchange Bindings

The `queue.bind` method binds a queue to an exchange: message flow from the exchange to the queue.

We have introduced `exchange.bind` which binds one exchange to another.
This allows richer topologies to be created. Just like with queues, multiple bindings can be created.

RabbitMQ will detect and eliminate cycles during message delivery: every queue will receive exactly one copy of that message (!).








# Alternate Exchanges

AE: Alternate Exchange: handle messages that an exchange was unable to route (i.e. because there were no bound queues or no matching bindings).

An AE can be defined by clients or using policies.
Example: on "my-direct" exchange, send unroutable messages to "my-ae".

```console
$ rabbitmqctl set_policy AE "^my-direct$" '{"alternate-exchange":"my-ae"}' --apply-to exchanges
```

Whenever an exchange with a configured AE cannot route a message to any queue, it publishes the message to the specified AE instead. If that AE does not exist then a warning is logged. If an AE cannot route a message, it in turn publishes the message to its AE, if it has one configured. This process continues until:

* until either the message is successfully routed,
* the end of the chain of AEs is reached,
* or an AE is encountered which has already attempted to route the message (loop detection)













































# Streams

Streams are new persistent and replicated data structures: append-only log with non-destructive consumer.
They can be used like a normal queue, or through a dedicated binary protocol:
[rabbitmq_stream](https://github.com/rabbitmq/rabbitmq-server/blob/v3.12.x/deps/rabbitmq_stream/docs/PROTOCOL.adoc),
which is recommended because it provides access to all stream-specific features.

Use cases that streams were designed to cover:

1. Large fan-outs. Many consumers, each with a dedicated queue, is inefficient.
    Streams allow many consumers to get the same messages from the same queue in a non-destructive manner (read cursor).
2. Replay / Time-travelling. All queues have destructive consume behavior: i.e. messages are deleted from a queue
    when a consumer is finished with them. It is not possible to re-read a message.
    Streams allow consumers to attach at any point in the log and read from there.
3. Performance. No queue can compete.
4. Larse logs. Most queues are designed to converge towards the empty state and are optimized as such.
    Streams are designed to store large amounts of data in an efficient manner with minimal in-memory overhead.

As streams persist all data to disks before doing anything it is recommended to use the fastest disks possible.
Due to the disk I/O-heavy nature of streams, their throughput decreases as message sizes increase.

Streams replicate data across multiple nodes and publisher confirms are only issued once the data has been replicated to a quorum of stream replicas.

Enable streams:

```console
$ rabbitmq-plugins enable rabbitmq_stream
```

Management commands:

```console
# rabbitmqctl list_queues name type messages

$ rabbitmqctl list_stream_connections
$ rabbitmqctl list_consumers
$ rabbitmqctl list_consumer_groups
$ rabbitmqctl publishers
```

To create a stream, use `queue_declare()` and provide an `x-queue-type=stream`.
This argument must be provided by a client at declaration time; it cannot be set or changed using a policy.

This will create a stream with a replica on each configured RabbitMQ node.
Streams are quorum systems so uneven cluster sizes is strongly recommended.

```python
with connection.channel() as channel:
    channel: pika.adapters.blocking_connection.BlockingChannel

    # Declare a stream
    # This will create a stream with a replica on each configured RabbitMQ node.
    channel.queue_declare(
        'events',
        durable=True,
        # Make it a stream
        arguments={'x-queue-type': 'stream'},
    )
```

A stream remains an AMQP queue, so it can be bound to any exchange after its creation, just as any other RabbitMQ queue.

Streams support 3 additional queue arguments that are best configured using a policy:

* `x-max-length-bytes`: max stream size in bytes. Default: not set
* `x-max-age`: maximum age of the stream. Default: not set
* `x-stream-max-segment-size-bytes`: on disk, a stream is divided up into segments of this fixed size. Default: 500 000 000 (500 Mb)

## Consuming

As streams never delete any messages, any consumer can start reading/consuming from any point in the log.

Start reading from a certain point: `x-stream-offset` consumer argument.
If not set, will start reading fresh messages only.

Supported values:
* `first`
* `last`: starts reading from the last writen "chunk of messages"
* `next`: only new messages
* `<Offset>` offset: specific offset to read. It not exists, will clamp to either the start or the end of the log
* `<Timestamp>`: the point in time to attach to the log at. It will clamp to the closest offset.
* `<Interval>`: a string value: the time interval relative to current time to attach the log at.

Read messages:

```python
# Read from a stream
with connection.channel() as channel:
    channel: pika.adapters.blocking_connection.BlockingChannel

    # Streams require QoS
    channel.basic_qos(prefetch_count=1)

    # Declare a stream
    # This will create a stream with a replica on each configured RabbitMQ node.
    channel.queue_declare(
        'events',
        durable=True,
        # Make it a stream
        arguments={'x-queue-type': 'stream'},
    )

    # Start reading
    def on_message(ch, method, props, body):
        print(vars())
        # Ack is required
        ch.basic_ack(delivery_tag = method.delivery_tag)

    channel.basic_consume(
        "events", on_message,
        arguments={
            # Start from "first".
            # Alternatively, use offset `5000`, or a datetime
            'x-stream-offset': 'first',
        })
```

## Single Active Consumer

When several consumers are reading from a stream, RabbitMQ will ensure that only one consumer is active
and all others remain idle (backup consumers). Makes sure that messages are processed in order.

(no details available)

## Super Streams

A super stream is a logical stream made of individual regular streams.
Allows to handle large streams by dividing them into partitions, splitting up the storage and the traffic
on several cluster nodes:

```console
$ rabbitmq-streams add_super_stream invoices --partitions 3
```

Super streams add complexity compared to individual streams, so they should not be considered
the default solution for all use cases involving streams. Consider using super streams only if
you are sure you reached the limits of individual streams.

## Retention

Streams are implemented as an immutable append-only disk log: it will grow indefinitely until the disk runs out.
To avoid this , use queue arguments (or a policy):

* `max-age`: use number + YMDhms. Example: "7D" = 7 days
* `max-length-bytes`: the max total size in bytes

The stream is cleaned-up in segments, and one latest segment would always be present.


























# MQTT Plugin

The plugin requires a quorum of cluster nodes: 2 out of 3, or 3 out of 5, etc.
It **does not support** cluster of two nodes!

Supported features:

* QoS0 and QoS1. QoS2 is downgraded to QoS1.
* Last Will and Testament (LWT)
* Session stickiness
* Retained messages with pluggable storage back-ends

Enable:

```console
$ rabbitmq-plugins enable rabbitmq_mqtt
```

Messages published to MQTT topics use a topic exchange (`amq.topic` by default) internally.
Subscribers consume from RabbitMQ queues bound to the topic exchange.

MQTT topics use slashes `cities/london`, but the plugin translates them to `cities.london`.
Limitation: MQTT topics that have dots in them won't work as expected.

By default, MQTT allows `guest` access.
To disable anonymous connections:

```ini
mqtt.allow_anonymous = false
```

MQTT QoS levels:

* QoS0: at most once. No guarantee of delivery. The recipient does not ack receipt.
* QoS1: at least once. Guarantees that a message is delivered at least one time.
* QoS2: exactly once. Makes sure that a message is delievered exactly once.

In RabbitMQ:

* QoS0 uses non-durable, auto-delete queues: will be deleted when the client disconnects
* QoS1 use durable queues. Auto-delete may be set by a client's "clean session" flag:
    clients with clean sessions use auto-deleted queues.

Queues created for MQTT subscribers will ahve names starting with `mqtt-subscription-*`,
one per subscription QoS level. Queues will have a *queue TTL* depending on configuration:
24h by default.

Configuration:

```ini
mqtt.allow_anonymous  = false
mqtt.vhost            = /
mqtt.exchange         = amq.topic
mqtt.subscription_ttl = 86400000  # 24h
mqtt.prefetch         = 10
```

## Vhosts

When a user connects, the plugin extracts vhost from the user.

Another way to specify a vhost is to prepend the vhost to a username:

> /:username

It's also possible to open multiple MQTT ports and configure a port-to-vhost mapping:

```console
$ rabbitmqctl set_global_parameter mqtt_port_to_vhost_mapping \
    '{"1883":"vhost1", "8883":"vhost1", "1884":"vhost2", "8884":"vhost2"}'
```

It's also possible to authenticated using X.509 certificates. See the docs.

## Retained messages

The plugin supports *retained messages* with two pluggable storages:

* ETS: `rabbit_mqtt_retained_msg_store_ets`: in-memory
* DETS: `rabbit_mqtt_retained_msg_store_dets`: on-disk

ETS is limited by RAM, DETS is limited to 2Gb per vhost.

Configuration:

```ini
mqtt.retained_message_store = rabbit_mqtt_retained_msg_store_dets
mqtt.retained_message_store_dets_sync_interval = 2000
```

See better-performing stores based on Riak and Cassandra.

## Web MQTT

Use MQTT over a WebSocket.

```console
$ rabbitmq-plugins enable rabbitmq_web_mqtt
```

