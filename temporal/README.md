# Temporal

Date: 2026-04

Durable execution: Temporal ensures that, once started, a function executes to completion, whether that takes minutes, or weeks.

# Quickstart

Note that Temporal uses "admin tool" to migrate your DB before it starts.
Here's a Docker with everything set up already:

```console
$ mise install  # python, temporal cli
$ pip install temporalio
$ docker compose up -d
```

# Core Objects

## Activity

*Activity*: normal function that executes a single well-defined action.
If an Activity fails, Temporal automatically retries it based on your configuration.

Arguments: position-based only.
Best practice: use one dataclass. This allows you to add fields without breaking the signature.
Limit: 2MB per argument (imposed by gRPC). But large payloads may slow things down because they are recorded in the *Workflow Execution Event History*.

Facts:

- Activity implementation code should be *idempotent*!
- Every activity has a `name`; Temporal calls them *Activity Type*.

Activities are spawned by workflows: `start_activity()` creates `ScheduleActivityTask` Commands.
The returned value is an `ActivityHandle`: extends `asyncio.Task` and supports *some* task features:
e.g. you can multiplex them and wait on multiple at the same time; cancel activities.

The `execute_activity()` is a shortcut that waits on its result: sugar on the sequential case.

### Failures and Timeouts

Available timeouts: 

* Schedule-To-Close Timeout: max overall Activity Execution time: scheduled + running
* Start-To-Close Timeout: max single Activity Task Execution time.
* Schedule-To-Start Timeout: max time between Activity Task scheduled and Worker starts executing it (lag, delay)

An activity execution must have either #1 or #2 set.

If an activity fails, it's retried using the default retry policy.
This is unlike workflows — which never fail, and are not retried by default.
You can raise a `non_retryable` error to stop trying.

The workflow does not receive these intermediate errors: it only receives one when an activity stops trying (final fail).

### Heartbeat
If an activity does not send a heartbeat to the Temporal Service within Heartbeat Timeout, it's considered crashed and may be rescheduled.
Heartbeats may be throttled by the Worker (in case you send too many of them).

This is bound to Activity Cancellations:
an activity receives a Cancellation only when it heartbeats! 
Activities that don't Heartbeat can't receive a Cancellation.

A heartbeat may contain details: its current progress. 
If an Activity gets retried, the Activity can access the details from the last Heartbeat that was sent to the Temporal Service.

### Standalone Activities
Release status: pre-release

*Standalone Activities*: can be run independently, without a Workflow.

### Async Activities

An async activity receives its state updates from an external source: e.g. webhook.
Use it when:
* The external system may fail to deliver a Workflow signal: you're then relying on the activity timeout feature
* You need heartbeats

In short: 
* The activity will be called with a Start-to-Close timeout
* Your activity will only notify an external user/system that they need to do something
* You create a handler to receive input from them
* In the handler, get the activity handle
* Call `.heartbeat()` or `.complete()` on the handle to change its state


## Workflow

*Workflow*: orchestrates activities. 
They are resilient and can keep running for years.

Workflow code must be deterministic because the Temporal Server may replay your Workflow to reconstruct its state:
no randomness, no side-effects, no external calls.
In short, it MUST make the same Workflow API calls in the same sequence, given the same input. 

If you change anything that produces *Commands*, use *versioning*.

The SDK provides replay-safe alternatives:

- `workflow.logger()` — SDK will suppress duplicates during replays
- `workflow.random()`, `workflow.uuid4()` — deterministic random
- `workflow.now()` — the time of the last Workflow Task
- `workflow.unsafe.is_replaying` — guard code that should only run on the first execution
  (emitting metrics, external notifications, etc)

To start a workflow:
- Choose a queue (with a worker running on it!)
- Generate a unique workflow id.
  Recommended: map it to a business process, e.g. order id or customer id

If you used `start_workflow()`, you can get the workflow id.
When it completes successfully, you'll also have the "run id".
To get the results from a workflow execution, use either.

Using this handler, you can interrupt the Workflow:

* `cancel()` cancels it gracefully: the Workflow gets the signal and can do clean-up
* `terminate()` stops it forcefully: the Workflow code gets no chance to handle termination
* reset: terminate the current workflow and start a new Workflow Execution from a point in history.
  Use it when a Workflow is blocked.

Continue-as-New: use `workflow.continue_as_new()` to close this Workflow Execution successfully
and start a new one from the same point — but with a clean history.
Use case: checkpoints. Use when your Workflow gets too long or approaches certain scaling limits.

