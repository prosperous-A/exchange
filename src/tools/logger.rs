use chrono::Local;
use tracing::instrument::WithSubscriber;
use tracing::{info, Level};
// use tracing_subscriber::fmt::format::FmtSpan;
use crate::exchange::types::ExchangeConfig;
use std::time::Duration;
use tracing_appender::non_blocking::NonBlocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::{fmt, prelude::*};

pub fn init_logger() -> tracing_appender::non_blocking::WorkerGuard {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(Level::INFO) // 设置最大日志级别
        .with_target(true)
        .with_line_number(true)
        .with_timer(ChronoLocal::rfc_3339())
        .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.6f".to_string()))
        .finish()
        .init();
    info!("tracing logger initialized.");
    _guard
}
