import pika
import pika.adapters.blocking_connection
import sys
import uuid

connection = pika.BlockingConnection(
    pika.ConnectionParameters(
        'localhost', 5672,
        credentials=pika.PlainCredentials('u', 'u'),
    ),
)


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