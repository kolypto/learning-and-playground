# NATS

Read:
* <https://docs.nats.io/>
* <https://natsbyexample.com/> — a ton of examples in Go, CLI, Python, etc

Docs date: 2025-05

NATS is a pub/sub with at most once delivery:
fan in / fan out, request and reply.

Has persistent queues (JetStream) with exactly once delivery:
work queues, data replication, K/V storage and object storage.

Built-in MQTT server (QoS 1).
Supports WebSockets, a Kafka bridge, a Redis Connector, Telegraf, HTTP, and more.

Supports request/reply pattern:
unlike Kafka, where you'd have to correlate responses with request ids.

NATS infrastructure and clients communicate all topology changes in real-time:
when backend servers are added or removed, NATS clients need not reconnect.
Any client can connect to any node.

Supports memory and file persistence. Messages can be replayed by time, count, or sequence number,
and durable subscriptions are supported. With NATS streaming, scripts can archive old log segments to cold storage.

QoS: Core NATS provides "at most once" delivery guarantee: i.e. if a subscriber is not listening on a subject,
the message is not received. This is the same level of guarantee that TCP/IP provides.
For "at least once" and "exactly once", see JetStream: part of NATS that has durability.

Here's how you init a client in Go:

```go
// app/main.go
func serve() error {
  // NATS Connect
	k, err := nats.Connect(
        strings.Join([]string{
            // Front line servers in the cluster. They will tell the client about other ones.
            "nats://app:verysecret@127.0.0.1:4222/",
            // Alternatively, you can use a token (nats://token@127.0.0.1/)
            // or an NKEY (ed25519 key file) or an OAuth JWT token
        }, ","),
        // Connection name (for monitoring)
        nats.Name("api-server"),
        // Don't deliver my pub messages to me
        nats.NoEcho(),
        // Ping server every <duration>
        nats.PingInterval(10 * time.Second),
        // Timeout for draining a connection
        nats.DrainTimeout(10*time.Second),
        // Reconnect: default wait=2s, timeout=2s
        // Ping interval: 2 minutes (heartbeat)
        // Log connection events
        nats.DisconnectErrHandler(func(_ *nats.Conn, err error) {
            if err != nil {
                log.Error().Err(err).Msg("NATS Disconnected")
            } else {
                log.Info().Msg("NATS Disconnected")
            }
        }),
        nats.ReconnectHandler(func(_ *nats.Conn) {
            log.Info().Msg("NATS Reconnected")
        }),
        nats.ClosedHandler(func(_ *nats.Conn) {
            log.Info().Msg("NATS client closed")
        }),
        nats.DiscoveredServersHandler(func(nc *nats.Conn) {
            log.Info().Strs("known", nc.Servers()).Strs("discovered", nc.DiscoveredServers()).Msg("NATS new servers discovered")
        }),
        nats.ErrorHandler(func(_ *nats.Conn, _ *nats.Subscription, err error) {
            // E.g. slow consumer
            // log, or maybe send to an error channel
            log.Error().Err(err).Msg("NATS Error")
        }),
    )
	if err != nil {
		return errors.Wrap(err, "failed to connect to NATS")
	}

  // Drain the connection, which will close it when done.
  // It lets all handlers finish: unsubscribe, process all cached/inflight messages, clean-up.
  // Drain() can be used instead of Unsubscribe()
  // Do this before quitting.
	// defer k.Close()
  defer k.Drain()


  //... work work work ...//

  return nil
}
```









# Subjects

NATS is an *interest-based* messaging system where listeners subscribe to *subjects*:
specific subject names or wildcards.
Subjects are ephemeral resources, which will disappear when no longer subscribed to.

*Subject hierarchies* are used to scope messages into semantic namespaces:

```
time.us.east
orders.online.store123.order171711
```

Naming advice: encode business intent, not technical details. Name business or physical entities.
Use the first tokens to establish a general namespace; use final tokens for identifiers.

NOTE: some data may be encoded in message headers. No need to mention everything in topics.

