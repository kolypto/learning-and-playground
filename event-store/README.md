Event Store
===========

Core features: guaranteed writes, concurrency model, high availability cluster, granular stream, immutable data, stream APIs.

Installation
------------

Docker: docker pull eventstore/eventstore

Also: debian packages: https://developers.eventstore.com/server/v20.10/installation/linux.html

Run in Docker:

```console
$ docker run -it --rm eventstore/eventstore --help
$ docker run -it \
    --name esdb-node -p 2113:2113 -p 1113:1113 \
    eventstore/eventstore --insecure --run-projections=All
```

* `--insecure` disables authentication
* `--run-projections=All` Enables the running of projections. System runs built-in projections, All runs user projections. (None, System, All)

Connection string for gRPC clients:

> esdb://localhost:2113?tls=false

Fine-tune performance: `STREAM_INFO_CACHE_CAPACITY`, `READER_THREADS_COUNT`, `WORKER_THREADS`

Access
------

Two default users: `$admin` and `$ops`.

`$admin` can read/write protected streams (names start with `$streamname`: usually, system streams) and run operational commands

`$ops` same, but cannot manage users and read from system streams

Admin Panel
-----------

http://localhost:2113/

Operation
---------

* 01-connect.js
* 02-projections.js
* 03-subscriptions.js
* 04-persistent-subscriptions.js

Scavenging Events
-----------------

An operation required for freeing up space after deleting events. It reclaims disk space.

NOTE: after a scavenge has run, you cannot recover any deleted events!

EventStore does not run scavenge automatically! Set up a scheduled task for this.

> curl -i -d {} -X POST http://localhost:2113/admin/scavenge -u "admin:changeit"

or use the admin panel.

It is safe to run it when EventStore is running, but event processing speed may decrease.

Backup
------

Just copy files.

More instruction: https://developers.eventstore.com/server/v21.10/operations/database-backup.html#backup