### Failures
Core design principles: workflows never fail. Any error just fails the Workflow Task and be retried.
The default policy: retry failed activities until a timeout; activities do not return a failure until
this or another non-retryable condition is met.

You can deliberately fail a workflow by raising `ApplicationError`.

By default, a failed workflow is not retried! It fails — and remains failed.
You must specify a `retry_policy` per invocation in order to get a retryable workflow.

### Queries, Signals, Updates
A Workflow can act like a stateful web service that receives messages: Queries, Signals, and Updates.

#### Query

Queries is the preferred method of accessing the state and results of workflow executions.
A Query can inspect the workflow but cannot mutate its state.
Examples
- Progress report (e.g. step/total)
- Debugging

A query can even have arguments: thus, looking like an actual API.

#### Signal

A Signal is an asynchronous message sent to a running Workflow to change its state and control its flow.
It can mutate the workflow state, but it shouldn't return any values.
Temporal replays signals in the correct sequence -- achieving determinism for queries and workflows.

Signal is fire-and-forget — no response, no rejection. Update is a synchronous round-trip:
it's a synchronous request that can return a result.
The sender must wait until the Worker returns a result -- or rejects the Update by returning an error.

You can even do a `while` loop to wait on a queue where updates are pushed: such a Workflow will react:

```python
class ExampleWorkflow:
  def __init__(self):
      self._queue: asyncio.Queue[ApprovalResult] = asyncio.Queue()
  
  @workflow.signal
  async def approve(self, result: ApprovalResult):
      await self._queue.put(result)  # enqueue each arrival
  
  @workflow.run
  async def run(self):
      while True:
          result = await self._queue.get()  # blocks until next signal
          await workflow.execute_activity(notify_reviewer, result, ...)
          if len(self._approvals) >= self._required:
              break
```

#### Update

An Update is like a signal — but it returns a result, and is not retried:
retrying is then up to the caller.

Updates can have a validator:
use validators to reject an Update before it is written to History.
Validators are optional: if you don't need to reject Updates, you can skip them.

To handle incoming signals and updates, `workflow.wait_condition()` is the right tool:
it will keep calling a function until it returns a `true`.
Use `workflow.current_update_info` to obtain information about the current Update. 

If an Update starts an activity — the caller *has to wait* until it completes.
This is unique to updates: signal handlers were non-blocking.

```python
@workflow.update
async def approve(self, result: ApprovalResult) -> str:
    self._approvals.append(result)
    
    # You can run an activity here.
    # Why? To make sure that something external has really happened.
    receipt = await workflow.execute_activity(
        record_approval, result,
        start_to_close_timeout=timedelta(seconds=10),
    )
    
    return receipt  # returned to the caller once activity completes
```

The natural design gravity is to keep the main logic in the Workflow `run()`, method.
Updates can invoke activities, but they're supposed to *influence* the main flow, not replace it. 
The update handler is a doorbell. run() is the house.
So the common pattern is:

* Update: uses an activity to persist a document
* Workflow: uses `wait_condition()` to wait on the persisted document

```python
@workflow.update
async def submit_document(self, doc: Document) -> str:
    # activity here = "durably record this, then confirm to caller"
    doc_id = await workflow.execute_activity(
        persist_document, doc,
        start_to_close_timeout=timedelta(seconds=10),
    )
    self._pending.append(doc_id)
    return doc_id  # caller gets confirmation it was persisted

@workflow.run
async def run(self):
    await workflow.wait_condition(lambda: len(self._pending) > 0)
    # main logic: orchestrate, branch, sequence
    for doc_id in self._pending:
        await workflow.execute_activity(process_document, doc_id, ...)
```

Update handlers are not retried by Temporal, but they can be retried by the caller.
This is unlike Signals, which are retried.

Activities inside the update, however, are retried normally.
Activities *are not re-run* if the update handler is retried: history protects it! Idempotency here.

The workflow can wait on a condition to become true using `wait_condition()` with a lambda function.

#### Notes on Signals / Updates

The rule of thumb:

* Signal handler: mutate state only, no awaiting activities
* Update handler: can do real async work, return result to caller
* `run()`: orchestrates, waits on conditions or queues fed by signals

To send a Query, Signal, or Update, you need the workflow handle.

You can also use "Signal-With-Start" to send a Signal to a Workflow Execution, starting the Execution if it is not already running. 
Use case: idempotent event ingestion, where the workflow may not have started yet.