Wildcards:
* `time.*.east` will match a whole single token. (it can't do prefix/suffix matching!)
* `time.us.>` will match one or more tokens. Can only be used at the end of the subject.

Message subjects are filtered by security rules (allow/deny per user), accounts, transformations, ...

Names are case sensitive and cannot contain whitespace.
Reserved: `.` `*` `>`.
By convention subject names starting with a `$` are reserved for system use.

# Core NATS

Core NATS: PubSub model using subject-based addressing.

A Message has:

* Subject
* Body (bytes)
* Header fields
* (optional) "reply" address field

Request/reply:

* A request is published on a given subject using a reply subject.
* Responders listen on that subject and send responses to the reply subject.
* Reply subject (called "inbox") are unique subjects that are dynamically directed back to the requester
* NATS supports multiple responses: multiple responders can process the request, but only the first one will be utilized and the rest will be discarded

No subscribers notification:
When a request is sent to a subject that has no subscribers, use `no_responder`:
you will immediately receive a reply that has no body, and a 503 status.

```go
m, err := nc.Request("foo", nil, time.Second);
if err == nats.ErrNoResponders {
    //...
}
```

## Slow Consumers
Slow consumers:

* NATS is designed to move messages quickly, and consumers must consider and respond to changing message rates.
* If a client is too slow, the server will close the connection
* Client libraries can buffer some messages to give the application time to catch up — and will look healthy to the server.
* Common patterns:
  * Use request-reply to throttle the sender and prevent overloading the subscriber
  * Use a queue with multiple subscribers splitting the work
  * Persist messages with something like NATS streaming


## Queue Groups

Queue Group: a group of subscribers to a subject where only one subscriber gets the message.
It's a work queue, and it has load balancing. Useful for scaling services.

If a subscriber is registered based on a queue name, it will always receive messages it is subscribed to.
However, if more subscribers are added to the same queue name — they become a queue group,
and only one randomly chosen subscriber of the queue group will consume a message.

No configuration required: to scale up, just spin up more consumers.

So: if clients subscribe using a queue group, the NATS Servers automatically distributes the messages
published on the matching subjects between the members of the queue group.

Queue group names are also hierarchical: `namespace.group.entity.>`.
Some server functionalities like queue permissions can use wildcard matching on them.

With JetStream, a stream can also be used as a queue:
set the retention policy to `WorkQueuePolicy`.

Geo Affinity:
When connecting to a globally distributed NATS super-cluster,
NATS will automatically route messages within the same cluster (unless failover kicks in).




## Publish/Subscribe in Go

Publisher:

```go
// app/publisher.go
func publisherServe(ctx context.Context, k *nats.Conn) error {
	// Give the consumer some time :)
	time.Sleep(100 * time.Millisecond)



	// Publish one message
	if err := k.Publish("updates", []byte("All is Well")); err != nil {
		return errors.Wrap(err, "Publish() failed")
	}



	// Publish, expect a response (using "reply-subject" or "inbox")
	resp, err := k.Request("request.hello", []byte("Hi there"), 1 * time.Second)
	if err != nil {
		return errors.Wrap(err, "Request() failed")
	}
	fmt.Printf("response: %+v\n", resp)//nocommit



	// Send job (to a work queue, but we don't care)
	if err := k.Publish("jobs.task", []byte("do this")); err != nil {
		return errors.Wrap(err, "Publish() failed")
	}


	// Send a PING/PONG to the server to make sure all publishes are written
	if err := k.FlushTimeout(time.Second); err != nil {
		return errors.Wrap(err, "FlushTimeout() failed")
	}


	// Wait
	<-ctx.Done()
	return nil
}
```

Subscriber:

```go
//app/subscription.go
func subscriptionServe(ctx context.Context, k *nats.Conn) error {
	// Async subscription: will invoke the callback
	// It starts 1 goroutine that will call your callback. Running more goroutines is up to you.
	// Message delivery: serial, one at a time.
	async_sub, err := k.Subscribe("updates", func(msg *nats.Msg) {
		fmt.Printf("async msg: %+v\n", msg)
		// if err := msg.Ack(); err != nil {
		// 	log.Error().Err(err).Msg("Ack() failed")
		// }
	})
	if err != nil {
		return errors.Wrap(err, "Subscribe() failed")
	}
	defer async_sub.Unsubscribe()



	// Async chan subscription: will push to a channel
	// It will not start a goroutine: instead, it will push messages to the channel
	updates_chan := make(chan *nats.Msg, 3)
	chan_sub, err := k.ChanSubscribe("updates", updates_chan)
	if err != nil {
		return errors.Wrap(err, "Subscribe() failed")
	}
	defer chan_sub.Unsubscribe()





	// Wait for a message: "Sync Subscribe"
	// Under the hood, it reads from a channel.
	one_sub, err := k.SubscribeSync("request.*")
	if err != nil {
		return errors.Wrap(err, "SubscribeSync() failed")
	}
	// Auto-unsubscribe after N messages.
	// Use case: when only 1 messages is expected
	if err := one_sub.AutoUnsubscribe(1); err != nil {
		return errors.Wrap(err, "AutoUnsubscribe() failed")
	}
	msg, err := one_sub.NextMsg(1 * time.Second)
	if err != nil {
		return errors.Wrap(err, "NextMsg(2) failed")
	}
	// Reply: Incoming messages may have a "reply-to" field: a subject where a reply is expected.
	err = msg.Respond([]byte("nice to see you!"))
	if err != nil {
		return errors.Wrap(err, "Respond() failed")
	}






	// Queue Subscriptions
	// Provide a queue name: the server will load-balance between all members of the queue group.
	// Queue groups in NATS are dynamic and do not require any server configuration.
	// NOTE: it's still not JetStream: just load-balancing in real time!
	queue_sub, err := k.QueueSubscribe("jobs.>", "job_workers", func(msg *nats.Msg) {
		fmt.Printf("job msg: %+v\n", msg)
	})
	if err != nil {
		return errors.Wrap(err, "QueueSubscribe() failed")
	}
	defer queue_sub.Unsubscribe()


	<-ctx.Done()
	return nil
}

```







# CLI

Install the NATS CLI tool:

```console
$ go install github.com/nats-io/natscli/nats@latest
```

Or with Docker:

```console
$ docker run --rm -it -e NATS_URL=admin:admin@nats:4222 --network nats natsio/nats-box server list
```

Nats tool:

```console
$ nats sub <subject>
$ nats pub <subject> <message>
```

`nats` tool has contexts: like Docker contexts, that keeps your servers' credentials.



# HTTP Monitoring

Open: <http://localhost:8222/>

Accounts, connections, etc.









# JetStream

JetStream: a built-in persistence engine that enables messages to be stored and replayed at a later time.

Unlike NATS Core which requires you to have an active subscription to process messages as they happen,
JetStream allows the NATS server to capture messages and replay them to consumers as needed.
It enables fault-tolerant and high-availability configurations.

> Bragging:
> JetStream was created to address the problems identified with streaming technology today - complexity,
> fragility, and a lack of scalability. Some technologies address these better than others,
> but no current streaming technology is truly multi-tenant, horizontally scalable, or supports multiple deployment models.
> No other technology that we are aware of can scale from edge to cloud using the same security context
> while having complete deployment observability for operations.

Essentially, a JetStream is a subscriber inside NATS that
1. subscribes to subjects and stores every message it sees
2. replays these messages to consumers starting from some offset (start, only new ones, specific date/time, etc)

*Streaming* is: temporal decoupling between the publishers and subscribers.
In NATS Core, subscribers only receive the message if they're actively connected.
A *durable subscriber* ("queue") holds the message until a subscriber comes.

However, queues are (in general) are not meant to be used as a mechanism for message replay; streams are.

A *stream* can replay messages on demand:
JetStream provides both the ability to consume messages as they are published (i.e. 'queueing')
as well as the ability to replay messages on demand (i.e. 'streaming').

Streams are different from queues:
* Streams persiste their data
* Queues (queue group) distribute messages as consumers join & leave, but are removed when they all quit

On top of JetStream, NATS provides:
* Key/Value store
* Object store (for binary files)

NOTE: NATS does not intend to compete with the rich feature set of in-memory databases.

## Deciding to Use Streaming and Higher Qualities of Service
With PubSub, the basic aspect is temporal coupling: subscribers need to be up and running
to receive the message when it's published.

JetStream provides temporal decoupling.

Use JetStream when:

* Observability is required
* Delay message processing
* Consume at your own pace
* Recall old messages: historical record
* Consumers and producers may be online at different times (temporal decoupling)
* You need "exactly once" QoS with de-duplication and double-ack

When to use Core NATS:

* Where applications will retry (e.g. request/responses)
* Where only the last message received is important (e.g. GPS location) and they are sent frequently enough.
* Where message TTL is low and the value of data expires quickly
* Where every participant is expected to be online all the time
* Control plane messages

## Replay
JetStream replay policies:

1. All: a complete replay. With two replay policies: "instant"
  (i.e. as fast as the consumer can take) and "original" (at the rate they were published: i.e. with simulated delays)
2. Last message in a stream
3. Last message for each subject (as streams can capture more than one subject)
4. Starting from a specific ssequence number
5. Starting from a specific time

## Retention
JetStream retention policies:
i.e. delete message that are too old, or when the stream gets too large:

*Limits*:

* Max message age
* Max total stream size (in bytes)
* Max number of messages
* Max individual message size

*Discard policy*:
what should happen once the stream has reached one of its limits and a new message is published:

* Discard old: delete the oldest message in the stream to make room for the new messages.
* Discard new: discard the new message (e.g. queue overflow)

Limits always apply,
but the *retention policy* can provide additional discarding methods:

* Limits: retention based on limits only. Provide a replay of messages in the stream, discard messages exceeding the limits (the default)
* Work Queues: messages are removed as they are consumed. No replay. One consumer per subject.
* Interest: like work queue, but allows overlapping subjects, and requires that *all* consumers ACK the message. It will drop messages if no one's consuming (no interest)

With *Work Queues*, each message can be consumed only once:
this is enforced by only allowing one consumer to be created per subject, i.e. consumers' subject filters
must not overlap! That is, each subject captured by the stream can only have one consumer at a time!
So, with Work Queues, the messages will be removed as soon as the Consumer received an Acknowledgement.

With *Interest Policy*, messages will be removed as soon as *all* (!) Consumers of the stream for that subject
have received an Acknowledgement for the message. This policy

Note that limits always apply, even to a work queue.


## Consistency

NATS provides immediate consistency (not eventual consistency).
You can choose the durability of the message storage:

1. Memory storage
2. File storage
3. Replication (1 (none), 2, 3) between nats servers for Fault Tolerance.

Consistency: NATS is serializable, but doesn't guarantee ["read your writes"](https://jepsen.io/consistency/models/read-your-writes)
because a read may land on a follower. Send get requests to the stream leader for more consistent results.

> *Read your writes*: requires that if a process performs a write `w`,
> then that same process performs a subsequent read `r`, then `r` must observe `w`’s effects.

Replication factor:

* use memory and R=1 if losing messages is ok.
* use file-based R=3 for good resilience: can tolerate the loss of one server servicing the stream
* use R=5 for high resilience — but it will cost you performance.
* R=2 and R=5 — no significant benefit

QoS: *at least once*.
Normally, NATS is reliable and duplicate-free, but there are some specific failure scenarios that could result in duplicates:
* in a publishing application believing (wrongly) that a message was not published successfully and therefore publishing it again
* in a client application's consumption acknowledgment getting lost and therefore in the message being re-sent to the consumer by the server.

QoS: JetStream supports *exactly once*,
but this involves:
* publisher: assigning unique ids to messages (header: `Nats-Msg-Id`)
* server: de-duplicating ids for a configurable rolling period of time
* consumer: double acknowledgment mechanism

Example for de-duplication, with NATS's default windows of 2m:
we send 2 messages with exactly the same `Nats-Msg-Id`. Nats will detect that these are duplicates:

```console
$ nats req -H Nats-Msg-Id:1 ORDERS.new hello1
$ nats req -H Nats-Msg-Id:1 ORDERS.new hello2
$ nats stream info ORDERS
State:
            Messages: 1
               Bytes: 67 B
```

## Mirroring
Stream mirroring:
you can mirror a stream to between different domains in order to offer disaster recovery.
Mirroring is suitable for geographic distribution over high latency and unreliable connections.
E.g. even a leaf node starting and connecting intermittently every few days will still receive or send messages over the source/mirror link.

More info: <https://docs.nats.io/nats-concepts/jetstream/source_and_mirror>


## Streams
Streams consume normal NATS subjects.
You can publish to a subject — and stream will store it. But you won't get an ack.
Use JetStream publish calls instead to get an ack that it was stored.

Stream configuration: [see whole list](https://docs.nats.io/nats-concepts/jetstream/streams)
* Name: plain name, no `.*>`.
* Description. For humans.
* Storage: file, memory
* Subjects: a list of subjects to bind. Default: subject with name = stream name.
* Replicas: how many
* DuplicateWindow: ns time for deduplication
* Limits: MaxAge, MaxBytes, MaxMsgs, MaxMsgsPerSubject; Retention; Discard
* MaxConsumers
* NoAck: disable ack. By default, JetStream will acknowledge each message with an empty reply on the reply subject.
* RePublish: every stored message is re-published to another subject.
  Use: replace a dedicated consumer. Use a transform.
* DenyDelete, DenyPurge: restricts deletion/purge via the API
* [AllowRollup](https://docs.nats.io/nats-concepts/jetstream/streams#allowrollup):
  Allows the use of the `Nats-Rollup` header to replace all messages with a single new message.
  This is for event sourcing snapshots.
  Granularity: replace whole stream / replace one subject.
* FirstSeq: create with a specific initial sequence number
* [SubjectTransform](https://docs.nats.io/running-a-nats-service/configuration/configuring_subject_mapping):
  applies a transform to message subject before storing
* Sealed: no deletion, even through limits
* [Placement](https://docs.nats.io/nats-concepts/jetstream/streams#placement):
  where to place. Using tags or explicit names. This is generally useful to co-locate with the most active clients.
* AllowDirect, MirrorDirect: every replica will respond to direct get requests (default). False: only the leader will (for consistency)
* [Sources, Mirror](https://docs.nats.io/nats-concepts/jetstream/source_and_mirror): replicate messages from another stream. Or many streams.
  Useful for geographic distribution over high latency and unreliable connections. Even if the node is able to reconnect only occasionally.
  It still allows regular producer writes!
  When sourcing a work queue, it'll act as a consumer.

Mirroring example:

```js
// $ nats stream add --config stream_with_sources.json
{
  "name": "SOURCE_TARGET",
  "subjects": [
    "foo1.ext.*",
    "foo2.ext.*"
  ],
  "sources": [
    { "name": "SOURCE1_ORIGIN", },
  ],
  "storage": "file",
}
```


## Config

```nginx
# /nats-server.conf

# Enable JetStream
# It will automatically detect the available resources
jetstream: enabled

# Enable JetStream and specify limits. It's disabled by default.
# By default, the JetStream subsystem will store data in the /tmp directory
jetstream {
  # Where to store the data. Disabled because CLI flag is used already.
  # store_dir: /data/jetstream

  # Max size of the "memory" storage.
  # Default: 75% of the available memory
  max_mem: 1G

  # Max size of the "file" storage
  # Default: 1TB
  max_file: 100G

  # Fsync interval.
  # By default JetStream relies on stream replication for redundancy.
  # If you run JetStream without replication, or with replication of just 2, set a shorter interval.
  # Use "always" to fsync after each message — but this will slow down the throughput to a few hundred msg/s.
  sync_interval: 120s

  # Isolates the JetStream cluster to the local cluster. Recommended use with leaf nodes.
  domain: acme
}

```


## Consumers
A consumer is a stateful view of the stream:
an interface to consume a subset of messages that keeps track of which messages
were delivered and acknowledged by clients.

Equivalent to Consumer Group in Kafka, or Durable Queue subscribed to a topic in RabbitMQ.

Consumers can be:
* *Push consumers*: new messages are pushed to a consumer as they arrive (using a specific subject).
  Message flow is controlled by the server.  Messages are distributed automatically between consumers.
  Use case: receive messages as fast as possible, no ACK. For high message rates.
* *Pull consumers*: request messages explicitly from the server in batches.
  Gives the client full control. Provides horizontal scalability. Don't worry about partitions.
  The tradeoff here is: reliable, but not as fast as possible.

> NATS team: We recommend using *pull consumers* for new projects.
> In particular when scalability, detailed flow control or error handling are a design focus.
>
> However, ephemeral *push consumers* can be a lightweight and useful way to do one-off consumption
> of a subset of messages in a stream.

Acknowledgments:

* Some consumers support ACK of all messages up to a sequence number
* Some consumers require ACK of reception and processing of each message (with a timeout)
* You can send back negative ACK (retry)
* You can send *in progress* ACKs (to indicate that you need more time: avoid the timeout)

If a message is not acknowledged within a user-specified number of delivery attempts,
an advisory notification is emitted.

Consumers can also be ephemeral or durable:
* *Durable*: when an explicit name is set on the `Durable` field when creating the consumer, or when `InactiveThreshold` is set.
  Durable consumers maintain state from one run of the application to another.
* *Ephemeral*: will not have persisted state or fault tolerance and will be automatically cleaned up (deleted) after a period of inactivity (no subscriptions). Applications typically use them to read a stream and quit.

Consumer configuration [see whole list](https://docs.nats.io/nats-concepts/jetstream/consumers#configuration)
* `Durable`: clients can reconnect and resume until the consumer is explicitly deleted
* `InactiveThreshold`: remove if inactive for that long (for ephemeral consumers)
* `Description`: for humans. Useful for ephemeral consumers to indicate their purpose (because there's no durable name)
* `MemoryStorage`: keep in memory. Useful for ephemeral consumers to reduce I/O
* `FilterSubjects`: filter stream subjects, e.g. `[factory-events.A.*, factory-events.B.*]`
* `DeliverPolicy`: start from beginning? offset? time? new messages only? replay last message, or even last per subject?
* `AckPolicy`: explicit (require every message ack), none (naive mode), all (ack only the last message; all previous messages are automatically acknowledged).
* `AckWait`: timeout for consumer ack'ing the message. No ack? will be re-delivered. Also see Backoff.
* `MaxAckPending`: max messages in flight, un-acked.
  For push consumers, this is the only form of flow control.
* `MaxDeliver`: how many times to retry a message if timeout/negative-ack?
  Note: messages that have reached MaxDeliver will stay in the stream.
* `Replicas`: the number of replicas. Default: same as stream

For high throughput, set `MaxAckPending` to a high value.

For applications with high latency due to external services, use a lower value and adjust `AckWait` to avoid re-deliveries.

Only for pull consumers:
* MaxWaiting: how many simultaneous pulls
* MaxRequestExpires: max wait time (for long polling)
* MaxRequestBatch, MaxRequestMaxBytes: max batch size in count of bytes

Only for push consumers:
* DeliverSubject: the server will push messages to this subject
* IdleHeartbeat: heartbeat check
* FlowControl, RateLimit: control how many messages to send
* HeadersOnly: ignore payload, only send headers

Consumer acknowledgements:
with "explicit" ACK, more than one kind of acknowledgment can be used:

* `Ack`: message processed
* `AckSync`: send ACK, and also require that the server confirms the reception of this ACK
* `Nack`: failed, but retry.
  Indicates that the client app is temporarily unable to process it.
* `Term`: failed, do not retry (message invalid, permanent failure).
  Indicates that the client will not be able to process it.
* `InProgress`: more time is needed

Note that when a message is delivered to a subscriber/consumer, a timer starts.
If the message is still not acked in `AckWait` time, it will be redelivered.
`InProgress` resets this timer. `MaxDeliver` controls max number of delivery retries.

Example with CLI:

```console
$ nats stream add ORDERS --storage file --subjects "ORDERS.*" --ack \
  --retention limits --discard old --max-msgs=-1 --max-bytes=-1 --max-age=1y --max-msg-size=-1
$ nats consumer add ORDERS NEW --filter ORDERS.received --ack explicit \
  --pull --deliver all --max-deliver=-1 --sample 100
$ nats consumer add ORDERS DISPATCH --filter ORDERS.processed --ack explicit \
  --pull --deliver all --max-deliver=-1 --sample 100
$ nats consumer add ORDERS MONITOR --filter '' --ack none \
  --target monitor.ORDERS --deliver last --replay instant
```

Create interactively:

```console
$ nats stream add my_stream
Copy
? Subjects foo
? Storage file
? Replication 1
? Retention Policy Limits
? Discard Policy Old
? Stream Messages Limit -1
? Per Subject Messages Limit -1
? Total Stream Size -1
? Message TTL -1
? Max Message Size -1
? Duplicate tracking time window 2m0s
? Allow message Roll-ups No
? Allow message deletion Yes
? Allow purging subjects or the entire stream Yes
Stream my_stream was created

$ nats consumer add
? Consumer name pull_consumer
? Delivery target (empty for Pull Consumers)
? Start policy (all, new, last, subject, 1h, msg sequence) all
? Acknowledgment policy explicit
? Replay policy instant
? Filter Stream by subjects (blank for all)
? Maximum Allowed Deliveries -1
? Maximum Acknowledgments Pending 0
? Deliver headers only without bodies No
? Add a Retry Backoff Policy No
? Select a Stream my_stream
```

Note with Go libraries:
there are two ways to read from the JetStream: the old way and the new way with "nats.io/jetstream" library.

* The old one starts with `nc.Jetstream()` that creates a wrapper that has a `Subscribe()` method — but it's fake:
  it's a shim that makes JetStream feel like Core NATS: you'd subscribe to a topic — but in reality it would
  read from a stream that holds it.
  The goal was to keep the interface familiar and make adoption easier — but in fact, it's more confusing.
* The new one uses `jetstream.New(nc)` and has proper Stream and Subscription objects. Use this one.

Practical notes:

* NATS does not have partitions. You can't map messages to consumers using key hash!
  If you need something like this, do it manually: consume, map, republish.
  This feature is intentionally missing because it makes things complicated: you need to plan your consumers upfront.


## JetStream in Go:

```go
// app/jetstream.go
func serveJetstream(ctx context.Context, k *nats.Conn) error {
	js, err := jetstream.New(k)
	if err != nil {
		return errors.Wrap(err, "JetStream() failed")
	}

	// A stream: a process within NATS that listens to subjects and stores everything it sees.
	// CreateStream() is idempotent! But use UpdateStream() to migrate
	stream, err := js.CreateOrUpdateStream(ctx, jetstream.StreamConfig{
		// NOTE: stream names
		// Stream name is also a subject that you can subscribe to!
		// This is why in NATS you'd use UPPERCASE on stream names and consumers.
		Name:     "orders",
		Description: `Order statuses: new, processed`,
		// Subjects that are consumed by the stream.
		// Wildcards are supported
		Subjects: []string{
			"orders.*",  // -> orders.new, orders.processed
		},
		// Store where? File | Memory
		Storage: jetstream.MemoryStorage,
		// Retention Policy:
		// * `limits`: limit N messages, storage size, message age: MaxAge, MaxMsgs, MaxBytes, MaxMsgsPerSubject
		// * `work`: only keep messages until they're consumed, then delete (NOTE: in this mode consumer filters shouldn't overlap!)
		// * `interest`: keep messages until *all* consumers have ACKed it. Messages are removed if no one is listening.
		Retention: jetstream.LimitsPolicy,
		DenyDelete: true,
		DenyPurge: false,
		AllowRollup: true, // requires purge permission
	})
	if err != nil {
		return errors.Wrap(err, "AddStream() failed")
	}

	// Stream: get one last msg for subject
	{
	msg, err := stream.GetLastMsgForSubject(ctx, "orders.new")
	if err != nil {
		return errors.Wrap(err, "GetLastMsgForSubject() failed")
	}
	fmt.Printf("stream last msg: %+v\n", msg)
	}


	// Consumer: a view into the stream with their own cursor.
	// Idempotent! But use UpdateConsumer() to migrate
	consumer, err := js.CreateOrUpdateConsumer(ctx, "orders", jetstream.ConsumerConfig{
		// Create a durable consumer: because it has a name.
		// Consumer with no name: an ephemeral consumer with an "InactiveThreshold" removal timeout
		//
		// Durable consumers remember where they are. They can be used for load-balancing.
		// Ephemeral consumer does not persist progress. It will get deleted when no one is connected (after a timeout)
		Durable: "new",
		// Description.
		// Especially useful for Ephemeral consumers because they have no name.
		Description: "New orders",
		// Filter messages by subjects
		// NOTE: work queues do not allow subjects to overlap! But regular streams ("limits") and interest streams can.
		FilterSubjects: []string{
			"orders.new",
		},
		// First launch: deliver all messages from the beginning? Only new ones?
		// Replay one last message? Replay one last message per subject?
		DeliverPolicy: jetstream.DeliverAllPolicy,
		// How to ack messages? None|All|Explicit. Don't; ack all messages (batch); ack explicitly every single one
		AckPolicy: jetstream.AckExplicitPolicy,  // ACK every message
		AckWait: 30 * time.Second,
		// Max number of times a message will be redelivered (when nack'ed)
		MaxDeliver: 3,
		// ACK wait time (?)
		// AckWait: 30 * time.Second,
		// MaxWaiting: 512,
		// MaxAckPending: ,
	})
	if err != nil {
		return errors.Wrap(err, "AddConsumer() failed")
	}



	// Publish some messages for ourselves.
	// They will get stored in the JetStream.
	for i := range 5 {
		// Each PublishAsync() returns a promise.
		// But we'll watch them all together using PublishAsyncComplete()
		_, err := js.PublishAsync("orders.new", []byte(fmt.Sprintf("order #%d", i)))
		if err != nil {
			return errors.Wrap(err, "PublishAsync() failed")
		}
	}
	// Wait for all Publish()es simultaneously:
	// PublishAsyncComplete() returns a channel that gets closed when all outstanding requests are ACKed
	select {
	case <-js.PublishAsyncComplete():
	case <-time.After(5 * time.Second):
		fmt.Println("Did not resolve in time")
	}
	fmt.Println("Published")//nocommit




	// Read messages one by one
	{
	msg, err := consumer.Next()
	if err != nil {
		return errors.Wrap(err, "Next() failed")
	}
	fmt.Printf("stream Next(): %+v\n", string(msg.Data()))
	msg.Nak()
	}


	// Read them manually in batches:
	// - Fetch(): receive a batch
	// - Messages(): iterate over messages



	// Start consuming messages: pull consumer.
	// Unlike the legacy Subscribe(), it won't create a consumer but instead use the existing one.
	consumeContext, err := consumer.Consume(func(msg jetstream.Msg){
		fmt.Printf("stream msg: %s %+v\n", msg.Subject(), string(msg.Data()))
		msg.Ack()
	})
	if err != nil {
		return errors.Wrap(err, "Consume() failed")
	}
	defer consumeContext.Stop()

	// Keep working
	<-ctx.Done()


	// Done
	return nil
}
```

Advanced features (as NATS headers):

* `WithMsgID()` to specify a message id for de-duplication
* `WithExpectStream()`: reject a message if it is not received by a specific stream
* `WithExpectLastSequence()`, `WithExpectLastSequencePerSubject()`, `WithExpectLastMsgID()`:
  reject a message if the expected sequence number is different (concurrency control)
* `"Nats-TTL"` Header: specify a message's TTL (overrides stream's default)
* `"Nats-Rollup"` Header: purge all prior messages (in the stream or in the subject)
* `"Nats-Marker-Reason"` Header: reason for message deletion

### Dead Letter Queue

If a message hits the `MaxDeliver` number of retries, it is considered dead.
It will be delivered to the `$JS.EVENT.ADVISORY.CONSUMER.MAX_DELIVERIES.<STREAM>.<CONSUMER>` subject.
It does not contain the payload — but has the `stream_seq` offset that you can read.

Check its schema with:

```console
$ nats schema info io.nats.jetstream.advisory.v1.max_deliver
JSON: type, id, timestamp, stream, consumer, stream_seq, deliveries
```

Terminated messages are published to `$JS.EVENT.ADVISORY.CONSUMER.MSG_TERMINATED.<STREAM>.<CONSUMER>`.
See `$ nats schema info io.nats.jetstream.advisory.v1.terminated`.

You can leverage those advisory messages to implement "Dead Letter Queue" (DLQ).


## Key/Value
JetStream has a persistent key/value store.

You can create *buckets* and use them as *immediately* consistent (as opposed to *eventually* consistent) maps.

NOTE: NATS guarantees "monotonic reads" and "monotonic writes" — but not "read your writes".

Operations:
* get, put, delete, keys, purge (clear)
* atomic set if not exists (can be used for distributed locking)
* atomic compare and set (compare and swap)
* expiring keys (TTL)
* limit: max size of the bucket (LRU)

Because k/v is a value stream, you also have:
* watch key: watch changes for a key (subscribe to key)
* watch all changes
* history: retrieve historical values

Example in CLI:

```console
$ nats kv add my-kv
$ nats kv put my-kv Key1 Value1
$ nats kv get my-kv Key1
$ nats kv del my-kv Key1
```

Key/Value in Go:

```go
// app/kv.go

func serveKvStorage(ctx context.Context, k *nats.Conn) error {
	// KV is implemented using JetStreams.
	js, err := jetstream.New(k)
	if err != nil {
		return errors.Wrap(err, "jetstream.New() failed")
	}

	// Create/Update KV store
	// Use `KeyValue()` to lookup an existing store.
	kv, err := js.CreateOrUpdateKeyValue(ctx, jetstream.KeyValueConfig{
		// Bucket name.
		// A "bucket" is an independent K/V store.
		Bucket: "bucket-name",
		Description: `Test bucket`,
		// The number of historical values to keep per key.
		// Default: 1. Max: 64
		History: 1,
		// Key expiration time.
		// Default: no expiration.
		TTL: 10 * time.Minute,
		// Storage: file | memory
		Storage: jetstream.MemoryStorage,
		// RePublish: publish new values to a subject
		RePublish: &jetstream.RePublish{
			Source: "subject.pattern.>",  // from
			Destination: "subject.pattern.>",  // to
			HeadersOnly: false,
		},
	})
	if err != nil {
		return errors.Wrap(err, "KeyValue() failed")
	}


	// Watch a key, or many keys
	w, err := kv.Watch(ctx, "key-name")
	if err != nil {
		return errors.Wrap(err, "kv.Watch() failed")
	}
	defer w.Stop()
	go func(){
		for {
			select {
			// Will return updates to the key as they come.
			// Will first report the current value, then `nil` as a separator.
			// Opt out with: UpdatesOnly
			case v, ok := <-w.Updates():
				if ok {
					if v == nil {
						fmt.Printf("kv value update: %+v\n", v)
					} else {
						fmt.Printf("kv value update: %+v\n", string(v.Value()))
					}
				}
			case <-ctx.Done():
				return
			}
		}
	}()

	// Store a value
	revision, err := kv.Put(ctx, "key-name", []byte("value1"))
	if err != nil {
		return errors.Wrap(err, "kv.Put() failed")
	}

	// Get a value
	v, err := kv.Get(ctx, "key-name")
	if err != nil {
		return errors.Wrap(err, "kv.Get() failed")
	}
	fmt.Printf("kv value get: %+v\n", string(v.Value()))

	// Get a specific revision
	kv.GetRevision(ctx, "key-name", revision)



	return nil
}
```


## Object Store
Stores arbitrarily large objects (this is achieved by chunking messages).

Note: Object store is not a distributed storage system.
All files in a bucket will need to fit on the target file system.

Example:

```console
$ nats object add myobjbucket
$ nats object put myobjbucket ~/Movies/NATS-logo.mov
$ nats object ls myobjbucket
$ nats object get myobjbucket ~/Movies/NATS-logo.mov
$ nats object rm myobjbucket ~/Movies/NATS-logo.mov
```

You can also watch for changes in a bucket:

```console
$ nats object watch myobjbucket
```

Object Store in Go:

```go
// app/object.go

func serveObjectStorage(ctx context.Context, k *nats.Conn) error {
	// ObjectStore is implemented using JetStreams.
	js, err := jetstream.New(k)
	if err != nil {
		return errors.Wrap(err, "jetstream.New() failed")
	}

	// Create/Update Object store
	// Use `ObjectStore()` to lookup an existing store.
	os, err := js.CreateOrUpdateObjectStore(ctx, jetstream.ObjectStoreConfig{
		// Independent Object Store
		Bucket: "avatars",
		Description: `User avatars`,
		// TTL
		// Default: no expire
		TTL: 365 * 24 * time.Hour,
		// Max file size
		MaxBytes: 100 * MB,
		// Storage: file | memory
		Storage: jetstream.MemoryStorage,
	})
	if err != nil {
		return errors.Wrap(err, "CreateOrUpdateObjectStore() failed")
	}

	// Put: create/overwrite
	buf := bytes.NewBufferString("contents")
	savedObj, err := os.Put(ctx, jetstream.ObjectMeta{
		Name: "filename.txt",
		Description: `Whatever description`,
	}, buf)
	if err != nil {
		return errors.Wrap(err, "Put() failed")
	}
	fmt.Println("\tChunks:", savedObj.Chunks)
	fmt.Println("\tDigest:", savedObj.Digest)
	fmt.Println("\tNUID:", savedObj.NUID)


	// Get the file as I/O
	obj, err := os.Get(ctx, "filename.txt")
	if err != nil {
		return errors.Wrap(err, "Get() failed")
	}
	contents, _ := io.ReadAll(obj)
	fmt.Println("Contents:", string(contents))


	// Watch for any updates
	watcher, err := os.Watch(ctx)
	for update := range watcher.Updates() {
		fmt.Println("Update:", update)

		// Just once
		break
	}

	// Done
	return nil
}

```


## Microservices: Service Mesh and Discovery

Experimental NATS support for microservices: discovery, RPC.
Use JSON, Protobuf, whatever.

```go
// app/micro.go

func microserviceServe(ctx context.Context, k *nats.Conn) error {
	srv, err := micro.AddService(k, micro.Config{
		Name:        "minmax",
		Version:     "0.0.1",
		Description: "Returns the min/max number in a request",
		// Will by default listen on topic <group>.<endpoint>
	})
	if err != nil {
		return errors.Wrap(err, "AddService() failed")
	}

	// Register microservice APIs
	root := srv.AddGroup("minmax")
	root.AddEndpoint("min", micro.HandlerFunc(handleMin))
	root.AddEndpoint("max", micro.HandlerFunc(handleMax))


	// Now make a request
	requestData, _ := json.Marshal([]int{-1, 2, 100, -2000})
	msg, _ := k.Request("minmax.min", requestData, 2*time.Second)
	var res ServiceResult
	json.Unmarshal(msg.Data, &res)
	fmt.Printf("microservice response: %+v\n", res)//nocommit


	// Done
	return nil
}



func handleMin(req micro.Request) {
	// JSON input
	var arr []int
	_ = json.Unmarshal([]byte(req.Data()), &arr)
	slices.Sort(arr)

	// Result
	res := ServiceResult{Min: arr[0]}
	req.RespondJSON(res)
}


func handleMax(req micro.Request) {
	// JSON input
	var arr []int
	_ = json.Unmarshal([]byte(req.Data()), &arr)
	slices.Sort(arr)


	// Result
	res := ServiceResult{Max: arr[len(arr)-1]}
	req.RespondJSON(res)
}

type ServiceResult struct {
	Min int `json:"min,omitempty"`
	Max int `json:"max,omitempty"`
}
```

NATS also enables you to run these functions as serverless:
actually deploy them into the NATS network.

Read more here: [Nex](https://docs.nats.io/using-nats/nex)




# Subject Mapping and Partitioning

A stream provides total order: it guarantees consumers to have the same order of messages.
But sometimes you'd want to scale-out messages publishing or consumption:
this can be achieved by creating N streams, each of which represent a partition of the total set of messages.

Further, with *deterministic subject token partitioning*, you make sure that messages with the same "key" (as in Kafka)
are mapped to the same stream and, consequently, consumers.

Read more here:
* <https://docs.nats.io/nats-concepts/subject_mapping>
* <https://natsbyexample.com/examples/jetstream/partitions/cli>

Subject mapping and transforms: a feature in NATS that can be used to:

* Translate between namespaces
* Suppress subjects (e.g. temporarily for testing)
* For backwards compatibility
* Merging subjects together
* Partitioning subjects
* Filtering messages

Transforms are applied:

* As soon as the message enters the scope: account, stream, etc
* Before any routing or subscription interest is applied
* Are not applied recursively: i.e. only the first matching rule will be applied

Example: because transforms are not recursive, this transform won't create a loop:

```nginx
# /nats-server.conf

mappings: {
	transform.order target.order
	target.order transform.order
}
```


Examples:

```js
{
  // Simply: all messages the server receives on subject "foo" are remapped
  // and can be received by clients subscribed to "bar".
  src: "foo",
  dest: "bar"
}

{
  // Mapping a full wildcard
  src: "orders.>"
  dest: "orders.central.>"
}
{
  // Reference wildcard by position number
  src:  "orders.*.*",
  dest: "orders.{{wildcard(2)}}.{{wildcard(1)}}",  // swap
}
```

Transforms can be used to partition messages deterministically:
i.e. divide a flow of messages into separate streams. Like Kafka partitions.

Examples: if you orders come into `neworders.<customer-id>`,
use `partition(3, 1)` to deterministically split #1 into 3 partitions:

```js
{
  src: "neworders.*",
  dest: "neworders.{{wildcard(1)}}.{{partition(3,1)}}",
}
```

Here's how you can test this mapping:

```console
$ nats server mapping "neworders.*" "neworders.{{wildcard(1)}}.{{partition(3,1)}}" neworders.customerid1
neworders.customerid1.0
```

Partitions are useful when you need to scale consumers
but preserve message ordering from a specific entity (same as Kafka partitions).

Mapping examples:

```nginx
# /nats-server.conf

mappings = {
  # Simple direct mapping.  Messages published to foo are mapped to bar.
  foo: bar

  # Remap tokens: $1, $2 represent token position.
  # In this example bar.a.b would be mapped to baz.b.a.
  bar.*.*: baz.$2.$1

  # You can scope mappings to a particular cluster
  foo.cluster.scoped : [
    { destination: bar.cluster.scoped, weight:100%, cluster: us-west-1 }
  ]

  # Use weighted mapping for canary testing or A/B testing.  Change dynamically
  # at any time with a server reload.
  myservice.request: [
    { destination: myservice.request.v1, weight: 90% },
    { destination: myservice.request.v2, weight: 10% }
  ]

  # A testing example of wildcard mapping balanced across two subjects.
  # 20% of the traffic is mapped to a service in QA coded to fail.
  myservice.test.*: [
    { destination: myservice.test.$1, weight: 80% },
    { destination: myservice.test.fail.$1, weight: 20% }
  ]

  # A chaos testing trick that introduces 50% artificial message loss of
  # messages published to foo.loss
  foo.loss.>: [ { destination: foo.loss.>, weight: 50% } ]
}
```

## Weighted mappings
Traffic can be split by percentage from one subject transform to multiple subject transforms.

Use: For A/B testing or canary releases

```nginx
# /nats-server.conf

myservice.requests: [
    { destination: myservice.requests.v1, weight: 98% },
    { destination: myservice.requests.v2, weight: 2% }
]
```




# Authentication

In NATS, every *account* is an isolated subject namespace: it's multi-tenant.
Accounts, however, can import and export specific subjects.

NATS Authentication:

* Token
* Bcrypted token
* Username/Password
* Bcrypted username/Password
* TLS Certificate
* NKEY with Challenge (ed25519)
* Decentralized JWT Authentication/Authorization
* Auth callout (script)

Authentication is configured in the `authorization` section. But JWT auth is configured in `resolver {}` section.

The recommended way to manage accounts and users is using the `nsc` CLI tool:

```console
$ nsc add account -n APP
$ nsc add user -a APP -n <username> -public-key <nkey>

$ # Allow
$ nsc edit user --name <n> --allow-pubsub <subject>,...
$ nsc edit user --name <n> --allow-pub <subject>,...
$ nsc edit user --name <n> --allow-sub <subject>,...

$ # Deny
$ nsc edit user --name <n> --deny-pubsub <subject>,...
$ nsc edit user --name <n> --deny-pub <subject>,...
$ nsc edit user --name <n> --deny-sub <subject>,...

$ # With queues
$ nsc edit user --name <n> --deny-sub "<subject> <queue>,..."
$ nsc edit user --name <n> --allow-sub "<subject> <queue>,..."
```

## Tokens
The easiest way to connect.
This method is exclusive of user and password:

```nginx
# /nats-server.conf

authorization {
  token: "s3cr3t"
}
```

Now connect:

```console
$ nats sub -s nats://s3cr3t@localhost:4222 ">"
```

Tokens can be bcrypted as an additional layer of security, as the clear-text version of the token would not be persisted on the server configuration file.

```console
$ nats server passwd
? Enter password [? for help] **********************
? Reenter password [? for help] **********************
$2a$11$PWIFAL8RsWyGI3jVZtO9Nu8.6jOxzxfZo7c/W0eLk017hjgUKWrhy
```

## Username/Password
As an alternative to tokens, you may specify a single user/password (exclusive of tokens):

```nginx
# /nats-server.conf

authorization: {
    user: a,
    password: b
}
```

or provide multiple users:

```nginx
# /nats-server.conf

authorization: {
    users: [
        {user: a, password: b},
        {user: b, password: a}
    ]
}
```

This block can live at the top level and define users on the default `$G` account.
Or it can be nested within accounts.

Use bcrypted passwords for more security:

```console
$ nats server passwd
? Enter password [? for help] **********************
? Reenter password [? for help] **********************

$2a$11$V1qrpBt8/SLfEBr4NJq4T.2mg8chx8.MTblUiTBOLV3MKDeAy.f7u
```

```nginx
# /nats-server.conf

authorization: {
    users: [
        {user: a, password: "$2a$11$V1qrpBt8/SLfEBr4NJq4T.2mg8chx8.MTblUiTBOLV3MKDeAy.f7u"},
        ...
    ]
}
```

Reload the server as you add/remove passwords:

```
$ nats-server --signal reload
```

## TLS Certificate
You can extract information from a trusted certificate to authenticate a client.

Here's how you generate certificates: [NATS TLS](https://docs.nats.io/running-a-nats-service/configuration/securing_nats/tls#self-signed-certificates-for-testing)

Configure the server to first verify certificates:

```nginx
# /nats-server.conf

tls {
  cert_file: "server-cert.pem"
  key_file:  "server-key.pem"
  ca_file:   "rootCA.pem"
  verify:    true
}
```

This option verifies the client's certificate is signed by the CA specified in the ca_file option.
It also makes sure that the client provides a certificate with the extended key usage "TLS Web Client Authentication".

Specify `verify_and_map` to use information encoded in the certificate to authenticate a client:

```nginx
# /nats-server.conf

tls {
  cert_file: "server-cert.pem"
  key_file:  "server-key.pem"
  ca_file:   "rootCA.pem"
  # Require a client certificate and map user id from certificate
  verify_and_map: true
}
```

The server will now check if a Subject Alternative Name (SAN) maps to a user.
It will search:
* All email addresses first
* All DNS names
* Finally, it will try the certificate subject

```console
$ openssl x509 -noout -text -in  client-cert.pem
Certificate:
...
        X509v3 extensions:
            X509v3 Subject Alternative Name:
                DNS:localhost, IP Address:0:0:0:0:0:0:0:1, email:email@localhost
            X509v3 Extended Key Usage:
                TLS Web Client Authentication
...
```

The configuration to authorize this user would be as follow:

```nginx
# /nats-server.conf

authorization {
  users = [
    {user: "email@localhost"}
  ]
}
```

Use the RFC 2253 Distinguished Names syntax to specify a user corresponding to the certificate subject:

```console
$ openssl x509 -noout -text -in client-cert.pem
Certificate:
    Data:
...
        Subject: O=mkcert development certificate, OU=testuser@MacBook-Pro.local (Test User)
...
```

The configuration to authorize this user requires you to specify the full subject.
Mind the order or attributes!!

```nginx
# /nats-server.conf

authorization {
  users = [
    {user: "OU=testuser@MacBook-Pro.local (Test User),O=mkcert development certificate"}
  ]
}
```

## NKeys (ed25519)
NKeys are like ssh keys: they let you verify identities without ever seeing the private key!
The sever requires the client to sign a random challenge as a proof: it's immune to playback attacks.
If the public key is known to the server, authentication succeeds.

It's a great replacement for tokens.

Generate an nkey.

```console
$ nk -gen user -pubout
SUACSSL3UAHUDXKFSNVUZRF5UHPMWZ6BFDTJ7M6USDXIEDNPPQYYYCU3VY
UDXU4RCSJNZOIQHZNWXHXORDPRTGNJAHAHFRGZNEEJCPQTT2M7NLCNF4
```

The first line starts with `S...`: it is the *seed* (the private key).
The second line starts with `U...`: it is the *user* key (public key). It can be safely shared.

```nginx
# /nats-server.conf

authorization: {
  users: [
    { nkey: UDXU4RCSJNZOIQHZNWXHXORDPRTGNJAHAHFRGZNEEJCPQTT2M7NLCNF4 }
  ]
}
```

## Auth Callout
Delegate authentication and authorization to an application-defined NATS service.
This can be LDAP, SAML, and OAuth, an ad-hoc database, or even a file on disk.

NATS would PUB a request to `$SYS.REQ.USER.AUTH` and expect a response (from a service)
with a JWT for this user. The JWT would contain permissions.

Read more: <https://docs.nats.io/running-a-nats-service/configuration/securing_nats/auth_callout>

# Decentralized JWT Authentication

Tutorial:
1. <https://docs.nats.io/running-a-nats-service/configuration/securing_nats/auth_intro/jwt>
2. <https://docs.nats.io/using-nats/nats-tools/nsc>

JWTs provide for a distributed configuration paradigm. Prevously, you'd create a user on the server
before they could sign in. Now you don't have to create a user: the token itself contains their username and permissions!

When a new user is added to an Account, the account configuration need not change:
each user can and should have its own user JWT that can be verified against the Account.

NOTE: Accounts and their separation still applies! But now the configuration of accounts,
exports, imports, permissions are moved away from the server into several trusted JWTs.

Requirements in NATS:

* JWTs to be signed with ed25519 only! NATS will reject everything else
* Both `Issuer` and `Subject` must be a public NKey (user)
* `Issuer` and `Subject` must match specific roles (depending on the claim NKeys)

NKeys Roles are hierarchical and form a chain of trust:

1. Operators
2. Accounts
3. Users

Operators → Accounts → Users. *Operators* issue *Accounts* which in turn issue *Users*.
Servers trust specific *Operators*. If an account is issued by an operator that is trusted, account users are trusted.

1. When a User connects to a server, it presents a JWT issued by its Account and proves by signing a challenge.
2. Next, the server retrieves the associated Account and verifies that the User's `Issuer` matches the referenced account.
3. Finally, the server checks that a trusted Operator (i.e. one the server is configured with) issued the Account, completing the chain of trust.

In NATS, JWTs are only used as configuration: issuer, subject, capabilities.
From the authorization point of view:

1. The Account provides information on the account: import/export subjects, limits
2. The User's JWT can have permissions: restrictions on which subjects they can pub/sub to.

## Configure the Server

To configure JWT security, you only need to tell the server to trust the Operator token.
Once the server is configured, you'll use `nsc` locally and sync with the account resolvers built into the nats-server.

Create an operator, account, user:

```console
$ nsc add operator MyOperator
[ OK ] generated and stored operator key "OCXEXYLQMYU4PRJZU6YL25SBISJI5BUE77LHOZ72OCTQO5RDUVUMHWFK"
[ OK ] added operator "MyOperator"
[ OK ] When running your own nats-server, make sure they run at least version 2.2.0
$ nsc edit operator --service-url nats://localhost:4222
[ OK ] added service url "nats://localhost:4222"
[ OK ] edited operator "MyOperator"

$ nsc add account MyAccount
[ OK ] generated and stored account key "ADHEBFJOYMMCLQ2H3FHGKSRHQY65CWJRNQTOX5GQPEXMQ6MK3XFTO5TM"
[ OK ] added account "MyAccount"

$ nsc add user MyUser
[ OK ] generated and stored user key "UAZTWWQCU5T4Y7OMDVJN75LUGI5LXNOFLDDMTSU2X6YGFQ62KIANUPZY"
[ OK ] generated user creds file `/nsc/nkeys/creds/MyOperator/MyAccount/MyUser.creds`
[ OK ] added user "MyUser" to account "MyAccount"
```

Every tool generated an NKey representing the account/user and stored them safely in the keystore
in `$NSC_HOME` (default: `~/.nsc` or `/nsc` in Docker):
JWTs go into `$NSC_HOME/nats`, and NKeys go into `$NKEYS_PATH=/nsc/nkeys`:

```
$ find /nsc
/nsc/nats/nsc/stores/MyOperator/.nsc
/nsc/nats/nsc/stores/MyOperator/accounts/MyAccount/MyAccount.jwt
/nsc/nats/nsc/stores/MyOperator/accounts/MyAccount/users/MyUser.jwt
/nsc/nats/nsc/stores/MyOperator/MyOperator.jwt

/nsc/nkeys/creds/MyOperator/MyAccount/MyUser.creds
/nsc/nkeys/keys/O/CX/OCXEXYLQMYU4PRJZU6YL25SBISJI5BUE77LHOZ72OCTQO5RDUVUMHWFK.nk
/nsc/nkeys/keys/U/AZ/UAZTWWQCU5T4Y7OMDVJN75LUGI5LXNOFLDDMTSU2X6YGFQ62KIANUPZY.nk
/nsc/nkeys/keys/A/DH/ADHEBFJOYMMCLQ2H3FHGKSRHQY65CWJRNQTOX5GQPEXMQ6MK3XFTO5TM.nk
```

Every operator, account, and user gets an NKey and a JWT.
NKey seeds (private keys) are stored inside files named after user keys (public keys).
Operator's JWT looks like this:

```js
{
  // Issued at
  "iat": 1750110083,
  // JWT ID (unique identifier for this token)
  "jti": "N4EAWCWO4SQBVKDUS64CVIEWCGQXKPZ7VJXPBJP5XCOIAR45UIMQ",
  // Issuer: who created and signed it
  "iss": "OCXEXYLQMYU4PRJZU6YL25SBISJI5BUE77LHOZ72OCTQO5RDUVUMHWFK",
  // Subject: whom the token refers to
  "sub": "OCXEXYLQMYU4PRJZU6YL25SBISJI5BUE77LHOZ72OCTQO5RDUVUMHWFK",
  // Username (from nsc)
  "name": "MyOperator",
  // Nats-specific
  "nats": {
    "type": "operator",
    "version": 2
    "operator_service_urls": [
      // NATS server URL. Tools can use it.
      "nats://localhost:4222"
    ],
  }
}
```

List created keys:

```console
# nsc list keys
+------------+----------------------------------------------------------+-------------+--------+
| MyOperator | OCXEXYLQMYU4PRJZU6YL25SBISJI5BUE77LHOZ72OCTQO5RDUVUMHWFK |             | *      |
|  MyAccount | ADHEBFJOYMMCLQ2H3FHGKSRHQY65CWJRNQTOX5GQPEXMQ6MK3XFTO5TM |             | *      |
|   MyUser   | UAZTWWQCU5T4Y7OMDVJN75LUGI5LXNOFLDDMTSU2X6YGFQ62KIANUPZY |             | *      |
+------------+----------------------------------------------------------+-------------+--------+
# nsc describe operator
+----------------------------------------------------------------------------------+
|                                 Operator Details                                 |
+-----------------------+----------------------------------------------------------+
| Name                  | MyOperator                                               |
| Operator ID           | OCXEXYLQMYU4PRJZU6YL25SBISJI5BUE77LHOZ72OCTQO5RDUVUMHWFK |
| Issuer ID             | OCXEXYLQMYU4PRJZU6YL25SBISJI5BUE77LHOZ72OCTQO5RDUVUMHWFK |
| Issued                | 2025-06-16 21:41:23 UTC                                  |
| Expires               |                                                          |
| Operator Service URLs | nats://localhost:4222                                    |
| Require Signing Keys  | false                                                    |
+-----------------------+----------------------------------------------------------+
# nsc describe account
+--------------------------------------------------------------------------------------+
|                                   Account Details                                    |
+---------------------------+----------------------------------------------------------+
| Name                      | MyAccount                                                |
| Account ID                | ADHEBFJOYMMCLQ2H3FHGKSRHQY65CWJRNQTOX5GQPEXMQ6MK3XFTO5TM |
| Issuer ID                 | OCXEXYLQMYU4PRJZU6YL25SBISJI5BUE77LHOZ72OCTQO5RDUVUMHWFK |
| Issued                    | 2025-06-16 21:41:56 UTC                                  |
| Expires                   |                                                          |
+---------------------------+----------------------------------------------------------+
| Max Connections           | Unlimited                                                |
| Max Leaf Node Connections | Unlimited                                                |
| Max Data                  | Unlimited                                                |
| Max Exports               | Unlimited                                                |
| Max Imports               | Unlimited                                                |
| Max Msg Payload           | Unlimited                                                |
| Max Subscriptions         | Unlimited                                                |
| Exports Allows Wildcards  | True                                                     |
| Disallow Bearer Token     | False                                                    |
| Response Permissions      | Not Set                                                  |
+---------------------------+----------------------------------------------------------+
| Jetstream                 | Disabled                                                 |
+---------------------------+----------------------------------------------------------+
| Imports                   | None                                                     |
| Exports                   | None                                                     |
+---------------------------+----------------------------------------------------------+
| Tracing Context           | Disabled                                                 |
+---------------------------+----------------------------------------------------------+
```

Now let's configure the server to trust the Operator NKey.
Create a system account, then generate a config file:

```console
$ nsc add account -n SYS
$ nsc edit operator --system-account SYS
$ nsc generate config --nats-resolver
```

Now push your changes to accounts to the NATS server:

```console
$ nsc push -a MyAccount -u nats://nats:4222
[ OK ] push to nats-server "nats://nats:4222" using system account "SYS":
       [ OK ] push MyAccount to nats-server with nats account resolver:
              [ OK ] pushed "MyAccount" to nats-server NBK2TWXIXEWN2RDHI7A6VVTBNIA4ACVXUGOISDIU53TLFP7P2TMBDD6W: jwt updated
              [ OK ] pushed to a total of 1 nats-server
```

the server will take care of distributing that new account changes to the other nats servers in the cluster.

If you need to work with more accounts, enter the folder of an account to select it:

```console
$ cd /nsc/nats/nsc/stores/MyOperator/accounts/MyAccount#
$ nsc env
+--------------------------+-----+-------------------------------------------------------------------------------+
| From CWD                 |     | Yes                                                                           |
| Default Stores Dir       |     | /nsc/nats/nsc/stores                                                          |
| Current Store Dir        |     | /nsc/nats/nsc/stores                                                          |
| Current Operator         |     | MyOperator                                                                    |
| Current Account          |     | MyAccount                                                                     |
| Root CAs to trust        |     | Default: System Trust Store                                                   |
+--------------------------+-----+-------------------------------------------------------------------------------+
```

### Client Connect

The `/nsc/nkeys/creds` directory is organized in a way friendly to humans.
Every user file contains a JWT and an NKey in one file.
These files are used by NATS clients to connect to a NATS server:

Test a client:

```console
# nats sub --creds /nsc/nkeys/creds/MyOperator/MyAccount/MyUser.creds ">"
22:25:12 Subscribing on >
[#1] Received on "hello"
NATS

# nats pub --creds /nsc/nkeys/creds/MyOperator/MyAccount/MyUser.creds hello NATS
22:25:59 Published 4 bytes to "hello"
```

Generate a nats client context if you plan to use cli on this user:

```console
$ nats context add myuser --creds /nsc/nkeys/creds/MyOperator/MyAccount/MyUser.creds
```

### Create JWT Users Programmatically

If you need to generate JWT keys in Go:

See here: <https://natsbyexample.com/examples/auth/nkeys-jwts/go>


## JWT Authorization
With `nsc` you can specify subjects to which the user can or cannot publish or subscribe.
By default a user doesn't have any limits: the whole account is available to them.

When limiting users, remember about generated inboxes to allow requests:
allow publish and subscribe to `_INBOX.>`!
Let's create permissions for "client" that can publish on the request subject "q",
and receive replies on an inbox:


```console
$ nsc add user service --allow-pub "_INBOX.>" --allow-sub q
$ nsc add user client --allow-pub q --allow-sub "_INBOX.>"
$ nsc describe user service
+----------------------+----------------------------------------------------------+
| Name                 | service                                                  |
| User ID              | UDYQFIF75SQU2NU3TG4JXJ7C5LFCWAPXX5SSRB276YQOOFXHFIGHXMEL |
| Issuer ID            | AD2M34WBNGQFYK37IDX53DPRG74RLLT7FFWBOBMBUXMAVBCVAU5VKWIY |
| Issued               | 2021-10-27 23:23:16 UTC                                  |
| Expires              |                                                          |
| Bearer Token         | No                                                       |
+----------------------+----------------------------------------------------------+
| Pub Allow            | _INBOX.>                                                 |
| Sub Allow            | q                                                        |
| Response Permissions | Not Set                                                  |
+----------------------+----------------------------------------------------------+
| Max Msg Payload      | Unlimited                                                |
| Max Data             | Unlimited                                                |
| Max Subs             | Unlimited                                                |
| Network Src          | Any                                                      |
| Time                 | Any                                                      |
+----------------------+----------------------------------------------------------+
```

## Scoped Signing Keys
Previously if you wanted to limit the permissions of users, you had to specify permissions on a per-user basis.
With scoped signing keys, you associate a signing key with a set of permissions:

```console
$ nsc edit account -n A --sk generate
[ OK ] added signing key "AAZQXKDPOTGUCOCOGDW7HWWVR5WEGF3KYL7EKOEHW2XWRS2PT5AOTRH3"
$ nsc edit signing-key --account A --role service \
  --sk AAZQXKDPOTGUCOCOGDW7HWWVR5WEGF3KYL7EKOEHW2XWRS2PT5AOTRH3 \
  --allow-sub "q.>" --deny-pub ">" --allow-pub-response
$ nsc add user U -K service
```

Now this user "inherits" permissions from the signing key.

## Template Functions
Template Functions are more powerful when you want more flexibility:
e.g. you want users `pam` and `joe` to subscribe to their own subjects `pam.>` and `joe.>`.

```console
nsc edit signing-key \
  --account sales \
  --role team-service \
  --sk AXUQXKDPOTGUCOCOGDW7HWWVR5WEGF3KYL7EKOEHW2XWRS2PT5AOTRH3 \
  --allow-sub "{{account-name()}}.{{tag(team)}}.{{name()}}.>" \
  --allow-pub-response
```

Available templates:

* `{{name()}}` - user name
* `{{subject()}}` - user publis NKey value (long token: `UAC...`)
* `{{account-name()}}` - signing account name (e.g. sales)
* `{{account-subject()}}` - account public NKey (long token: `AXU...`)
* `{{tag(key)}}` - key:value tags associated with the signing key (custom values from the user)

Now create users:

```console
$ nsc add user pam -K team-service --tag team:support
$ nsc add user joe -K team-service --tag team:leads
```

## For MQTT
MQTT cannot sign the challenge: therefore you'd use the whole JWT token as a password. With any username.

The JWT has to have the Bearer boolean set to true, which can be done with nsc:

```console
$ nsc edit user --name U --account A --bearer
```

To allow users to use MQTT, set `allowed_connection_types: ["MQTT"]` on them:

```nginx
# /nats-server.conf

authorization {
  users [
    {user: foo password: foopwd, permission: {...}}
    {user: bar password: barpwd, permission: {...},
     allowed_connection_types: ["MQTT"]
    }
  ]
}
```

They'd also need permissions to use JetStreams for QoS1:

```nginx
# /nats-server.conf

listen: 127.0.0.1:4222
jetstream: enabled
authorization {
    mqtt_perms = {
        publish = ["baz"]
        subscribe = ["foo", "bar", "$MQTT.sub.>"]
    }
    users = [
        {user: mqtt, password: pass,
         permissions: $mqtt_perms,
         allowed_connection_types: ["MQTT"]
        }
    ]
}
mqtt {
    listen: 127.0.0.1:1883
}
```



## Create Users and NKeys Programmatically

See: <https://natsbyexample.com/examples/auth/nkeys-jwts/go>


## JWT Import/Export Streams

To share messages you publish with other accounts, you have to *Export a Stream*.

Export any stream that matches the pattern as *public stream*:

```console
$ nsc add export --name abc --subject "a.b.c.>"
$ nsc describe account
╭───────────────────────────────────────────────────────────╮
│ Name │ Type   │ Subject │ Public │ Revocations │ Tracking │
├──────┼────────┼─────────┼────────┼─────────────┼──────────┤
│ abc  │ Stream │ a.b.c.> │ Yes    │ 0           │ N/A      │
╰──────┴────────┴─────────┴────────┴─────────────┴──────────╯
```

Now another account can import the stream to get message forwarding.
You'll need to know the other account's public key and subject name.

Note that messages will be received on the same subject name.
You can use `--local-prefix` to prefix it.
For example if `--local-subject abc`, the message will be received as abc.a.b.c.>.

```console
$ nsc add account B
$ nsc add import --src-account ADETPT36WBIBUKM3IBCVM4A5YUSDXFEJPW4M6GGVBYCBW7RRNFTV5NGE --remote-subject "a.b.c.>" --local-subject prefix
```

A *private stream* will only be available to the accounts you designate:

```console
$ nsc add export --subject "private.abc.*" --private --account A
```

You'll have to get an *activation token* from another account to import it:

```console
$ nsc generate activation --account A \
  --target-account AAM46E3YF5WOZSE5WNYWHN3YYISVZOSI6XHTF2Q64ECPXSFQZROJMP2H \
  --subject private.abc.AAM46E3YF5WOZSE5WNYWHN3YYISVZOSI6XHTF2Q64ECPXSFQZROJMP2H \
  -o /tmp/activation.jwt
$ nsc add import --account B --token /tmp/activation.jwt
```

## JWT Import/Export Services

Likewise, you can import/export services between accounts.

More info: <https://docs.nats.io/using-nats/nats-tools/nsc/services>

## Revocation
Revocations are stored in the account JWT.

```console
$ nsc revocations add-user -n <name> | -u <pubkey>
$ nsc push -i
```

If a revoked client is currently connected, their connection will be terminated.




# Authorization
NATS authorization: subject-level permissions on a per-user basis.
Each permission specifies the subjects the user can publish to and subscribe to.

Special field: `default_permissions` contains permissions that apply to users that do not have permissions associated with them.

Special field: `no_auth_user`: Clients connecting without authentication can be associated with a particular user within an account.
Please note that the no_auth_user will not work with nkeys.

NOTE: It is important to not break request-reply patterns.
In some cases you need to add rules for the `_INBOX.>` pattern.
The `allow_responses` option can simplify this.

Permissions map:

* `publish`: subject(s) list or permissions map (see below)
* `subscribe`: subjects(s) list or permissions map.
  Here you can provide an optional queue name: `<subject> <queue>`.
  Use wildcards: `v2.*` or `v2.>`
* `allow_responses`: set `true` to dynamically allow publishing to reply subjects.
  Note: if you provide a reply subject that this client normally does not have access to,
  it will still temporarily be allowed!

  Note: enabling this implicitly denies publish to other subjects (!),
  however an explicit `publish` allow on a subject will override.

  Note: in `nsc`, this option is called `--allow-pub-response`.

  Also can be a map: `max` max number of allowed responses, and `expires` the amount of time the permission is valid for.

Permissions map: more granular allow/deny lists:

* `allow`: subject(s)
* `deny`: subject(s). In case of overlap, `deny` has priority.

Example:

```nginx
# /nats-server.conf

# No Auth User: The user to assume when no auth is provided.
# Their permissions and account will be used.
no_auth_user: app

authorization {
  # Special entry: applies to all users that don't have specific permissions set.
  default_permissions = {
    publish = "SANDBOX.*"
    subscribe = ["PUBLIC.>", "_INBOX.>"]
  }

  # Clients connecting without authentication can be associated with a particular user within an account.
  no_auth_user: other

  # Variables
  ADMIN = {
    publish = ">"
    subscribe = ">"
  }
  REQUESTOR = {
    publish = ["req.a", "req.b"]
    subscribe = "_INBOX.>"
  }
  RESPONDER = {
    subscribe = ["req.a", "req.b"]
    publish = "_INBOX.>"
  }

  # Variables. Missing ones are taken from the env.
  users = [
    {user: admin,   password: $ADMIN_PASS, permissions: $ADMIN}
    {user: client,  password: $CLIENT_PASS, permissions: $REQUESTOR}
    {user: service,  password: $SERVICE_PASS, permissions: $RESPONDER}
    {user: other, password: $OTHER_PASS}
  ]
}
```

Example: queue permissions.

* User "a" can ony subscribe to "foo" as part of the queue subscriptions "queue"
* User "b" has permissions for queue subscriptions as well as plain subscriptions.
  They are, however, not allowed to use any queues with the name `*.prod`:

```nginx
# /nats-server.conf

users = [
  {
    user: "a", password: "a", permissions: {
      sub: {
        allow: ["foo queue"]
     }
  }
  {
    user: "b", password: "b", permissions: {
      sub: {
        # Allow plain subscription foo, but only v1 groups or *.dev queue groups
        allow: ["foo", "foo v1", "foo v1.>", "foo *.dev"]

        # Prevent queue subscriptions on prod groups
        deny: ["> *.prod"]
     }
  }
]
```

## Accounts
Accounts allow the grouping of clients, isolating them from clients in other accounts, thus enabling multi-tenancy in the server.

The top-level `accounts{ }` maps account names to configs:

```nginx
# /nats-server.conf

accounts: {
    A: {
        users: [
            {user: a, password: a}
        ]
    },
    B: {
        users: [
            {user: b, password: b}
        ]
    },
}
```

While the name account implies one or more users, it is much simpler and enlightening to think of one account as a messaging container for one application.

## Import/Export Subjects

You can *export* streams (actually, subjects) and services from one account and import them into another.

> NOTE: In import/export, the term "stream" refers to a stream of Core NATS messages. Subjects, actually.
> This is an unfortunate naming collision with JetStreams: as the import/export between accounts predates JetStream.

Configuration:

* `exports`: the services and streams that others can import.

  * `stream`: subject wildcards that the account will publish
  * `service`: subject wildcards that the account will subscribe to (exclusive of `stream`)
  * `accounts`: (optional) list of account names that can import the stream (limitation)
  * `response_type`: indicates if a response to a service request consists of a `single` (default) or a `stream` of messages.

* `imports`: the services and streams that an Account imports.

  * `stream` or `service`: map `{account: string, subject: string}`
  * `prefix`: A local subject prefix mapping for the imported stream. (applicable to stream)
  * `to`: A local subject mapping for imported service. (applicable to service)


Example:

```nginx
# /nats-server.conf

accounts: {
    A: {
        users: [
            {user: a, password: a}
        ]
        exports: [
            {stream: puba.>}
            {service: pubq.>}
            {stream: b.>, accounts: [B]}
            {service: q.b, accounts: [B]}
        ]
    },
    B: {
        users: [
            {user: b, password: b}
        ]
        imports: [
            {stream: {account: A, subject: b.>}}
            {service: {account: A, subject: q.b}}
        ]
    }
    C: {
        users: [
            {user: c, password: c}
        ]
        imports: [
            {stream: {account: A, subject: puba.>}, prefix: from_a}
            {service: {account: A, subject: pubq.C}, to: Q}
        ]
    }
}
```

## OCSP Stapling
The Online Certificate Status Protocol (OCSP) stapling, formally known as the TLS Certificate Status Request extension,
is a standard for checking the revocation status of X.509 digital certificates. It allows the presenter of a certificate
to bear the resource cost involved in providing Online Certificate Status Protocol (OCSP) responses by appending ("stapling")
a time-stamped OCSP response signed by the CA (certificate authority) to the initial TLS handshake.

When a certificate is configured with OCSP Must-Staple, the NATS Server will fetch staples from the configured OCSP responder URL that is present in a certificate.

```nginx
# /nats-server.conf

[ ext_ca ]
...
authorityInfoAccess = OCSP;URI:http://ocsp.example.net:80
tlsfeature = status_request
...
```

Read more: <https://docs.nats.io/running-a-nats-service/configuration/securing_nats/ocsp>










# Connectivity

* Plain NATS connections
* TLS encrypted NATS connections
* WebSocket
* MQTT

Also bridges:

* Kafka
* JSM for RabbitMQ bridge
* More

Monitoring NATS:

* <https://github.com/nats-io/nats-top>

Config:

```nginx
# /nats-server.conf

# Default: 4222
port: 4222
monitor_port: 8222

```


## TLS

NATS prefers this scheme:

1. You connect to the server, no encryption
2. The server sends you an [INFO message](https://docs.nats.io/reference/reference-protocols/nats-protocol#info)
   which tells the client whether TLS is required
3. You switch to TLS

However, the server can be configured to do TLS handshake right away: `handshake_first: true`.

More details about TLS: <https://docs.nats.io/running-a-nats-service/configuration/securing_nats/tls>


## MQTT
For an MQTT client to connect to the NATS server, the user's account must be JetStream enabled.
When an MQTT client creates a subscription on a topic, the NATS server creates the similar NATS subscription.

NOTE: MQTT wildcard `#` may cause the NATS server to create two subscriptions.

MQTT topics are mapped to NATS subjects: `a/b` -> `a.b`.

Extra slashes may create surprising topics:

```
/foo/bar    /.foo.bar
foo/bar/    foo.bar./
foo//bar    foo./.bar
//foo/bar   /./.foo.bar
foo.bar     foo//bar
```

QoS1:
When the server delivers a QoS 1 message to a QoS 1 subscription,
it will keep the message until it receives the PUBACK for the corresponding packet identifier.
If it does not receive it within the "ack_wait" interval, that message will be resent.

Max Ack Pending:
This is the amount of QoS 1 messages the server can keep retrying.

Retained Messages:
When the server receives a message published with the RETAIN flag set,
it will store the message for future subscribers.

Config:

```nginx
# /nats-server.conf

# MQTT Configuration
# README: https://docs.nats.io/running-a-nats-service/configuration/mqtt/mqtt_config
mqtt {
  port: 1883

  # The username to assume when no user is provided.
  # Their permissions and account will be used.
  no_auth_user: default_mqtt_client

  # Explicit usernames.
  # Use if there are no users configured for accounts.
  authorization {
    # Username/Password
    # username: "my_user_name"
    # password: "my_password"

    # Or token (provided as "password" in the CONNECT packet)
    # token: "my_token"
  }

  # JWT authentication: pass the JWT token as password. With any user name, except empty.
  # The JWT has to have the Bearer boolean set to true, which can be done with nsc:
  # $ nsc edit user --name U --account A --bearer


  # Redeliver QoS 1 messages as a DUPLICATE if the server has not received the PUBACK
  ack_wait: "30m"
}

# MQTT requires the server name to be set
server_name: "my_mqtt_server"

# This section is only useful if when no accounts are defined.
# Otherwise, define your `authorization` and `authentication` in `account.users`
authorization {
  # When an MQTT client creates a QoS 1 subscription, this translates to the creation of a JetStream durable subscription.
  # To receive messages for this durable, the NATS Server creates a subscription with a subject such as $MQTT.sub.
  # and sets it as the JetStream durable's delivery subject.
  # Therefore, if you have set some permissions for the MQTT user, you need to allow subscribe permissions on $MQTT.sub.>.
  mqtt_perms = {  # <- variable
    publish = ["baz"]
    subscribe = ["foo", "bar", "$MQTT.sub.>"]
  }

  users [
    # This user can only use MQTT
    # Options: STANDARD WEBSOCKET LEAFNODE MQTT
    {
      user:mqtt, password:pass,
      permissions: $mqtt_perms,
      allowed_connection_types: ["MQTT"]
    }
  ]
}
```

## WebSocket

WebSocket uses the [NATS Protocol](https://docs.nats.io/reference/reference-protocols/nats-protocol).
You'll use `nats.ws` to be able to publish and subscribe.

Config:

```nginx
# /nats-server.conf

# Turn on websockets
websocket {
  port: 443
  no_tls: true  # for test environments
  compression: true

  # CORS
  same_origin: true
  allowed_origins [
    "http://www.example.com"
    "https://www.other-example.com"
  ]

  # Use this Cookie, if present, as the client's JWT.
  # If the client specifies a JWT in the CONNECT protocol, this option is ignored.
  jwt_cookie: "my_jwt_cookie_name"

  # Separate "no_auth_user" for websocket clients
  no_auth_user: "my_username_for_apps_not_providing_credentials"

  # Limit websocket clients to specific users.
  authorization {}
}

```

# System Events
System events:
These events are enabled by configuring `system_account` and subscribing/requesting using a system account user.

Events:

* `$SYS.ACCOUNT.<id>.CONNECT`, `$SYS.ACCOUNT.<id>.DISCONNECT` client connect/disconnect

Request server stats:

* `$SYS.REQ.SERVER.<id>.<endpoint-name>`: server info, connections, routing, JetStream, accounts, etc
* `$SYS.REQ.ACCOUNT.<account-id>.<endpoint-name>`: account specific monitoring endpoint: connections, subscriptions, etc
* `$SYS.REQ.USER.INFO`: get connected user info
* `$SYS.REQ.SERVER.<id>.RELOAD`: hot reload configuration

More: <https://docs.nats.io/running-a-nats-service/configuration/sys_accounts>








