extern crate amethyst;
#[macro_use]
extern crate log;
extern crate gdb;

mod pong;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline,
                         RenderBundle, Stage};
use amethyst::utils::application_root_dir;
use amethyst::core::transform::TransformBundle;

use crate::pong::Pong;

fn main() -> amethyst::Result<()> {

		amethyst::start_logger(Default::default());

		let path = format!("{}/resources/display_config.ron", application_root_dir());

		let config = DisplayConfig::load(&path);

		let pipeline = Pipeline::build()
		    .with_stage(
		        Stage::with_backbuffer()
		            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
		            .with_pass(
		                DrawFlat2D::new()
		                    .with_transparency(ColorMask::all(), ALPHA, None)
		            )
		    );

		let game_data = GameDataBuilder::default()
	        .with_bundle(
	          RenderBundle::new(pipeline, Some(config))
	            .with_sprite_sheet_processor()
	        )?
	        .with_bundle(TransformBundle::new())?;

		let mut game = Application::new("./", Pong, game_data)?;

		game.run();

		return Ok(());
	}