Also, "Update-With-Start" is available.

#### Concurrency

Concurrency note: in Python SDK, the instance of a Workflow class is long-lived: the class is reconstructed on a Worker
*and remains loaded*. This means that if you have multiple signals updating the object concurrently, 
you must use locking primitives (e.g. Lock) to synchronize their accesses to `self`!

The docs show this pathological case: multiple executions of this Signal may cause a race condition on `self`:
`self.x` and `self.y` may have values from different Signal executions => different activity executions:

```python
@workflow.defn
class MyWorkflow:

    @workflow.signal
    async def bad_async_handler(self):
        data = await workflow.execute_activity(
            fetch_data, start_to_close_timeout=timedelta(seconds=10)
        )
        self.x = data.x
        
        # Sleeping is fine — but while you're sleeping, another signal may come, execute another activity,
        # have modified `self.x` — and be sleeping here, along with you
        await asyncio.sleep(1)  # or await anything else
        
        # Race condition: multiple signal handlers executing concurrently may lead to inconsistent results:
        # `self.x` set by one process, `self.y` from another
        self.y = data.y
```

The solution is to use

```python
    def __init__(self) -> None:
        self.lock = asyncio.Lock()
    
    @workflow.signal
    async def safe_async_handler(self):
        async with self.lock:
```

Now only one signal can update `self` at a time.


## Worker

The Worker Process is where Workflow Functions and Activity Functions are executed.
A Worker Process may run multiple Worker Entities, each listening on a queue and performing specific Workflows and Actions.

A Worker Entity must associate itself with a Task Queue. Tasks will come from it: Workflows and Activities

Workers must register the exact Workflow Types (names) and Activity Types (names) it may execute.
If multiple workers are polling on the same queue, they must all be registered with the same set of workflows and activities.

Task Queues are capability-based routing: just explicit instead of implicit.
You may want to use specific queues for:
- Hardware: e.g. GPU workers for ML
- Geography: data residency requirements
- Credentials: workers that have prod DB access
- Priority queues
- Deployment: blue/green, canary, etc, to minimize blast radius


## Dynamic Components

You can define a Workflow or an Activity that *matches a pattern* rather than having a distinct name.
Use it sparingly: as a fallback mechanism, not a primary design.

Same for Signals, Queries and Updates: you can define a dynamic one
that receives "name" and "args":

```python
@workflow.signal(dynamic=True)
async def dynamic_signal(self, name: str, args: Sequence[RawValue]) -> None:
    ...
```

# Versioning

Versioning allows workflows that are already running to use the older version of a Workflow —
whereas new workflows are launched using the newer version.

Two versioning methods:

1. Worker versioning: tag workers. When rolling out new code, old Workers will keep running old code paths.
2. Versioning with Patching: add branches to your code: `if (version)`

