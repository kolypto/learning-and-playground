const {
    EventStoreDBClient,
    jsonEvent, FORWARDS, START,
    ANY, NO_STREAM, STREAM_EXISTS,
    NoStreamError,
} = require("@eventstore/db-client");


const client = EventStoreDBClient.connectionString`esdb://localhost:2113?tls=false`;

/* Persistent subscriptions: like subscriptions, but they store the last known position on the server side.
This allows subscribers to be load-balanced and process events in parallel. It enables the competing consumers pattern.

=== Consumer Groups ===
Load balancing is done in "consumer groups", where each consumer will get a portion of events.
It is possible to create multiple consumer groups: they will be completely independent from each other, with their own last known position.

Importantly, in a consumer group, events may get out of order within a certain window.

=== Acknowledge, Parking. ===
Clients must acknowledge messages as they are handled. If a message is not acknowledged, it's retried after a timeout.
If a message is retried more than `maxRetryCount`, it will be parked and processing will continue.

Parked messages: into a separate parked message stream:
    $persistentsubscription-{groupname}::{streamname}-parked
You can `Replay` the parked messages for that subscription: this will push them to subscribers before any new events on the subscription.
If you don't want to replay, delete the parked stream.

=== Checkpoints ===
Subscriptions write their checkpoints from time to time.
If a subscription is restarted (e.g. due to a Leader change), the persistent subscription will continue from the last checkpoint!
This means that some events may be received multiple times by consumers!

=== Consumer strategies ===
* RoundRobin (default): distributes events evenly. If a client's `bufferSize` is reached, the client is ignored until events are acknowledged.
* DispatchToSingle: dispatch to one client until `bufferSize` is reached, then select the next client.
* Pinned: (to be used with indexing projections) hash stream id and permanently assign hashes to clients
* PinnedByCorrelation: same as "Pinned", but uses correlationId to distribute

=== Ordering ===
Ordering is not guaranteed due to the possibility of messages being retried, or consumers handling events before others.
If you need an ordering guarantee, you should use a catch-up subscription instead and handle the checkpointing in your client code.
*/

async function main(){
    // Create a subscription group
    // Must have admin permissions. Can only create once.
    await client.createPersistentSubscription('test-stream', 'primary', {}, {});

    // Once you have created a subscription group, clients can connect:
    var sub = await client.connectToPersistentSubscription(
        'test-stream',
        'primary',
        {
            // `bufferSize`: how many messages the server should allow this client.
            BufferSize: 10,
            // ResolveLinkTos           Whether the subscription should resolve link events to their linked events.	false
            // StartFrom                The exclusive position in the stream or transaction file the subscription should start from.	null (start from the end of the stream)
            // ExtraStatistics          Whether to track latency statistics on this subscription.	false
            // MessageTimeout           The amount of time after which to consider a message as timed out and retried.	30 (seconds)
            // MaxRetryCount            The maximum number of retries (due to timeout) before a message is considered to be parked.	10
            // LiveBufferSize           The size of the buffer (in-memory) listening to live messages as they happen before paging occurs.	500
            // ReadBatchSize            The number of events read at a time when paging through history.	20
            // HistoryBufferSize        The number of events to cache when paging through history.	500
            // CheckPointAfter          The amount of time to try to checkpoint after.	2 seconds
            // MinCheckPointCount       The minimum number of messages to process before a checkpoint may be written.	10
            // MaxCheckPointCount       The maximum number of messages not checkpointed before forcing a checkpoint.	1000
            // MaxSubscriberCount       The maximum number of subscribers allowed.	0 (unbounded)
            // NamedConsumerStrategy    The strategy to use for distributing events to client consumers. See the consumer strategies in this doc.	RoundRobin
        }
    );

    for await (const event of sub) {
        // process
        // Must acknowledge when done
        await sub.ack(event); // use: Unknown, Park, Retry, Skip
        // also use nack() to not acknowledge (e.g. pass it on)
    }
}


main().then(console.info).catch(console.error);
