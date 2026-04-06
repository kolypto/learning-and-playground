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
