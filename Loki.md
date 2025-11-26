# Loki Primer

## Loki Components

See: <https://grafana.com/docs/loki/latest/get-started/components/#query-frontend>

Loki architecture: write-path, read-path, backend
- write-path: stores logs
- read-path: queries logs
- backend: maintenance

Write path has 2 components:
- Distributor: HTTP server accepts logs & forwards them with replication and parallelization.
- Ingester: persists logs to storage (on write path)
  and returns recently ingested, in-memory log data for queries (on read path).

Read path:
- (optional) Query Frontend: accelerates queryer API endpoints: queue, cache, etc
- (optional) Query Scheduler: ensures query fairness across tenants
- Querier: APIs for LogQL. Fetches log data from both the ingesters and from long-term storage.
- Index Gateway: handles metadata queries, i.e. looks data up in the index and finds chunks

Back-end:
- Compactor: accelerates the index: downloads index files and re-groups them per day/tenant. Deletes old logs.
- Ruler: evaluates rule/alert expressions

NOTE on Filesystem support:
While ingesters do support writing to the filesystem through BoltDB,
this only works in single-process mode as queriers need access to the same back-end store and BoltDB only allows one process to have a lock on the DB at a given time.

## Storage

See: <https://grafana.com/docs/loki/latest/get-started/architecture/>

Loki stores all data in a single object storage backend: AWS S3 and other cloud storage providers.

Loki has two main file types:

* Chunks: contains log entries
* Index: table of contents to know where to find logs for specific set of labels


## Loki Deployment Modes

See: <https://grafana.com/docs/loki/latest/get-started/deployment-modes/>

Loki is a single-binary that contains all the microservices mentioned above. Specify which microservices to run: `-target`

* **Monolithic mode (`-target=all`)**.
  All microservices run inside a single process. Sufficient for volumes &lt; 20Gb/day.
* **Simple Scalable (SSD)**. Separates execution paths into read, write, and backend targets:

    > Distributor + Ingester; Querier + Query Frontend ; Compactor + Index + ... .

    Easy depoyment: `-target=write`, `-target=read`, `-target=backend`.
* **Microservices Mode**. Runs each component as distinct process.
  Provides more granularity but is also most complex.


## Labels

See: <https://grafana.com/docs/loki/latest/get-started/labels/>

In Loki, log lines are not indexed. Only labels are indexed.

Loki does not parse log messages! It only ingests the log lines you've provided.
However, some labels may be automatically applied: specified in the HTTP request.

Default label: `service_name`. Loki tries multiple names, or users `"unknown_service"` by default.

