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