For Worker Versioning, see: [Worker Versioning](https://docs.temporal.io/production-deployment/worker-deployments/worker-versioning)
For most teams, Worker Versioning should be the default approach for evolving Workflow code in production.

Patching is a three-step process:

1. Patch new code with `if workflow.patched("my-patch"): new-activity`. It's a feature flag.
2. After all old workflows have left retention, add this to the code: `workflow.deprecate_patch("my-patch")`
3. After all old workflows have left retention, remove the feature flag and switch to the new code

Why retention period matters?
Because after all workflows have completed, they can still be reset: i.e. restarted. 
But after retention period (72h) they can't be restarted anymore.

# Workflow/Activity Metadata

When starting a workflow, you can provide a static summary in markdown (without HTML):

```python
handle = await client.start_workflow(
    # ...
    # Add context/metadata to help users in the UI
    static_summary="Order processing for customer #12345",  # single-line, 200 bytes max
    static_details="Processing premium order with expedited shipping",  # multi-line
)
```

within the workflow, you can get and set dynamic details:

```python
    # Get the current details
    current_details = workflow.get_current_details()
    print(f"Current details: {current_details}")
    
    # Set/update the current details
    workflow.set_current_details("Updated workflow details with new status")  # markdown, without HTML
```

Similarly, you can attach metadata to Activities:

```python
result = await workflow.execute_activity(
    # ...
    summary="Processing user data"
)
```

and to timers:

```python
await workflow.sleep(
    timedelta(minutes=5), 
    summary="Waiting for payment confirmation"
)
```

# Scheduled Workflow

Schedule workflows:

```python
async def main():
    client = await Client.connect("localhost:7233")

    await client.create_schedule(
        "workflow-schedule-id",
        Schedule(
            action=ScheduleActionStartWorkflow(
                YourSchedulesWorkflow.run,
                "my schedule arg",
                id="schedules-workflow-id",
                task_queue="schedules-task-queue",
            ),
            spec=ScheduleSpec(
                intervals=[ScheduleIntervalSpec(every=timedelta(minutes=2))]
            ),
            state=ScheduleState(note="Here's a note on my Schedule."),
        ),
    )
```


# Testing with Replay
See: [Testing with Replay History](https://docs.temporal.io/develop/safe-deployments)

Test new code against recorded histories.
In this mode, the workflow won't execute anything new: only test that the code is deterministic.

```python
start_time = (datetime.now() - timedelta(hours=10)).isoformat(timespec='seconds')
workflows = client.list_workflows(
    f"TaskQueue={task_queue} and StartTime > '{start_time}'",
limit = 100)
histories = workflows.map_histories()
replayer = Replayer(
    workflows=my_workflows,
)
await replayer.replay_workflows(histories)
```

# Further reading

* [Temporal Nexus](https://docs.temporal.io/develop/python/nexus/quickstart) to connect namespaces
* [Interceptors](https://docs.temporal.io/develop/python/interceptors) to intercept inbound and outbound Temporal calls:
  works similar to middleware and is used to add common behavior across many calls, such as logging, metrics collection, context propagation.
* [Environment Configuration](https://docs.temporal.io/develop/environment-configuration)
* [Worker Performance](https://docs.temporal.io/develop/worker-performance) 
* [Worker Tuning](https://docs.temporal.io/develop/worker-tuning-reference)
* [Plugins](https://docs.temporal.io/develop/plugins-guide)
* [Production Deployment](https://docs.temporal.io/production-deployment)
* [Best Practices](https://docs.temporal.io/best-practices/)
* [Encyclopedia](https://docs.temporal.io/encyclopedia/)

# Examples

## docker-compose.yml
```yaml
# Official docker-compose:
# https://github.com/temporalio/samples-server/tree/main/compose

services:
  postgresql:
    image: postgres:latest
    environment:
      POSTGRES_PASSWORD: temporal
      POSTGRES_USER: temporal
    volumes:
      # Note that mount point: new Postgres v18 uses a different data folder layout
      - postgres_data:/var/lib/postgresql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U temporal"]
      interval: 5s
      timeout: 5s
      retries: 60
      start_period: 30s

  # https://hub.docker.com/r/temporalio/server
  temporal:
    image: temporalio/server:latest
    depends_on:
      postgresql:
        condition: service_healthy
      temporal-admin-tools:
        condition: service_completed_successfully
    volumes:
      - temporal_data:/home/temporal
      - ./dynamicconfig:/etc/temporal/config/dynamicconfig
    environment:
      - DB=postgres12
      - DB_PORT=5432
      - DBNAME=temporal
      - VISIBILITY_DBNAME=temporal_visibility
      - POSTGRES_SEEDS=postgresql  # hostname
      - POSTGRES_USER=temporal
      - POSTGRES_PWD=temporal
      - BIND_ON_IP=0.0.0.0
      - DYNAMIC_CONFIG_FILE_PATH=config/dynamicconfig/development-sql.yaml
    ports:
      - 7233:7233   # gRPC frontend (your workers connect here)
      - 8233:8233
    healthcheck:
      test: ["CMD", "nc", "-z", "localhost", "7233"]
      interval: 5s
      timeout: 3s
      start_period: 30s
      retries: 60

  # https://hub.docker.com/r/temporalio/ui
  temporal-ui:
    image: temporalio/ui:latest
    depends_on:
      temporal:
        condition: service_healthy
    environment:
      - TEMPORAL_ADDRESS=temporal:7233
      - TEMPORAL_CORS_ORIGINS=http://localhost:3000
    ports:
      - 8080:8080   # Web UI


  temporal-admin-tools:
    image: temporalio/admin-tools:latest
    container_name: temporal-admin-tools
    restart: on-failure:6
    depends_on:
      postgresql:
        condition: service_healthy
    environment:
      - DB=postgres12
      - DB_PORT=5432
      - POSTGRES_USER=temporal
      - POSTGRES_PWD=temporal
      - POSTGRES_SEEDS=postgresql
      - SQL_PASSWORD=temporal
    volumes:
      - ./scripts:/scripts
    entrypoint: ["/bin/sh"]
    command: /scripts/setup-postgres.sh

  temporal-create-namespace:
    image: temporalio/admin-tools:latest
    container_name: temporal-create-namespace
    restart: on-failure:5
    depends_on:
      temporal:
        condition: service_healthy
    environment:
      - TEMPORAL_ADDRESS=temporal:7233
      - DEFAULT_NAMESPACE=default
    volumes:
      - ./scripts:/scripts
    entrypoint: ["/bin/sh"]
    command: /scripts/create-namespace.sh

volumes:
    postgres_data:
    temporal_data:
```

## selftest.py
```python
import asyncio
import temporalio
import temporalio.client

async def main():
    # Test connection
    client = await temporalio.client.Client.connect("localhost:7233")
    print(f'{client.namespace=}')


if __name__ == "__main__":
    asyncio.run(main())
```

## 01-quickstart/activities.py
```python
from datetime import timedelta
from temporalio import activity
from temporalio.exceptions import ApplicationError, ApplicationErrorCategory

# Activity: normal function that executes a single well-defined action.
# It may be long-running.
# If an Activity fails, Temporal automatically retries it based on your configuration.

# Activities can be sync or async.
# Use sync -- because its safer: only make it async if you are certain it doesn't block the event loop.
# Use plain Python tools to run sync code:
# - asyncio.to_thread()
# - loop.run_in_executor()
@activity.defn(
    # Custom name.
    # Default: function name
    name="greet"
)
async def greet(name: str) -> str:
    # Send heartbeat: indicate that I've not crashed!
    # An activity can only be cancelled when it heartbeats.
    # Activities that don't Heartbeat can't receive a Cancellation.
    activity.heartbeat("heartbeat details!")  # any type!
    # If an activity gets retried, it can access the details from the previous run
    if activity.info().attempt > 1:
        # Last heartbeat
        activity.info().heartbeat_details

    try:
        return f"Hello {name}"
    except Exception as e:
        attempt = activity.info().attempt
        # Temporal will catch your errors and internally convert them into an "ApplicationError"
        raise ApplicationError(
            'Give up: no reason to retry',
            # The only reason why you might want to use this ApplicationError:
            # set `non_retryable` to True if you want to prevent retries for this error.
            type='MyNonRetryableError',
            non_retryable=True,
            # Or if you want to customize the next retry:
            next_retry_delay=timedelta(seconds=3 * attempt),
            # Mark this error as "benign": an expected error is part of normal operations:
            # e.g. polling an external service that isn't ready yet, or transient erorrs that will be retried.
            # Such errors:
            # - logged with level=DEBUG: less noise
            # - do not emit activity failure metrics
            # - do not set the state to "error"
            category=ApplicationErrorCategory.BENIGN,
        ) from e

# Class-based activities.
# You'll initialize the class manually, yourself.
#
# Why? To inject dependencies like DB access and HTTP session object (connection multiplexing)
class TranslateActivities:
    db: DB

    @activity.defn
    def greet_in_spanish(self, name: str) -> str:
        greeting = self.call_service("get-spanish-greeting", name)
        return greeting

    # Utility method for making calls to the microservices
    def call_service(self, stem: str, name: str) -> str:
        base = f"http://localhost:9999/{stem}"
        url = f"{base}?name={urllib.parse.quote(name)}"
        response = requests.get(url)
        return response.text
```

## 01-quickstart/workflows.py
```python
from __future__ import annotations
from dataclasses import dataclass
from datetime import timedelta
from temporalio import workflow
from temporalio.exceptions import ApplicationError, ActivityError

# Pass modules through the "unsafe.imports_passed_through()" decorator:
# it marks all imports as "passed through": i.e. they won't be re-imported when the sandbox runs a workflow.
#
# Why: sandbox re-imports modules to prevent shared global state. "Passing" a module means it won't be re-imported,
# which is a must for modules that can't survive re-import (C extensions and modules with side effects).
# The modules will now have a shared state, which introduces non-determinism:
# Temporal makes you spell out "unsafe" to acknowledge you know what you're doing.
#
# "Pass Through" refers to passing modules from outside into the sandbox.
# By default, all standard library modules are passed through.
# Users should pass:
# - models
# - activities
# - Nexus services
# - other models whose calls are deterministic and free of side-effects
# Candidates:
# - modules that can't be imported twice
# - modules that are really slow to initialize
# Everything else is sandboxed:
# - When a workflow starts, its file is imported into a newly created sandbox.
# - A known set of modules are 'passed through' from outside the sandbox
#
# Examples of modules that are non-deterministic:
# - Random initialization
# - Mutable global state that accumulates (inc. cache)
# - Side effects that change the environment: e.g. setlocale()
#
# Alternative: use with_passthrough_modules() at Worker creation:
#   restrictions=SandboxRestrictions.default.with_passthrough_modules("pydantic")
#
# Skip sandboxing for a block of code:
#   with temporalio.workflow.unsafe.sandbox_unrestricted():
# Skip sandboxing for an entire Workflow:
#   @workflow.defn(sandboxed=False)
with workflow.unsafe.imports_passed_through():
    from activities import greet

@dataclass
class ApproveInput:
    name: str

# Workflows orchestrate activities.

# Define a workflow: class-based.
# In Temporal Python SDK, all workflows are classes.
# The class can also implement signals (e.g. "email_verified()") and queries ("is_verified()")
@workflow.defn(
    # You can customize the workflow name.
    # Default: the class name
    name="say-hello-workflow"
)
class SayHelloWorkflow:
    # Workflow run() method: can only have positional arguments
    # Best practice: use one dataclass argument, add properties as needed -- to make sure the signature is not broken.
    # Args and Returns must be serializable:
    @workflow.run
    async def run(self, name: str) -> str:
        # Replay-safe alternatives
        rnd = workflow.random()  # deterministic random
        workflow.logger.info('Greeting', extra=dict(arg_name=name, random=rnd))  # Suppresses duplicate logs

        # Execute activity
        return await workflow.execute_activity(
            greet,
            name,
            # Timeout: "schedule-to-close"
            schedule_to_close_timeout=timedelta(seconds=10),
            # Heartbeat timeout, per-activity
            heartbeat_timeout=timedelta(seconds=1),
        )

        # Core design principles: workflows never fail. Any error just fails the Workflow Task and be retried.
        # The default policy: retry failed activities until a timeout; activities do not return a failure until
        # this or another non-retryable condition is met.
        #
        # You can deliberately fail a workflow by raising `ApplicationError`.
        try:
            credit_card_confirmation = await workflow.execute_activity_method()
        except ActivityError as e:
            workflow.logger.error(f"Unable to process credit card {e.message}")
            # Give up: non-retryable
            raise ApplicationError(
                "Unable to process credit card", "CreditCardProcessingError"
            )
```

## 01-quickstart/worker.py
```python
import asyncio
import logging
import pydantic
from concurrent.futures import ThreadPoolExecutor

from temporalio.client import Client
from temporalio.worker import Worker
from temporalio import workflow
from temporalio.runtime import Runtime, TelemetryConfig, PrometheusConfig
from temporalio.contrib.pydantic import pydantic_data_converter

# Import workflows and activities in the context manager
# Context manager to mark all imports that occur within it as passed through (meaning not reloaded by the sandbox).
with workflow.unsafe.imports_passed_through():
    from workflows import SayHelloWorkflow
    from activities import greet

# The Worker Process is where Workflow Functions and Activity Functions are executed.
# All the logging also happens here.
async def main():
    # The SDK core uses WARN for its default logging level.
    # i.e. you won't see a thing unless you enable this one :)
    logging.basicConfig(level=logging.INFO)

    # Client
    # Optionally: configure from env:
    #   connect_config = ClientConfig.load_client_connect_config()
    new_runtime = Runtime(
        # With telemetry enabled
        telemetry=TelemetryConfig(metrics=PrometheusConfig(bind_address="0.0.0.0:9000"))
    )
    client = await Client.connect(
        "localhost:7233", runtime=new_runtime,
        # Use Pydantic Data Converter
        data_converter=pydantic_data_converter,
    )

    # Test connection
    print(f'{client.namespace=}')

    # Worker
    with ThreadPoolExecutor(max_workers=42) as activity_executor:
        worker = Worker(
            client,
            debug_mode=True,
            # A worker must associate itself with a Task Queue.
            # Tasks will come from it: Workflows and Activities
            task_queue="my-task-queue",
            # Workers must register the exact Workflow Types (names) and Activity Types (names) it may execute.
            workflows=[
                SayHelloWorkflow,
            ],
            activities=[
                greet,
            ],
            # Activities are run on the worker's main event loop.
            # No sandbox, no thread, no replay. Only retry.
            activity_executor=activity_executor,
            # Workflows are run in threads because they are CPU-bound.
            # NOTE: each workflow gets its own event loop -- in its own thread!
            # NOTE: because of GIL, spawn multiple processes to use multiple cores!
            workflow_task_executor=ThreadPoolExecutor(5),
            # To reduce the risk of event loops or executors getting blocked,
            # some users choose to deploy separate Workers for Workflow Tasks and Activity Tasks.
        )
        print("Worker started.", flush=True)
        await worker.run()

if __name__ == "__main__":
    asyncio.run(main())
```

## 01-quickstart/launch.py
```python
import asyncio
import uuid
from datetime import timedelta
from temporalio.client import Client
from temporalio.common import RetryPolicy, Priority

async def main():
    # Client
    client = await Client.connect("localhost:7233")

    # Execute workflow, get the workflow handle
    handle = await client.start_workflow(
        # Workflow by name
        "say-hello-workflow",
        # Positional args
        "Temporal",
        # Workflow id
        # Recommended: map it to a business process, e.g. order id or customer id
        id=f"say-hello-workflow-{uuid.uuid4()}",
        # Signal with start: send a signal to the workflow, start it if it's not already running.
        start_signal="submit_greeting",
        start_signal_args=["User Signal with Start"],
        # Specify the task queue
        # Use separate Task Queues for distinct workloads.
        # This isolation allows you to control rate limiting, prioritize certain workloads.
        task_queue="my-task-queue",
        # Workflows are not retried by default! If they fail — they stay failed.
        # Use a retry policy:
        retry_policy=RetryPolicy(maximum_interval=timedelta(seconds=2)),
        priority=Priority(
            # Priority: [1..5], where 1=highest. Default=3.
            # Lower-priority tasks are BLOCKED until higher-priority ones finish running!
            # Tasks with the same priority are run in FIFO order.
            priority_key=1,
            # Fairness key: each key creates a "virtual queue": workers take tasks using round-robin.
            # Use case: tenants, applications, workload types.
            fairness_key="greets",
            # Fairness weight: give more or less resources to a fairness key.
            # Default: 1.0.
            # Tasks from a key with weight=2.0 will be dispatched twice as often.
            fairness_weight=2.0,
        ),

        # Recommended: don't set workflow timeouts because they are designed to be long-running.
        # Without a timeout, workflows can survive temporal outages and bugs.
        # execution_timeout=timedelta(seconds=2),
        # run_timeout=timedelta(seconds=2),
        # task_timeout=timedelta(seconds=2),

        # Add context/metadata to help users in the UI
        static_summary="Order processing for customer #12345",  # markdown without HTML
        static_details="Processing premium order with expedited shipping"
    )

    # Get details.
    details = await handle.describe()
    print(f'Workflow {details.status=} {details.start_time}')

    # `await result` means we actually wait for it to complete
    result = await handle.result()
    print("Workflow result:", result)


if __name__ == "__main__":
    asyncio.run(main())
```

## 01-quickstart/workflow_examples.py
```python
from dataclasses import dataclass
from temporalio import workflow
from datetime import timedelta


@workflow.defn
class ExampleWorkflow:
    # NOTE: object is not persisted. Instead, it is rebuilt every time — using cached results from activities.
    # After replay, the fields just happen to equal what they were before: thanks to determinism.
    total_steps: int
    current_step: int
    name: str | None

    # By default, __init__() does not have access to workflow input parameters.
    # But if your signals need them, use this decorator
    # @workflow.init
    def __init__(self):
        self.total_steps = 2
        self.current_step = 0
        self.name = None  # unknown yet

    @workflow.run
    async def run(self, name: str) -> str:
        self.current_step += 1

        ...

        self.current_step += 1
        return ''


    # A Query can inspect but must not mutate the Workflow state.
    @workflow.query
    def progress(self) -> dict:
        return {"step": self.current_step, "total": self.total_steps}

    # It can also receive arguments!
    @workflow.query
    def already_processed(self, event_id: str) -> bool:
        return event_id in self._processed_events

    # A Signal handler mutates the Workflow state but cannot return a value.
    @workflow.signal
    def approve(self, input: ApproveInput) -> None:
        self.approved_for_release = True
        self.approver_name = input.name

    # An Update is synchronous: is mutates the state and returns a result
    @workflow.update
    def set_language(self, language: Language) -> Language:
        previous_language, self.language = self.language, language
        return previous_language

    # Validator: update may get rejected early
    # Use validators to reject an Update before it is written to History.
    @set_language.validator
    def validate_language(self, language: Language) -> None:
        if language not in self.greetings:
            # 👉 In an Update validator you raise any exception to reject the Update.
            raise ValueError(f"{language.name} is not supported")

@dataclass
class ApprovalResult:
    approved: bool
    reviewer: str
    comment: str

@workflow.defn
class DocumentApprovalWorkflow:
    # By default, __init__() does not have access to workflow input parameters.
    # But if your signals need them, use this decorator
    # @workflow.init
    def __init__(self):
        self._approvals: list[ApprovalResult] = []
        self._rejected = False
        self._cancelled = False

    @workflow.signal
    async def approve(self, result: ApprovalResult):
        self._approvals.append(result)

    @workflow.signal
    async def reject(self, result: ApprovalResult):
        self._rejected = True
        self._approvals.append(result)

    @workflow.signal
    async def cancel(self):
        self._cancelled = True

    @workflow.query
    def status(self) -> dict:
        return {
            "approvals": len(self._approvals),
            "approved_by": [a.reviewer for a in self._approvals if a.approved],
            "rejected": self._rejected,
        }

    @workflow.run
    async def run(self, doc_id: str, required_approvals: int) -> str:
        await workflow.execute_activity(
            notify_reviewers,
            doc_id,
            start_to_close_timeout=timedelta(seconds=10),
        )

        # wait until cancelled, rejected, or enough approvals
        await workflow.wait_condition(
            lambda: self._cancelled
                or self._rejected
                or len(self._approvals) >= required_approvals,
            timeout=timedelta(days=7),  # auto-expire after a week
        )

        if self._cancelled:
            return "cancelled"
        if self._rejected:
            reason = next(a.comment for a in self._approvals if not a.approved)
            await workflow.execute_activity(
                notify_rejection, doc_id, reason,
                start_to_close_timeout=timedelta(seconds=10),
            )
            return "rejected"

        await workflow.execute_activity(
            publish_document, doc_id,
            start_to_close_timeout=timedelta(seconds=30),
        )
        return "published"

@workflow.defn
class GreetingWorkflow:
    def __init__(self) -> None:
        self.approved_for_release = False
        self.approver_name: Optional[str] = None

    @workflow.signal
    def approve(self, input: ApproveInput) -> None:
        self.approved_for_release = True
        self.approver_name = input.name

    @workflow.run
    async def run(self) -> str:
        await workflow.wait_condition(lambda: self.approved_for_release)
        ...
        return self.greetings[self.language]
```

## 01-quickstart/test_activities.py
```python
import pytest
from temporalio.testing import ActivityEnvironment

from activities import greet

@pytest.mark.asyncio
async def test_greeting():
    # Simple activities are plain functions: test them as usual
    # result = await greet("Traveller")
    # assert result == "Hello Traveller!"

    # If they use Temporal APIs: use ActivityEnvironment that provides a fake context:
    env = ActivityEnvironment()
    result = await env.run(greet, "Traveller")
    assert result == "Hello Traveller"

    # Here's how to collect heartbeats:
    heartbeats = []
    env.on_heartbeat = lambda *args: heartbeats.append(args[0])
    result = await env.run(greet, "Traveller")
    assert heartbeats == [
        "heartbeat details!",
    ]
```

## 01-quickstart/test_workflows.py
```python
import pytest
from temporalio import activity
from temporalio.client import Client
from temporalio.testing import WorkflowEnvironment
from temporalio.worker import Worker

from workflows import SayHelloWorkflow

@pytest.mark.asyncio
async def test_say_hello_mocked():
    # Time skipping
    async with await WorkflowEnvironment.start_time_skipping() as env:
        # Start a real worker -- but with a mocked Temporal client
        async with Worker(
            env.client,
            task_queue="test",
            workflows=[SayHelloWorkflow],
            activities=[mock_greet],  # real activity — or mock it:
        ):
            # Execute the workflow through `env` -- for time skipping.
            result = await env.client.execute_workflow(
                SayHelloWorkflow.run,
                "Traveller",
                id="test-wf",
                task_queue="test",
            )
            assert result == "Hey Traveller from mocked activity!"

# Mocked activity
@activity.defn(name="greet")
async def mock_greet(name: str) -> str:
    return f"Hey {name} from mocked activity!"
```
