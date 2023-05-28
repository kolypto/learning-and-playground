import pika
import sys

# Goal:
# Publish a mesage.
# Receive the message.
# Implement it as a task queue.



# Establish a connection
connection = pika.BlockingConnection(
    pika.ConnectionParameters(
        'localhost', 5672,
        credentials=pika.PlainCredentials('u', 'u'),
    ),
)



# Send a message
with connection.channel() as channel:
    # RabbitMQ will drop the message if a queue does not exist.
    # Create the queue
    channel.queue_declare(queue='tasks')

    # Messages cannot be posted to queues directly: they need to go through an *exchange*.
    # Publish a message, using the default exchange: ''
    channel.basic_publish(
        # Default exchange
        exchange='',
        routing_key='tasks',  # queue name
        body='Hello world!',
        properties=pika.BasicProperties(
            # NOTE: persistence guarantees aren't strong: RabbitMQ does not fsync every message
            delivery_mode=pika.spec.PERSISTENT_DELIVERY_MODE,
        )
    )




# Receive the message
with connection.channel() as channel:
    # Fair QoS: don't give more than one message to this worker at a time
    # Otherwise it's Rounb Robin, even if we're still busy
    channel.basic_qos(prefetch_count=1)

    # Ensure the queue exists
    # Idempotent: safe to run every time
    channel.queue_declare(queue='tasks')

    # Message handler
    def on_message(ch: pika.adapters.blocking_connection.BlockingChannel, method: pika.spec.Basic.Return, properties: pika.BasicProperties, body: bytes):
        print(f"{method.routing_key!r}: {body=}")

        # ... do long processing ....

        # Ack the message: show that we didn't lose it.
        # RabbitMQ can delete it now.4
        ch.basic_ack(delivery_tag=method.delivery_tag)

    # Subscribe a callback to a queue
    channel.basic_consume(
        queue='tasks',
        # auto_ack=True,  # Turn off manual message acknowledgements
        on_message_callback=on_message
    )

    # Serve forever
    try:
        channel.start_consuming()
    except KeyboardInterrupt:
        sys.exit(0)
