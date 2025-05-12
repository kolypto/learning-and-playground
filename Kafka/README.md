RedPanda
========

Docs date: 2025-05-13

Redpanda is a C++ port of Kafka without all the cancerous parts. Very quick to set up.

Run:

```console
$ docker compose up
```

Run rpk (RedPanda Cli):

```console
$ docker compose exec -it redpanda-0 rpk -X user=superuser -X pass=secretpassword cluster info

## or use env
$ docker compose exec -it redpanda-0 rpk -X list
$ docker compose exec -it -e RPK_USER=superuser -e RPK_PASS=secretpassword -e RPK_BROKERS=redpanda-0:19092 -e RPK_ADMIN_HOSTS=redpanda-0:9644 rpk cluster info

## or configure an rpk profile:
$ rpk profile create redpanda-0 -X user=superuser -X pass=secretpassword -X brokers=redpanda-0:19092
```

Login to console: <http://localhost:8080/> superuser:secretpassword


# rpk

Alias:

```console
$ alias rpk="docker compose exec -it redpanda-0 rpk"
```

Create topic:

```console
$ rpk topic create chatroom
$ rpk topic describe chatroom
```

Topic configuration: <https://docs.redpanda.com/current/reference/properties/topic-properties/>

Produce & consume a message:

```console
$ rpk topic produce chatroom
<keep typing><Enter>
Ctr-D

$ rpk topic consume --num=1 chatroom
```

If you read without a consumer group, you'll be reading all messages: no offset it saved.

Consume from a group.
A consumer group is used to share workloads and balance consumption. It records last read offset.

```console
$ rpk topic consume -g groupname
```

# HTTP Proxy

REST api to access Kafka topics & messages.

```console
$ http GET http://localhost:18082/topics
[
    "chatroom",
    "logins",
    "transactions"
]

$ http -p HBhb POST http://localhost:18082/topics/chatroom records[0][value]="one two three" Content-Type:'application/vnd.kafka.json.v2+json'
{
    "records": [
        {
            "value": "one two three"
        }
    ]
}

{
    "offsets": [
        {
            "offset": 25,
            "partition": 0
        }
    ]
}

$ http GET http://localhost:18082/topics/chatroom/partitions/0/records offset==25 timeout==1000 max_bytes==100000 Accept:'application/vnd.kafka.json.v2+json'
[
    {
        "key": null,
        "offset": 25,
        "partition": 0,
        "topic": "chatroom",
        "value": "one two three"
    }
]
```

More commands: <https://docs.redpanda.com/current/develop/http-proxy/>
* Create a consumer
* Subscribe to a topic
* Commit offset
* Use partitions













# Develop

## Producer

Every event consists of a key and value.

Producers publish events to a specific partition (not only to a topic!).

* If the key is not provided, partitions are selected round-robin.
* If a key is provided, its hash is used to *modulate* across the number of partitions.

Properties:

* `acks`: the number of acknowledgments the producer requires the leader to have received
before considering a request complete. This controls the durability of records that are sent.

    * `acks=0`: The producer doesn't wait for any response from the broker and doesn't retry.

        This increases throughput and lowers latency at the expense of durability:
        i.e. "send things faster, ok if some are lost".
        Risk: data loss if the broker crashes.

        It's riskier than `acks=1` because it's "fire-and-forget": the producer sends the message and moves on blindly.
        If the message is lost in transit, or the broker is down, the producer won't know.
        There’s no retry mechanism based on acknowledgment failures, because the producer assumes success immediately.
        So, even network blips or unavailable brokers can cause silent data loss.

        Use case: IoT devices where updates are periodic and stateless.

    * `acks=1`: The producer waits for one ack from the leader only: not from the followers.

        This ensures that at least the leader broker has received and written the message.
        If the leader fails before other replicas are updated, the data may still be lost.

        This option is a balance between throughput, latency and durability.
        Replication is not guaranteed because it happens in the background.

        Risk: data loss if the broker crashes before followers can replicate.

    * `acks=all`: The producer waits for the majority of replicas to ack the message.

      Same as `acks=-1`.

    With `acks=all`, every write is fsynced by default: this increases durability at the expense of lower throughput and increased latency.
    Here RedPanda differs from Kafka. In Kafka, there's no fsync guarantee.
    Use `write.caching=true` on a topic to relax the rule: no fsync results in better performance at the expense of durability.
    Only enable write caching on workloads that can tolerate some data loss in the case of multiple, simultaneous broker failures.
    Leaving write caching disabled safeguards your data against complete data center or availability zone failures.

