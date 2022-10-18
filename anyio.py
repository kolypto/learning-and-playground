# pip install anyio
# pip install anyio[trio]
# export PYTHONDEVMODE=1

# Updated: v3.6.1

import anyio
from anyio import run
from anyio import create_task_group, TASK_STATUS_IGNORED
from anyio import CancelScope, move_on_after, fail_after, get_cancelled_exc_class
from anyio.abc import TaskStatus
from anyio import sleep


# ### Basics
# Task group: async context manager.
# If the parent, or any child, raises an exception, all tasks are cancelled.
# Otherwise it waits for all children to finish.


# The ability to cancel tasks is the foremost advantage of the asynchronous programming model.
# Threads, on the other hand, cannot be forcibly killed and shutting them down will require perfect cooperation from the code running in them.


async def main():
    # Task group
    async with create_task_group() as tg:

        # Don't proceed until this task reports that it's ready
        await tg.start(server)

        # Create a cancel scope with a deadline
        # move_on_after(): on timeout, don't wait for tasks, move on without error
        # fail_after(): raise a TimeoutError on timeout
        with move_on_after(2) as scope:
            # Spawn background sleepers
            print('Starting sleepers')
            for num in range(5):
                tg.start_soon(sleeper, num)  # background task

            # Wait until they all finish
            # Any exceptions are raised by the task group.
            # Multiple exceptions are reported as: ExceptionGroup()
            print('Waiting for sleepers...')

        print('No more waiting')

        # Shield tasks from cancellation
        # use case: is performing shutdown procedures on asynchronous resources.
        # The shielded block will be exempt from cancellation except when the shielded block itself is being cancelled.
        # Best combined with move_on_after(shield=True) or fail_after(shield=True)
        with CancelScope(shield=True, deadline=1) as scope:
            tg.start_soon(sleeper, 'final')
            tg.cancel_scope.cancel()  # won't do a thing

    print('Done!')


# A task that needs some time to prepare itself: e.g. start listening on a port
async def server(*, task_status: TaskStatus = TASK_STATUS_IGNORED):
    await sleep(1)
    try:
        print('Server ready')
    except get_cancelled_exc_class():
        # Clean-up code for cancellation
        # If you need `await`, use `with CancelScope(shield=True)`.
        # Othewise your operation will be cancelled immediately since it’s in an already cancelled scope:
        with CancelScope(shield=True):
            pass

    task_status.started()


async def sleeper(name):
    await sleep(0.5)
    print('Sleeper', name, 'start')
    await sleep(0.5)
    print('Sleeper', name, 'done')

run(main, backend_options=dict(use_uvloop=1))











# ### Synchronization primitives

from anyio import Event
# notify tasks that something they’ve been waiting to happen has happened.
# NOTE: it cannot be reused! must be replaced!

from anyio import Semaphore
# Limit access to a shared resource: only N at a time

from anyio import Lock
# Critical section

from anyio import Condition
# Lock + Event: acquires a lock, then waits for an event to release the lock (event-controlled lock)

from anyio import CapacityLimiter
# Semaphore where a borrower (a task) can only hold a single token at a time













# ### Memory Object Streams (channels)

from anyio import create_memory_object_stream
from anyio.streams.memory import MemoryObjectSendStream, MemoryObjectReceiveStream

async def main():
    # You get a pair of streams.
    # By default, buffer size=0: send() blocks until someone read()s
    send_stream, receive_stream = create_memory_object_stream()

    # Streams can be cloned and passed around.
    # Purpose: let tasks close() it when it's done.
    # The stream itself only closes when all clones are close()d
    # When all clones are close()d, the stream is closed
    send_stream_2 = send_stream.clone()

    # Reader task
    async def reader(recv_stream: MemoryObjectReceiveStream):
        print(await recv_stream.receive())
        recv_stream.close()

    # Writer task
    async def writer(send_stream: MemoryObjectSendStream):
        await send_stream.send('HEY')
        send_stream.close()

    async with create_task_group() as tg:
        tg.start_soon(reader, receive_stream)
        tg.start_soon(writer, send_stream)

