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
