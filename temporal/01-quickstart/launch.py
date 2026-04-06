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
