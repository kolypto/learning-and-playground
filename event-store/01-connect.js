// npm install --save @eventstore/db-client

const {
  EventStoreDBClient,
  jsonEvent, FORWARDS, START,
  ANY, NO_STREAM, STREAM_EXISTS,
  NoStreamError,
} = require("@eventstore/db-client");


// Init client
// const client = new EventStoreDBClient({
//   endpoint: "localhost:2113",
// });
const client = EventStoreDBClient.connectionString`esdb://localhost:2113?tls=false`;


// Append an event, load the list of events
async function simpleTest() {
  // Stream name
  // Use '-' to separate name from id. Projections will be able to pick it up then.
  const STREAM_NAME = "signin";

  // Clear it
  let res = await client.deleteStream(STREAM_NAME);

  // Create an event
  const event = jsonEvent({
    // Event id: UUID that uniquely identifies the event.
    // If you attempt to add two events with a same id quickly, only one copy will be appended!
    id: '99cefc12-702d-41ca-a850-ef5f1506d3d7', // OPTIONAL
    // Unique type name. Use explicit names, not class names, so that you can version.
    type: "UserSignedIn",
    // Event contents. JSON, or binary
    data: {
      username: 'kolypto',
    },
    // Additional information: correlation ids, timestamps, access information, etc.
    metadata: {},
  });

  // Append it
  const appendResult = await client.appendToStream(
    // Stream to append to
    STREAM_NAME,
    // Can save multiple events at once
    [event],
    {
      // What stream state you expect. When mismatched, an exception will be thrown.
      // Use to implement optimistic concurrency:
      // * NO_STREAM: expect that the stream does not exist
      // * STREAM_EXISTS: expect that it exists
      // * event.version: a specific version
      // * ANY: I dont' care
      expectedRevision: ANY,
      // Use a different set of credentials for appending
      // credentials: {username: 'lol', password: 'lol'},
    });

  // Read events from the stream.
  const events = client.readStream(
    // Read from:
    // * one stream by name; or
    // * "$all" to read all events. This will include system events: their names start with '$'. Ignore them.
    STREAM_NAME,
    {
      // Read: forwards or backwards
      // TIP: read one event backwards to find your position in the stream!
      direction: FORWARDS,
      // Read: from start, or from a specific position: e.g. if you last seen some specific event
      fromRevision: START,
      // Limit the number of events returned
      maxCount: 10,
  });

  // Events
  try {
    for await (const event of events) {
      console.log(event);
    }
  } catch (e) {
    // Handle errors
    // Result can be "StreamNotFound"
    if (e instanceof NoStreamError){
      return;
    }
    throw e;
  }

  /* Results:

  {
    event: {
      streamId: 'test_stream',
      id: '99cefc12-702d-41ca-a850-ef5f1506d3d7',
      revision: 26n,
      type: 'UserSignedIn',
      data: { username: 'kolypto' },
      metadata: {},
      isJson: true,
      created: 16401855351786328
    }
  }
  */

  // All events have a StreamPosition, which is the place of the event in the stream, represented by a big int (unsigned 64-bit integer)
  // and a Position that is the events logical position that is represented by CommitPosition and a PreparePosition.
  // This means that when reading events you have to supply a different "position" depending on if you are reading from a stream or the $all stream.
}

simpleTest().then(console.info).catch(console.error);
