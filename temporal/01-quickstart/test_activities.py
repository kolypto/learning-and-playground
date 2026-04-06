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
