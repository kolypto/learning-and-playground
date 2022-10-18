const {
    EventStoreDBClient,
    jsonEvent, FORWARDS, START,
    ANY, NO_STREAM, STREAM_EXISTS,
    NoStreamError,
} = require("@eventstore/db-client");


const client = EventStoreDBClient.connectionString`esdb://localhost:2113?tls=false`;

/* Subscriptions: receive notifications about new events added to a stream.

You provide: a starting point, and an event handler.
*/

async function main(){
    // Send an event
    const event = jsonEvent({ type: "UserSignedIn", data: { username: 'kolypto' } });
    let res = await client.appendToStream('signin', [event]);
    const position = res.position.commit;

    // Subscribe to events
    await Promise.all([
        subscribeSinceBeginning(),
        subscribePosition(position),
        client.appendToStream('signin', [jsonEvent({ type: "lol", data: {} })]), // send another event
    ]);
}


async function subscribeSinceBeginning(){
    // Subscribe to a stream by name, or '$all'
    // This will subscribe to all events since the beginning
    const sub = client.subscribeToStream('signin');
    for await (const event of sub) {
        console.log(
            `subscribeSinceBeginning(): ${event.event.revision}@${event.event.streamId} ${event.event.type}`
        );
    }
}

async function subscribePosition(position){
    // Subscribe from a specific position
    const sub = client.subscribeToStream('signin', {
        // NOTE: positions are exclusive! You'll only receive new events after this position!
        // Use `END` to receive live updates, without going through history
        fromRevision: position,
    });
    for await (const event of sub) {
        console.log(
            `subscribePosition(): ${event.event.revision}@${event.event.streamId} ${event.event.type}`
        );
    }
}

async function subscribeAll(){
    // Example: Subscribe to all events, with filter prefix
    const subscription = client.subscribeToAll({
        // filter events by stream name: prefix, or regexp
        filter: streamNameFilter({ regex: "^account|^savings" }),
        filter: streamNameFilter({ prefixes: ["test-", "other-"] }),
        // filter event by types: prefix, or regexp
        filter: eventTypeFilter({ prefixes: ["customer-"] }),
        filter: eventTypeFilter({ regex: "^user|^company" }),
    });

    // NOTE: when events are few and far apart, it may take a while to go through them.
    // EventStore can give you a "heads up" every N events just to tell you it's alive:
    const filter = eventTypeFilter({
        regex: "^[^$].*", // exclude system events
        checkpointInterval: 1000,
        checkpointReached(_subscription, position) {
            console.log(`checkpoint taken at ${position.prepare}`);
        },
    });
}

main().then(console.info).catch(console.error);
