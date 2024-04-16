use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlotError {
    #[error("Unexpected response.")]
    UnexpectedError,
    #[error("The plot settings are wrong.")]
    WrongPlotSettings,
}
