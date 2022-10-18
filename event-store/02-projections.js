const {
    EventStoreDBClient,
    jsonEvent, FORWARDS, START,
    ANY, NO_STREAM, STREAM_EXISTS,
    NoStreamError,
} = require("@eventstore/db-client");


const client = EventStoreDBClient.connectionString`esdb://localhost:2113?tls=false`;

/*
Projections: append new events, or link existing events to streams, in a reactive manner.

NOTE: Many problems are not a good fit for projections and are better served
by hosting another read model populated by a catchup subscription!

Projection: a continuous query.
You can choose whether it applies to old results or future results.

Projections emit events as a rection to events that they process.
This is *write amplification*: it creates additional I/O load on the server.

Limitation: a projection exclusively owns their stream. If your application appends to such stream, projection will break!
Why? Because they lose predictability. By controlling the stream, it knows where it left off, and can verify that everything is in order.
 */



async function main(){
    const STREAM_NAME = 'signin-1';

    // Create an event
    const event = jsonEvent({type: "UserSignedIn", data: {username: 'kolypto' }});
    await client.appendToStream(STREAM_NAME, [event]);

    // System projections:
    // * $et: events by type
    // * $bc: by correlation id
    // * $ce: events by stream id. Stream id "account-1" sends events to '$ce-account'
    // * $category: same as $ce
    // Info: https://developers.eventstore.com/server/v21.10/projections/system-projections.html
    const signins = client.readStream('$ce-signin', { direction: FORWARDS, fromRevision: START, resolveLinkTos: true });
    for await (const event of signins){
        console.log(event.event);
    }

  // User-defined projections:
  //    https://developers.eventstore.com/server/v21.10/projections/user-defined-projections.html#user-defined-projections-api
  // Debugging:
  //    https://developers.eventstore.com/server/v21.10/projections/debugging.html#logging-from-within-a-projection
  // Creating JS projections programmatically:
  //    https://developers.eventstore.com/clients/grpc/projections.html#required-packages
}

main().then(console.info).catch(console.error);
