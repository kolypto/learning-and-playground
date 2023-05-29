import pika


# Establish a connection
connection = pika.BlockingConnection(
    pika.ConnectionParameters(
        'localhost', 5672,
        credentials=pika.PlainCredentials('u', 'u'),
    ),
)



# Send a message
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
