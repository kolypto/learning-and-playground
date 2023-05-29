import pika


# Goal: implement pub/sub.
# Unlike the task queue, the message is delivered to all consumers.


# Establish a connection
connection = pika.BlockingConnection(
    pika.ConnectionParameters(
        'localhost', 5672,
        credentials=pika.PlainCredentials('u', 'u'),
    ),
)



# Send a message
with connection.channel() as channel:
    # Create an exchange.
    # Fanout:
    channel.exchange_declare(exchange='logs', exchange_type='fanout')

    # Publish a message
    channel.basic_publish(
        exchange='logs',
        routing_key='',  # ignored for fanout exchanges
        body='notification'
    )


# Receive messages
with connection.channel() as channel:
    # Create a fresh, empty queue every time we connect.
    res = channel.queue_declare(
        # Let the server choose a random name for us.
        queue='',
        # Delete it once the consumer connection is closed
        exclusive=True,
    )
    queue_name = res.method.queue
    print(f'Queue name: {queue_name}')  #-> amq.gen-bf8UOHzOz1DgZmqh2Sf41g

    # Now bind the queue to the exchange:
    # tell the exchange to send us messages.
    channel.queue_bind(exchange='logs', queue=queue_name)

    # Receive
    def on_message(ch, method, properties, body: bytes):
        print(f"{method.routing_key!r}: {body=}")
    channel.basic_consume(queue=queue_name, auto_ack=True, on_message_callback=on_message)
    channel.start_consuming()