* `retries`: how many times a message is retried if the broker fails to acknowledge it.

  Default: `retries=0`. No retry at all.

* `max.in.flight.requests.per.connection`: how many unacknowledged messages can be sent to the broker simultaneously at any given time.

  Use `max=1` to send messages strictly one by one. Use `max>1` to increase throughput but adds a risk of message reordering if retries are enabled.

  Default: `max=5`

* `enable.idempotence`: ensure that messages are not duplicated and not delivered out of order:
  the producer ensures that exactly one copy of every message is written to the broker.

  When disabled: a network glitch may cause the producer to retry and produce duplicate messages.
  Concurrent retries also can cause messages being sent out of order.
  Other reasons: transient errors like brokers not being available or not enough replicas exception.

  Internally, this is implemented by assigning a unique number (producer id + sequence number) to every message.
  The broker checks that the number is larger than the previous one and, if not, discards the message.

  To guarantee true idempotent behavior, you must set `ack=all` to ensure that all brokers have recorded the message in order,
  even in the event of node failures.

  Idempotence is only guaranteed within a session: i.e. a producer connection.

### Message Batching

Message batching:
an efficient way to save on both network bandwidth and disk size, because messages can be compressed easier.
Batching accumulates messages (up to `max.request.size`) before sending them as a single request.
The producer automatically puts messages being sent to the same partition into one batch.

* `linger.ms`: the producer will wait up to so many ms before sending a batch.

  Set `linger.ms > 0` if you're willing to tolerate some latency and send fewer, more efficient, messages.
  Setting it to `linger.ms = 0` still has a high chance of messages coming around the same time.
  Default: `0`
* `batch.size`: max batch size, bytes. If full, is sent immediately.
  Default: `16384`
* `buffer.memory`: total amount of memory available to the producer for buffering.
  If messages come faster than the producer can deliver to the broker, they may block (for at most `max.block.ms`)
  or throw an exception (when blocking for longer than `max.block.ms`).

The number of simultaneous requests is still limited by `max.in.flight.requests.per.connection`.

More settings:

* `compression.type`: the compression method to use on batches. Default: `none`
* Serializer: JSON serializer would send a large schema with every message, whereas protobuf and avro has centralized schema.

Broker timestamps:
Because the producer's time may skew, Redpanda records the broker’s system date and time in the `broker_timestamp` property of the message.

### Idempotence

When a producer writes messages to a topic, each message should be recorded only once in the order in which it was sent. However, network issues such as a connection failure can result in a timeout, which prevents a write request from succeeding. In such cases, the client retries the write request.

Since there is no way to tell if the initial write request succeeded before the disruption, a retry can result in a duplicate message. A retry can also cause subsequent messages to be written out of order.

To enable idempotence:
* Producer: set `enable.idempotence=true`
* Cluster: set `enable.idempotence=true` (enabled by default)



## Consumer

All messages are organized by *topic* and distributed across multiple partitions, based on a partition strategy (e.g. round-robin).

Offset: a unique sequence number assigned to every message in a partition: 0, 1, ...

As a consumer reads messages from Redpanda, it can save its progress by “committing the offset” (known as an offset commit).
Each committed offset is stored as a message in the `__consumer_offsets` topic.

### Consumer Group

A consumer group: a group of consumers that work together to consume data from topics.
The partitions are divided among them: each partition is assigned a single consumer that'll read & commit offsets.

For example, if a topic has 12 partitions, and there are two consumers, each consumer would be assigned six partitions to consume.
If a new consumer starts later and joins this consumer group, a rebalance occurs, such that each consumer ends up with four partitions to consume.
You specify a consumer group by setting the `group.id` property to a unique name for the group.

NOTE: More advanced consumers can read data from Redpanda without using a consumer group by requesting
to read a specific topic, partition, and offset range. This pattern is often used by stream processing systems
such as Apache Spark and Apache Flink, which have their own mechanisms for assigning work to consumers.
To do this, bypass the "consumer group" API and read partitions directly; commit offsets to RedPanda or elsewhere.

Consumers don't have to read from the Leader only: they can read from *followers*, i.e. closest read replicas.

### Offsets

