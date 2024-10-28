use catchup_server::app::App;
use catchup_server::telemetry::LogLevel;
use catchup_server::{configuration, telemetry};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    telemetry::init_tracing(
        String::from("catchup-server"),
        LogLevel::Info,
        std::io::stdout,
    );

    let settings = configuration::read_configuration().expect("Failed to read app settings");
    let app = App::build(settings).await?;

    app.run_until_stopped().await?;

    Ok(())
}
