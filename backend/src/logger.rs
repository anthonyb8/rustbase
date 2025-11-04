use crate::config::CONFIG;
use crate::Result;
use once_cell::sync::Lazy;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;

// Global logger guard to prevent dropping the worker
static LOGGER_GUARD: Lazy<Option<WorkerGuard>> = Lazy::new(|| None);

pub fn init_global_logger() -> Result<()> {
    let config = &*CONFIG;
    print!("{:?}", &config.log_file);

    let (non_blocking, guard) =
        tracing_appender::non_blocking(std::fs::File::create(&config.log_file)?);

    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(non_blocking)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true);

    let filter_layer = EnvFilter::new(&config.log_level);

    Registry::default()
        .with(file_layer)
        .with(filter_layer)
        .init();

    // Store the guard to prevent dropping
    std::mem::forget(guard);

    tracing::info!("Global logging system initialized");
    Ok(())
}

// pub fn system_logger(log_file: String, log_level: String) -> Result<()> {
//     // Create a file appender
//     let file = File::create(log_file)?;
//
//     // Create a file layer with JSON formatting
//     let file_layer = Layer::new()
//         .json()
//         .with_writer(file)
//         .with_file(true)
//         .with_line_number(true)
//         .with_thread_ids(true)
//         .with_target(true)
//         .with_span_events(FmtSpan::CLOSE); // Adding span close events for better traceability
//
//     // Create an EnvFilter layer to control log levels
//     let filter_layer = EnvFilter::new(log_level);
//
//     // Create a subscriber with the file layer and the filter layer
//     let subscriber = Registry::default().with(file_layer).with(filter_layer);
//
//     // Set the subscriber as the global default
//     tracing::subscriber::set_global_default(subscriber)?;
//
//     tracing::info!("Logging system initialized successfully.");
//
//     Ok(())
// }
