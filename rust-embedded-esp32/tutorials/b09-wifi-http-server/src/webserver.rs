use core::sync::atomic::Ordering;

use embassy_net::Stack;
use embassy_time::Duration;

// Picoserve: async http server for bare-metal environments.
// $ cargo add picoserve --features embassy
use picoserve::{
    AppBuilder, AppRouter, response::{File, IntoResponse}, routing::{self, Router}
};

// Serde: Serialize/Deserialize data structures
// $ cargo add serde --no-default-features --features derive
use serde;

// How many embassy tasks to spawm as http server workers?
pub const WEB_TASK_POOL_SIZE: usize = 2;


pub struct Application;

// Implement the `AppBuilder` trait for application: this creates a router.
// If you need a router with state, impl `AppWithStateBuilder`
impl AppBuilder for Application {
    type PathRouter = impl routing::PathRouter;

    fn build_app(self) -> Router<Self::PathRouter> {
        // Serve a static file.
        // Its contents are embedded.
        Router::new()
            .route(
                "/",
                routing::get_service(File::html(include_str!("index.html"))),
            )
            .route(
                "/set-led",
                routing::post(api_set_led),
            )
    }
}


// Request JSON
#[derive(serde::Deserialize)]
struct LedRequest {
    is_on: bool,
}

// Response JSON
#[derive(serde::Serialize)]
struct LedResponse {
    success: bool,
}

// API: set led
async fn api_set_led(input: picoserve::extract::Json<LedRequest, 0>) -> impl IntoResponse {
    // JSON value: put into LED_SATE
    crate::led::LED_STATE.store(input.0.is_on, Ordering::Relaxed);

    // Respond
    picoserve::response::Json(LedResponse { success: true })
}



// Web app: holds config and an instance of picoserve router
pub struct WebApp {
    pub router: &'static Router<<Application as AppBuilder>::PathRouter>,
    pub config: &'static picoserve::Config<Duration>,
}

impl Default for WebApp {
    fn default() -> Self {
        let router = picoserve::make_static!(AppRouter<Application>, Application.build_app());
        let config = picoserve::make_static!(
            picoserve::Config<Duration>,
            picoserve::Config::new(picoserve::Timeouts {
                start_read_request: Some(Duration::from_secs(5)),
                read_request: Some(Duration::from_secs(1)),
                write: Some(Duration::from_secs(1)),
                persistent_start_read_request: Some(Duration::from_secs(1)),
            })
            .keep_connection_alive()
        );

        Self { router, config }
    }
}


// A pool of http server workers
#[embassy_executor::task(pool_size = WEB_TASK_POOL_SIZE)]
pub async fn web_task(
    id: usize,
    stack: Stack<'static>,
    router: &'static AppRouter<Application>,
    config: &'static picoserve::Config<Duration>,
) -> ! {
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    picoserve::listen_and_serve(
        id,
        router,
        config,
        stack, 80,
        &mut tcp_rx_buffer,
        &mut tcp_tx_buffer,
        &mut http_buffer,
    )
    .await
}
