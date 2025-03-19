use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tower::limit::concurrency::ConcurrencyLimitLayer;
use socket2::{Socket, Domain, Type, Protocol};
use clap::Parser;
use tokio::runtime::Builder;
use papaya::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 250)]
    concurrency_limit: usize,
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

#[derive(Clone)]
struct AppState {
    data: Arc<HashMap<String, String>>,
}

fn create_optimized_listener(addr: SocketAddr) -> std::io::Result<std::net::TcpListener> {
    let domain = if addr.is_ipv6() { Domain::IPV6 } else { Domain::IPV4 };
    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_reuse_address(true)?;
    #[cfg(unix)]
    socket.set_reuse_port(true)?;
    socket.set_nodelay(true)?;
    socket.set_nonblocking(true)?;
    socket.set_recv_buffer_size(4 * 1024 * 1024)?;
    socket.set_send_buffer_size(4 * 1024 * 1024)?;
    let backlog = 4096;
    socket.bind(&addr.into())?;
    socket.listen(backlog)?;
    Ok(socket.into())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("Failed to install CTRL+C signal handler");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Builder::new_multi_thread()
        .worker_threads(8)
        .thread_name("http-worker")
        .thread_stack_size(4 * 1024 * 1024)
        .global_queue_interval(10)
        .thread_keep_alive(std::time::Duration::from_millis(100))
        .enable_all()
        .build()?;

    runtime.block_on(async {
        let args = Args::parse();
        let state = AppState {
            data: Arc::new(HashMap::with_capacity(1_000_000))
        };
        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route("/store/{key}", post(|Path(key), State(state): State<AppState>, body: String| async move {
                let map = state.data.pin();
                map.insert(key, body);
                StatusCode::OK
            }))
            .route("/get/{key}", get(|Path(key): Path<String>, State(state): State<AppState>| async move {
                let map = state.data.pin();
                match map.get(&key) {
                    Some(v) => (StatusCode::OK, v.clone()),
                    None => (StatusCode::NOT_FOUND, String::from("No value found for key")),
                }
            }))
            .with_state(state)
            .layer(ConcurrencyLimitLayer::new(args.concurrency_limit.min(500)));
        let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
        let listener = TcpListener::from_std(create_optimized_listener(addr)?)?;
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await?;
        Ok(())
    })
}