You can commit offsets manually: makes sense to do it after a DB transaction is committed.
The `commitSync()` has higher latency — but it will wait for an ack before consuming more messages and retry in case of failure.
The `commitAsync()` has lower latency: it does not wait and has no automatic retry — but it has a callback where you can handle it yourself.

The purpose of a commit is to save consumer progress.
More frequent commits reduce the amount of data to re-read after an application restart — but at high commit rates
this workload can itself become a bottleneck.

When a consumer group commits, RedPanda writes all their commits into a single partition (to preserve ordering).
If you have multiple consumer applications, create a consumer group for each:
then you'll benefit from having separate partitions. Otherwise they'll all commit to a single partition.

### Best practices:

* Monitor commit latency to ensure commits are timely. If you notice performance issues, commit less frequently.
* Assign a unique consumer group to each application to distribute the commit load across all partitions.
* If you have many consumers, increase the `heartbeat.interval.ms` to reduce the unnecessary overhead:
  3000 consumers can generate 6000 heartbeats per second, which is a lot.







# Kafka Partition Strategy
Kafka groups related messages into topics, allowing consumers and producers to categorize messages.
At a lower level, topics can be broken down into partitions, which is how a single topic can span across multiple brokers.
Partitions increase parallelization and allow Kafka to scale.

Each consumer group has a unique consumer offset representing the point in the topic where the consumer group is currently reading.
When a consumer starts consuming messages from a topic, it’ll use the group.id to determine the consumer offset from which to start consuming.

Kafka lets you choose how producers should publish messages to partitions and how partitions are assigned to consumers.
However, there are multiple ways to route messages to different partitions. Each strategy comes with its own pros and cons,
and learning which strategy is optimal for a certain circumstance differentiates Kafka experts from newbies.

Producer partition strategies: i.e. how messages are assigned to partitions:
* Default: by key hash; round-robin for null messages
* Round-Robin: in turns, regardless of key hash
* Uniform sticky: round robin, but batch up some messages (until the `batch.size` is met or `linger.ms` time is up) to reduce latency.
* Custom: custom key-to-partition routing strategy

Consumer assignment strategies: i.e. how consumers get a partition:
* Range assignor: (Total number of partitions) / (Number of consumers) partitions are assigned to each consumer.
  The aim is to have co-localized partitions: assign the same partition number of different topic to the same consumer.
  E.g. partition#0 of TopicA and partition#0 of TopicB to the same consumer.
* Round-Robin: Partitions are picked individually and assigned to consumers in any rational order, in a circular fashion.
* Sticky assignor: same as round robin, but preserves as many existing assignments as possible.
  The aim is to avoid/reduce partition movement during rebalancing.
* Custom assignor: custom logic

## Producer Partitioning

* Default (Key Hash)

  When the key is null, the record is sent randomly to one of the available partitions of the topic.
  If a key exists, Kafka hashes the key, and the result is used to map the message to a specific partition.

  This ensures that messages with the same key end up in the same partition.
  However, only until the number of partitions change: messages with the same key might get written to a different partition.

* Round Robin

  Use this approach when the producer wants to distribute the writes equally among all partitions.
  This distribution is irrespective of the key’s hash value, so messages with the same key can end up in different partitions.

  This strategy is useful when the workload becomes skewed by a single key.

  While round-robin spreads records evenly among the partitions,
  it also results in more batches that are smaller in size, leading to more requests and queuing as well as higher latency.

* Uniform Sticky

  It's like round-robin, but enables larger batches:
  it chooses a round-robin partition, then fills up a batch until the `batch.size` is full or `linger.ms` is up.

  This reduces overall latency when the throughput is high — but obviously, delays every message for up to `linger.ms` time.

  After sending a batch, the sticky partition changes. Over time, the records are spread out evenly among all the partitions.

  Records with the same key are not guaranteed to end up in the same partition.

* Custom Partitioner

  Manually choose the partition to produce to.

  Useful when you want to selectively route some messages to a dedicated partition,
  or on the contrary — spread them in some manner across many partitions.

  This strategy can be combined with others:
  e.g. only request a specific partition number for messages that match your criterion.

## Consumer Partitioning
In Kafka, many producers usually write to a topic, and a single consumer wouldn't keep up.
To scale consumption from topics, Kafka has *Consumer Groups*: a bunch of consumers.
When they subscribe to the same topic, the group rebalances to make sure that each consumer
receives messages from a different set of partitions in the topic thus distributing data among themselves.

