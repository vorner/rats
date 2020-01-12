use std::process;

use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::core::TransformBundle;
use amethyst::Error;
use amethyst::prelude::*;
use amethyst::renderer::{RenderingBundle, RenderFlat2D, RenderToWindow};
use amethyst::renderer::types::DefaultBackend;
use log::{error, info};

use load::Load;

mod load;

fn run() -> Result<(), Error> {
    amethyst::start_logger(Default::default());

    info!("{} version {} starting up", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let app_root = amethyst::utils::application_root_dir()?;
    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let game_data = GameDataBuilder::new()
        // Auto-support transformations ‒ positioning, scaling, etc
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            // Output to window
            .with_plugin(RenderToWindow::from_config_path(display_config)
                // With solid black background
                .with_clear([0.0, 0.0, 0.0, 1.0])
            )
            .with_plugin(RenderFlat2D::default())
        )?;

    Application::build(resources, Load::new())?
        .with_frame_limit(FrameRateLimitStrategy::Sleep, 30)
        .build(game_data)?
        .run();

    info!("Graceful shutdown");

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        error!("{}", e);
        process::exit(1);
    }
}
