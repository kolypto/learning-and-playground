import pika
import sys
import uuid

# Establish a connection
connection = pika.BlockingConnection(
    pika.ConnectionParameters(
        'localhost', 5672,
        credentials=pika.PlainCredentials('u', 'u'),
    ),
)



def main():
    """ CLI interface """
    try:
        match sys.argv[1]:
            case 'client':
                client()
            case 'server':
                server()
            case _:
                print('Unknown argument')
                exit(255)
    except IndexError:
        print("Provide an argument: 'client' or 'server'")
        exit(255)



def client():
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

def server():
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



if __name__ == '__main__':
    main()