The Docker/Loki plugin adds the following labels by default: `filename`, `host`, `swarm_stack/service`, `compose_project/service`.
Custom labels can be added using `logging.options` (`log_opts` in Terraform).
See: [docker-driver: configuration: labels](https://grafana.com/docs/loki/latest/send-data/docker-driver/configuration/#labels).

Strategy: start with the smallest set of labels, i.e. create *low cardinality* labels.
Why? Because high cardinality will result in a huge index and thousands of tiny chunks and lead to performance degradation.

DO: aim to have 10-15 labels max; use long-lived values; create labels for actual queries.
DONT: don't add a label until you know you need it!

Use `loki --analyze-labels` to debug high-cardinality labels.

Example label pipeline:

```yaml
pipeline_stages:
    # Regexp extracts values using capture groups
    - regex:
    expression: "^(?P<ip>\\S+) (?P<identd>\\S+) (?P<user>\\S+) \\[(?P<timestamp>[\\w:/]+\\s[+\\-]\\d{4})\\] \"(?P<action>\\S+)\\s?(?P<path>\\S+)?\\s?(?P<protocol>\\S+)?\" (?P<status_code>\\d{3}|-) (?P<size>\\d+|-)\\s?\"?(?P<referer>[^\"]*)\"?\\s?\"?(?P<useragent>[^\"]*)?\"?$"
    # Only 2 labels are set
    - labels:
        action:
        status_code:
```

Example:

```yaml
- regex:
    expression: '(level|lvl|severity)=(?P<level>\w+)'
- labels:
    level:
```



## Config

See: <https://grafana.com/docs/loki/latest/configure/>

Useful CLI flags:

* `-config.file=path`: path to the config file
* `-config.expand-env=true`: expand `${VAR}` environment variables in the config.
  Provide defaults: `${VAR:-default_value}`.

Print the current config/defaults:

```console
$ loki -print-config-stderr -config.file=/etc/loki/local-config.yaml
```

The configuration file is huge: most likely you won't need all the fields.






## Minimal Config

```yaml
# Docs: https://grafana.com/docs/loki/latest/configure/
#
# Use this to print the current config:
#  $ docker exec -it loki /usr/bin/loki -config.file=/etc/loki/local-config.yaml -print-config-stderr


# Require X-Scope-OrgID for authentication (tenant).
# Without OrgID, it's single-tenant.
auth_enabled: false

server:
  log_level: info  # debug, info, warn, error

# Common configuration for all components
common:
  path_prefix: /loki
  replication_factor: 1
  ring:
    instance_addr: 127.0.0.1
    kvstore:
      store: inmemory  # single-instance mode
      # store: memberlist  # cluster: multiple members
  # Choose one.
  storage:
    # filesystem:
    #   chunks_directory: "/loki/chunks"
    #   rules_directory: "/loki/rules"
    s3:
      # S3 authentication without login/password: using an EC2 instance role. See `aws_instance.server.iam_instance_profile`
      s3: s3://${S3_REGION}/${S3_BUCKET_NAME}  # Interpolated by Terraform
      s3forcepathstyle: false


# Connect to an existing Loki instance (when using cluster/memberlist)
memberlist:
  join_members: ${JOIN_MEMBERS}  # Interpolated by Terraform: a JSON array of strings


# Component: Incoming logs
ingester:
  chunk_encoding: snappy  # Snappy is better than gzip
  ## Uncomment this to test chunk flushing: immediate flush
  # chunk_idle_period: 1m
  # max_chunk_age: 2m

# Component: Index maintenance & old logs deletion ("retention")
compactor:
  working_directory: /loki/compactor
  retention_enabled: true
  # shared_store: filesystem
  shared_store: s3
  delete_request_store: s3

# Limits: global and per-tenant limits
limits_config:
  # When to delete old logs?
  # Only applies if `compactor.retention_enabled=true`.
  retention_period: 30d  # 6 months retention





# Schema configuration - defines where indexes and chunks are stored.
schema_config:
  configs:
    # Old logs: old BoltDB (deprecated)
    - from: "2020-10-24"
      schema: v11
      store: boltdb-shipper
      object_store: filesystem
      index:
        prefix: index_
        period: 24h
    # New logs: new TSDB.
    # NOTE: we won't use it. But keep this example config for future set ups.
    - from: 2025-12-01  # Must be in the future on first deployment!!
      schema: v13
      store: tsdb
      object_store: filesystem
      index:
        prefix: index_
        period: 24h
    # Cloud logs
    - from: 2026-01-01 # Must be in the future on first deployment!!
      schema: v13
      store: tsdb
      object_store: s3
      index:
        prefix: index_
        period: 24h

# Storage. Defines multiple storages.
# Which one is picked -- is determined by the `schema_config.configs.*.object_store`
storage_config:
  tsdb_shipper:
    active_index_directory: /loki/tsdb-index
    cache_location: /loki/tsdb-cache
  filesystem:
    directory: /loki/chunks
  aws:
    s3: s3://${S3_REGION}/${S3_BUCKET_NAME}  # Interpolated by Terraform
    s3forcepathstyle: false


# Table manager: for legacy BoltDB schemas.
# TODO: Delete when on TSDB
table_manager:
  retention_deletes_enabled: true
  retention_period: 30d

# Component: Queries
querier:
  # Allow running multiple queries in parallel.
  # Prevents the "too many outstanding requests" problem.
  # WARNING: high RAM usage! Queries are expensive! You may accidentally OOM your instance!
  max_concurrent: 8  # default=4
```