run(main)





# ### Networking

# NOTE: Unlike BSD sockets, EndOfStream exception is raised, not empty result!!

# Connect TCP
from anyio import connect_tcp

async def main():
    async with await connect_tcp('hostname', 1234) as client:
        await client.send(b'Client\n')
        response = await client.receive()
        print(response)

# Receive TCP
from anyio import create_tcp_listener

async def handle(client):
    async with client:
        name = await client.receive(1024)
        await client.send(b'Hello, %s\n' % name)


async def main():
    listener = await create_tcp_listener(local_port=1234)
    await listener.serve(handle)




# ### Threads

# Run function in a thread
import time
from anyio import to_thread, from_thread
from anyio import start_blocking_portal

from concurrent.futures import as_completed


async def main():
    # Run a function in a thread
    # Tasks are shielded when running a thread
    # Use `cancellable=True` to un-shield; the thread will keep running anyway
    await to_thread.run_sync(sync_sleep, 1)


def sync_sleep(seconds: float):
    time.sleep(seconds)
    print('Good morning')

    # Run a coroutine from a thread
    # Use `run()`.
    # Use `run_sync()` to run code in the event loop (e.g. non-thread-safe code)
    from_thread.run(anyio.sleep, 1)

    # Blocking portal: run async code from a foreign thread (e.g. not spawned by the loop)
    with start_blocking_portal() as portal:
        #portal.call(...)
        pass

    # Use async context manager in sync code
    async_cm = AsyncContextManager()
    with start_blocking_portal() as portal, portal.wrap_async_context_manager(async_cm):
        print('inside the context manager block')

    # Pattern: spawn a task from sync code
    with start_blocking_portal() as portal:
        futures = [portal.start_task_soon(long_running_task, i) for i in range(1, 5)]
        for future in as_completed(futures):
            print(future.result())

class AsyncContextManager:
    # NOTE: The __aenter__() and __aexit__() methods will be called from different tasks
    # so a task group as the async context manager will not work here.
    async def __aenter__(self):
        print('entering')

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        print('exiting with', exc_type)


def long_running_task(thread_id):
    time.sleep(1)


run(main)



# ### Subprocesses

# One-off command

from anyio import run_process, run

async def main():
    result = await run_process(['ps'])
    print(result.stdout.decode())

run(main)

# Use worker processes for CPU-bound tasks (avoids GIL)
import time
from anyio import run, to_process

def cpu_intensive_function(arg1, arg2):
    time.sleep(1)
    return arg1 + arg2

async def main():
    result = await to_process.run_sync(cpu_intensive_function, 'Hello, ', 'world!')
    print(result)

# This check is important when the application uses run_sync_in_process()
# The worker process imports the parent’s __main__ module, so guarding for any import time side effects using if __name__ == '__main__': is required to avoid inifinite recursion
# Worker processes terminate after 5 minutes of inactivity, or when the event loop is finished
if __name__ == '__main__':
    run(main)




# ### Async file I/O




from anyio import open_file, run
from anyio import wrap_file

async def main():
    # AnyIO provides async wrappers for sync file operations
    async with await open_file('/usr/share/dict/words') as f:
        contents = await f.read(15)
        print(contents)

    # Async iteration is supported
    async with await open_file('/usr/share/dict/words') as f:
        async for line in f:
            print(line, end='')
            break

    # Here's how to wrap an existing open file
    with open('/usr/share/dict/words') as f:
        async for line in wrap_file(f):
            print(line, end='')
            break

run(main)


# Async Path object

from anyio import Path, run

async def main():
    # Async file write
    path = Path('/tmp/iwrote')
    await path.write_bytes(b'hello, world')

    # Async list dir
    dir_path = Path('/tmp')
    async for path in dir_path.iterdir():
        if await path.is_file():
            print(await path.read_text())
            print('---------------------')

run(main)




# ### Testing

import pytest

# Use the mark
@pytest.mark.anyio
async def test_something():
    pass

# This is the same as using the @pytest.mark.anyio on all test functions in the module
pytestmark = pytest.mark.anyio
