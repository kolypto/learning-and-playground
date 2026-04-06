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
