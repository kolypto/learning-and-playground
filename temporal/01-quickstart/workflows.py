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
