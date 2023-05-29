# RabbitMQ

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
* "topic"
* "headers"

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

