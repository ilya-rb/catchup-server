use catchup_server::app::App;
use catchup_server::telemetry::LogLevel;
use catchup_server::{configuration, telemetry};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    telemetry::init_tracing("catchup-server".into(), LogLevel::Info, std::io::stdout);

    let settings = configuration::read_configuration().expect("Failed to read app settings");
    let app = App::build(settings.clone()).await?;
    app.run_until_stopped(settings).await?;

    Ok(())
}