One partition can only be assigned to a single consumer.
So when scaling, if you have more consumers than partitions, some of them will remain idle.
However, idle consumers act as failover: it can quickly pick up the slack if an existing consumer fails.

* Range assignor.

  Default strategy that works on a per-topic basis:
  Divide evenly, in alphabetic order.

  It's not completely even: the first few consumers will have an extra partition.

* Round robin assignor.

  Assigns in a round-robin fashion.
  it aims to maximize the number of consumers used, but it can't minimize partition movement.

* Sticky assignor.

  Like round-robin, but it also preserves as many existing assignments as possible when reassignment of partitions occurs.

* Custom assignor.

  Custom selective logic.

When consumers are added/removed, the group is rebalanced: partitions are re-assigned to consumers.
There are two types of rebalances:

* Eager rebalancing with stop-the-world: will cause a small window of downtime.
* Cooperative (incremental) rebalancing: performs the rebalancing in multiple phases,
  reassigning a small subset of partitions from one consumer to another.

By default, consumers have dynamic identities within the group.

It is possible to make a consumer a static group member by configuring it with a unique `group.instance.id` property:
when it restarts or shuts down, it remains a member, and is reassigned the same partitions when it rejoins.
This is useful for stateful applications where the state is populated by the partitions assigned to the consumer.















## Data Transforms

RedPanda can transform data directly in the broker:
e.g. take data from an input topic and map it to one or more output topics.

Each transform function reads from a specified input topic and writes to a specified output topic.
A transform can read exactly one record — and produce 0+ records:
i.e. it can write more than one record, or decide not to write at all.

A new transform function reads the input topic from the latest offset: i.e. it won't read older records.

A record is processed after it has been successfully written to disk on the input topic.
Because the transform happens in the background after the write finishes, the transform doesn’t affect the original produced record,
doesn’t block writes to the input topic, and doesn’t block produce and consume requests.
It also has a higher latency.

In the event of a leader crash, it is likely that the new leader will reprocess some events because
the offset might have not been committed.

Transforms use Wasm engines:
runs a WASM VM on the same CPU core (shard) as these partition leaders to execute the transform function.
When you deploy a data transform to a Redpanda broker, it stores the Wasm bytecode and associated metadata
(input and output topics and environment variables). The broker then replicates this data across the cluster using internal
Kafka topics. When the data is distributed, each shard runs its own instance of the transform function.

Limitations:
* A transform can read exactly one record.
  For aggregations, joins, or complex transformations, consider using Redpanda Connect or Apache Flink.
* Transforms have to access to disk or network
* Up to eight output topics are supported.
* Transform functions have at-least-once delivery.
* Env variables: combined length is limited to 2000 bytes

First, enable data transforms and restart all brokers:

```console
$ rpk cluster config set data_transforms_enabled true
$ rpk cluster config status
NODE  CONFIG-VERSION  NEEDS-RESTART  INVALID  UNKNOWN
0     2               false          []       []

$ rpk redpanda stop
$ rpk redpanda start
```

Create a project on your host machine:

```console
$ rpk transform init --language=tinygo --name=trchat
```

(it may need to use your local Docker)

Example: copy records from one topic to another:

```go
package main

import (
	"github.com/redpanda-data/redpanda/src/transform-sdk/go/transform"
)

func main() {
	transform.OnRecordWritten(copyRecordsToOutput)
}

// This will be called for each record in the input topic.
// The records returned will be written to the output topic.
func copyRecordsToOutput(event transform.WriteEvent, writer transform.RecordWriter) error {
	return writer.Write(event.Record())
}
```

Example: Validate JSON, send to DLQ topic:

```go
import (
	"encoding/json"
	"github.com/redpanda-data/redpanda/src/transform-sdk/go/transform"
)

func main() {
	transform.OnRecordWritten(filterValidJson)
}

func filterValidJson(event transform.WriteEvent, writer transform.RecordWriter) error {
	if json.Valid(event.Record().Value) {
		return w.Write(e.Record())
	}
	// Send invalid records to separate topic
	return writer.Write(e.Record(), transform.ToTopic("invalid-json"))
}
```

Build it and deploy:

```console
$ rpk transform build
$ rpk transform deploy --input-topic=input-topic --output-topic=output-topic
```

You can deploy a transform that will reprocess old messages:

```console
$ rpk transform deploy --from-offset +0
$ rpk transform deploy --from-timestamp @1617181723
```

