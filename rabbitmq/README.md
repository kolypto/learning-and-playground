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
* "topic"
* "headers"
* "fanout"

So here's what we do:

* We create an exchange
* Every time a consumer connects, they create a temporary queue, and bind it to the exchange

If no queue is bound to an exchange, the message will be lost: this way we only receive new messages.

```python

```

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
