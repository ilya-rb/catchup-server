use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub enum LogLevel {
    Debug,
    Info,
}

impl LogLevel {
    fn key(&self) -> String {
        match &self {
            LogLevel::Debug => String::from("debug"),
            LogLevel::Info => String::from("info"),
        }
    }
}

pub fn init_tracing<Sink>(app_name: String, log_level: LogLevel, sink: Sink)
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    LogTracer::init().expect("Failed to init logger");

    let subscriber = get_subscriber(app_name, log_level.key(), sink);
    set_global_default(subscriber).expect("Failed to set subscriber for tracing");
}

fn get_subscriber<Sink>(
    app_name: String,
    filter_level: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter_level));
    let formatting_layer = BunyanFormattingLayer::new(app_name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}
