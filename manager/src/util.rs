use tracing::subscriber::SetGlobalDefaultError;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

/// Sets up tracing, allows log level to be set from the environment variables
pub fn init_tracing() -> Result<(), SetGlobalDefaultError> {
    let subscriber_builder: tracing_subscriber::fmt::SubscriberBuilder<
        tracing_subscriber::fmt::format::DefaultFields,
        tracing_subscriber::fmt::format::Format,
        EnvFilter,
    > = FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env());
    tracing::subscriber::set_global_default(
            subscriber_builder.with_ansi(true).pretty().finish())
}