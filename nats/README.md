# NATS

Read:
* <https://docs.nats.io/>
* <https://natsbyexample.com/>

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











# Concepts

## Subjects

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

## Core NATS

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

## Queue Groups

Queue Group: a group of subscribers to a subject where only one subscriber gets the message.
It's a work queue, and it has load balancing. Useful for scaling services.

If a subscriber is registered based on a queue name, it will always receive messages it is subscribed to.
However, if more subscribers are added to the same queue name — they become a queue group,
and only one randomly chosen subscriber of the queue group will consume a message.

No configuration required: to scale up, just spin up more consumers.

Queue group names are also hierarchical: `namespace.group.entity.>`.
Some server functionalities like queue permissions can use wildcard matching on them.

With JetStream, a stream can also be used as a queue:
set the retention policy to `WorkQueuePolicy`.

Geo Affinity:
When connecting to a globally distributed NATS super-cluster,
NATS will automatically route messages within the same cluster (unless failover kicks in).

## CLI

Install the NATS CLI tool:

```console
$ go install github.com/nats-io/natscli/nats@latest
```

Nats tool:

```console
$ nats sub <subject>
$ nats pub <subject> <message>
```

`nats` tool has contexts: like Docker contexts, that keeps your servers' credentials.










## JetStream

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

On top of JetStream, NATS provides:
* Key/Value store
* Object store (for binary files)

NOTE: NATS does not intend to compete with the rich feature set of in-memory databases.

*Streaming* is: temporal decoupling between the publishers and subscribers.
In NATS Core, subscribers only receive the message if they're actively connected.
A *durable subscriber* ("queue") holds the message until a subscriber comes.

However, queues are (in general) are not meant to be used as a mechanism for message replay; streams are.

A *stream* can replay messages on demand:
JetStream provides both the ability to consume messages as they are published (i.e. 'queueing')
as well as the ability to replay messages on demand (i.e. 'streaming').

### Replay
JetStream replay policies:

1. All: a complete replay. With two replay policies: "instant"
  (i.e. as fast as the consumer can take) and "original" (at the rate they were published: i.e. with simulated delays)
2. Last message in a stream
3. Last message for each subject (as streams can capture more than one subject)
4. Starting from a specific ssequence number
5. Starting from a specific time

### Retention
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
* Interest: work queue, but drop messages if no one's consuming (no interest)

With Work Queues, each message can be consumed only once:
this is enforced by only allowing one consumer to be created per subject, i.e. consumers' subject filters
must not overlap.

Note that limits always apply, even to a work queue.


### Consistency

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

Stream mirroring:
you can mirror a stream to between different domains in order to offer disaster recovery.

QoS: *at least once*.
Normally, NATS is reliable and duplicate-free, but there are some specific failure scenarios that could result in duplicates:
* in a publishing application believing (wrongly) that a message was not published successfully and therefore publishing it again
* in a client application's consumption acknowledgment getting lost and therefore in the message being re-sent to the consumer by the server.

QoS: JetStream supports *exactly once*,
but this involves:
* publisher: assigning unique ids to messages (header: `Nats-Msg-Id`)
* server: de-duplicating ids for a configurable rolling period of time
* consumer: double acknowledgment mechanism


### Streams
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


### Consumers
A consumer is a stateful view of the stream:
an interface to consume a subset of messages that keeps track of which messages
were delivered and acknowledged by clients.

Equivalent to Consumer Group in Kafka, or Durable Queue subscribed to a topic in RabbitMQ.

Client application can choose to be:
* Push consumers: new messages are pushed to the consumer as they arrive. Receive messages as fast as possible. No ack.
* Pull consumers: demand-driven, support batching, much ack. Provide horizontal scalability. Don't worry about partitions.

The trade-off: as fast as possible vs reliable.
NOTE: pull consumers don't mean delays because they use long-polling.

Acknowledgments:

* Some consumers support ACK of all messages up to a sequence number
* Some consumers require ACK of reception and processing of each message (with a timeout)
* You can send back negative ACK (retry)
* You can send *in progress* ACKs (to indicate that you need more time: avoid the timeout)

If a message is not acknowledged within a user-specified number of delivery attempts,
an advisory notification is emitted.

Consumers can also be ephemeral or durable:
* Durable: when an explicit name is set on the `Durable` field when creating the consumer, or when `InactiveThreshold` is set.
* Ephemeral: will not have persisted state or fault tolerance and will be automatically cleaned up (deleted) after a period of inactivity (no subscriptions)

Consumer configuration [see whole list](https://docs.nats.io/nats-concepts/jetstream/consumers#configuration)
* Durable: clients can reconnect and resume until the consumer is explicitly deleted
* InactiveThreshold: remove if inactive for that long
* Description: for humans. Useful for ephemeral consumers to indicate their purpose (because there's no durable name)
* MemoryStorage: keep in memory. Useful for ephemeral consumers to reduce I/O
* FilterSubjects: filter stream subjects, e.g. `[factory-events.A.*, factory-events.B.*]`
* DeliverPolicy: start from beginning? offset? time? new messages only? replay last message, or even last per subject?
* AckPolicy: explicit (require every message ack), none (naive mode), all (ack only the last message; all previous messages are automatically acknowledged).
* AckWait: timeout for consumer ack'ing the message. No ack? will be re-delivered. Also see Backoff.
* MaxAckPending: max messages in flight, un-acked.
  For push consumers, this is the only form of flow control.
* MaxDeliver: how many times to retry a message if timeout/negative-ack?
  Note: messages that have reached MaxDeliver will stay in the stream.
* Replicas: the number of replicas. Default: same as stream

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

Example:

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


### Key/Value
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

Example:

```console
$ nats kv add my-kv
$ nats kv put my-kv Key1 Value1
$ nats kv get my-kv Key1
$ nats kv del my-kv Key1
```


### Object Store
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








## Subject Mapping and Partitioning

Read more here: <https://docs.nats.io/nats-concepts/subject_mapping>

NOTE: transforms are not applied recursively.
In this case, only the first matching rule will be applied:

```
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
  dest: "orders.{{wildcard(2)}}{{wildcard(1)}}",
}
```

Transforms can be used to partition messages deterministically:
i.e. same subjects will be mapped to the same partition:

```console
$ nats server mapping "foo.*.*" "foo.{{wildcard(1)}}.{{wildcard(2)}}.{{partition(5,1,2)}}" foo.us.customerid
```

Partitions are useful when you need to scale consumers
but preserve message ordering from a specific entity (same as Kafka partitions).

### Weighted mappings
Traffic can be split by percentage from one subject transform to multiple subject transforms.

Use: For A/B testing or canary releases

```
myservice.requests: [
    { destination: myservice.requests.v1, weight: 98% },
    { destination: myservice.requests.v2, weight: 2% }
]
```




## Authentication

Client authentication: Token, Username/Password, TLS Certificate, NKEY with challenge (Ed25519), JWT OAUTH,  Auth callout (script).

You can use accounts for multi-tenancy: each account has its own independent 'subject namespace'.

## Connectivity

* Plain NATS connections
* TLS encrypted NATS connections
* WebSocket
* MQTT

Also bridges:

* Kafka
* JSM for RabbitMQ bridge
* More

