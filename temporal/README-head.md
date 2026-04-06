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
