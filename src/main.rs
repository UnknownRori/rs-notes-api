use color_eyre::Report;
use rs_notes_api::app::App;
use rs_notes_api::setup;

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    App::new("127.0.0.1:3000").await?.serve().await?;

    Ok(())
}