You can deploy a wasm file using an URL:
this is useful for automated deployments:

```console
$ rpk transform deploy --file=https://my-site/my-transform.wasm
```

Best practices:
* Don't just fail on bad messages: send them to a DLQ!






## Transactions

Transactions: you can fetch messages starting from the last consumed offset and transactionally process them one by one,
updating the last consumed offset and producing events at the same time.

Transactions guarantee that either all messages are committed or none.

Cluster config: by default, `enable_transactions=true`.
However, in the following use cases, clients must explicitly use the Transactions API to perform operations within a transaction.

The required `transactional.id` property acts as a producer identity: it enables reliability semantics
that span multiple producer sessions by allowing the client to guarantee that all transactions
issued by the client with the same ID have completed prior to starting any new transactions.

The two primary use cases for transactions are:
* Atomic (all or nothing) publishing of multiple messages
* Exactly-once stream processing

Tips:
* A transaction can span partitions from different topics.
* If a topic is removed while a transaction is active, the transaction can drop deleted partitions alright
* Ongoing transactions can prevent consumers from advancing — for up to `transaction.timeout.ms` ms.
* There is a limit on the number of simultaneous transactions: `max_transactions_per_coordinator`.
  When exceeded, Redpanda terminates old sessions. The idle producer's batches will be rejected
  with "invalid producer epoch" or "invalid_producer_id_mapping".











# Schema Registry

Messages contain raw bytes, but schemas enable producers to share the information needed to de/serialize those messages.

Schemas are versioned.
When a producer or a consumer requests to register a schema change,
the registry checks whether schema is compatible or returns an error.

Terminology:
* Schema: defines the structure of data
* Subject: a logical grouping together of multiple schema versions.
  When a schema is updated (new version), it can be registered under the same subject.
* Serialization format: Avro, JSON, Protobuf
* Normalization: converting a schema to canonical form.
  When a schema is normalized, it can be compared and considered equivalent to another schema that may contain minor syntactic differences.

Backend: `_schemas` topic
By default, `_schemas` is protected from deletion and configuration changes by Kafka clients.
See the `kafka_nodelete_topics` cluster property.

Schema registry uses the default port 8081.

## Wire Format

With Schema Registry, producers and consumers can refer to a schema by id:

```
bytes
[0  ]  NULL byte
[1-5] int32 schema_id
[...] rest of the message
```

Producer: send a message in this *wire format*.
The serializer will check whether the `schema_id` for the given subject exists.
If not, the serializer registers it and collects the resulting schema ID in the response.

The *subject name* is derived using several strategies:
* Topic Name Strategy: `<topic>-key` or `<topic>-value` (the default)
* Record Name Strategy: a fully qualified record name: `<record-name>`
* Topic Record Name Strategy: `<topic>-<record>`

Consumer: if the message has the magic byte, it's passed to the deserializer.
It finds the schema and deserializes the message.

### Example:

Create a topic:

```console
$ rpk topic create products
```

Now register this schema in the Console using id=1, strategy=topic, topic "products", value (subject name: "products-value"):

```protobuf
syntax = "proto3";

message Product {
  int32 ProductID = 1;
  string ProductName = 2;
  double Price = 3;
  string Category = 4;
}
```

Now you have a schema with `id=1`.
Produce a message using JSON:

```console
$ rpk topic produce product --format '%v\n' --schema-id=1
{"ProductID":127, "ProductName": "Car", "Price": 999, "Category": "sedan"}
```

or of course you can produce them using protobuf:

```console
$ rpk topic produce product --format '%v{hex}\n' --schema-id=1
190000000000388f402205736564616e087f1203436172   --- doesn't work in CLI
```

Now see it stored in binary:

```console
$ rpk topic consume product
{
  "topic": "product",
  "value": "\u0000\u0000\u0000\u0000\u0001\u0000\u0019\u0000\u0000\u0000\u0000\u00008\ufffd@\"\u0005sedan\b\u0012\u0003Car",
  "timestamp": 1747080980940,
  "partition": 0,
  "offset": 0
}
```

and decode it using schema registry:

```console
$ docker compose exec -it redpanda-0 rpk topic consume product --use-schema-registry=value
{
  "topic": "product",
  "value": "{\"ProductID\":127, \"ProductName\":\"Car\", \"Price\":999, \"Category\":\"sedan\"}",
  "timestamp": 1747080980940,
  "partition": 0,
  "offset": 0
}
```
