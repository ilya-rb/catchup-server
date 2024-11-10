use catchup_server::app::App;
use catchup_server::telemetry::LogLevel;
use catchup_server::{configuration, telemetry};
use std::fmt::{Debug, Display};
use tokio::task::JoinError;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    telemetry::init_tracing(
        String::from("catchup-server"),
        LogLevel::Info,
        std::io::stdout,
    );

    let settings = configuration::read_configuration().expect("Failed to read app settings");
    let app = App::build(settings).await?;

    let app_worker = tokio::spawn(app.run_until_stopped());

    tokio::select! {
        outcome = app_worker => report_exit("APP", outcome)
    }

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => tracing::info!("{} exited", task_name),
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name,
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed to complete",
                task_name,
            )
        }
    }
}
