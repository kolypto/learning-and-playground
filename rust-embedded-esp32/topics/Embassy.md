# Embassy

## embassy-executor

`embassy-executor` is an async/await executor specifically designed for embedded systems.

It has a `nightly` Cargo feature:
when it's not enabled, it allocates tasks out of an arena (a very simple bump allocator).
If the task arena gets full, the program will panic at runtime.
To guarantee this doesn’t happen, you must set the size to the sum of sizes of all tasks.

The default arena size is 4096. However `esp-generate` uses `20480` as the default.

To configure the arena size:

* Use cargo feature: `task-arena-size-8192`
* Environment variables during build: `EMBASSY_EXECUTOR_TASK_ARENA_SIZE=8192 cargo build`.
  Environment variables take precedence over Cargo features.

When using `nightly` Rust, enable the `nightly` Cargo feature.
This will make `embassy-executor` use the `type_alias_impl_trait` feature to allocate all tasks in statics.
Each task gets its own `static`.
If tasks don’t fit in RAM, this is detected at compile time by the linker. Runtime panics due to running out of memory are not possible.

## `'static`

Embassy tasks need their arguments as `'static` values.
However, some peripherals that you init in `main()` aren't `'static`.

Here's how you can make them such:

### `static_cell::make_static!()`

Use `make_static!()` macro:

```rust
use static_cell::make_static;
let radio = make_static!(esp_radio::init().expect("Radio init"));
```

you can even choose which memory region to allocate it in:

```rust
let buf = make_static!([0u8; 4096], #[link_section = ".ext_ram.bss.buf"]);
```

However, in my case, the macro fails when ran inside `main()`.

### `static_cell::StaticCell`

You can initialize static values manually:

```rust
use static_cell::StaticCell;

static RADIO: StaticCell<esp_radio::Controller> = StaticCell::new();
let radio = RADIO.init(esp_radio::init().expect("Init radio"));
```

One issue: that value will be `&mut T`, and it will give exclusive access to just one task.
That's actually the whole point: otherwise multiple conflicting references would be possible!

To get rid:

```rust
&*radio
```

or just pass it to a func that accepts `&'static` argument: you'll lose the `&mut` part:

```rust
pub async fn run_tasks(bt_transport: &'static BleConnector<'static>) -> Result<()> {
    ...
}
```

Finally, here's a short version:

```rust
let radio = {
    static RADIO: StaticCell<esp_radio::Controller> = StaticCell::new();
    RADIO.init(esp_radio::init().expect("Init radio"))
};
```

### `mk_static!()`
If you don't want nightly Rust, use this macro:

```rust
#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}
```

### `once_cell`

```rust
use once_cell::sync::OnceCell;
static INSTANCE: OnceCell<Logger> = OnceCell::new();

fn setup() {
    INSTANCE.set(logger).unwrap();
    INSTANCE.get().expect("logger is not initialized"); // -> &T
}
```

### Mutex

Use this when multiple tasks need to co-operate:

```rust
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

static SHARED: Mutex<CriticalSectionRawMutex, Option<MyStruct>> = Mutex::new(None);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    *SHARED.lock().await = Some(MyStruct::new());
}
```


### `core::mem::MaybeUninit`

This is in `core` and bundled with Rust.
Note that `static_cell` already uses it under the hood :) So no point.

```rust
use core::mem::MaybeUninit;
static mut RADIO: MaybeUninit<esp_radio::Controller<'static>> = MaybeUninit::uninit();
let radio = unsafe {
    RADIO.write(esp_radio::init().expect("Init radio"));
    RADIO.assume_init_mut()
};
```


### Unsafe way

```rust
static mut BUFFER: [u8; 1024] = [0; 1024];

#[embassy_executor::task]
async fn my_task() {
    unsafe {
        BUFFER[0] = 42;
    }
}
```
