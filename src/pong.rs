use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::utils::application_root_dir;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};


pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;
pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub struct Pong;

impl SimpleState for Pong {

	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

		let world = data.world;
		
		// Load the spritesheet necessary to render the graphics.
	    let sprite_sheet_handle = load_sprite_sheet(world);

	    world.register::<Paddle>();

		initialize_paddles(world, sprite_sheet_handle);
		initialize_camera(world);

	}

}


pub enum Side {
	Left,
	Right
}

pub struct Paddle {
	pub side: Side,
	pub width: f32,
	pub height: f32
}

impl Paddle {
	fn new(side: Side) -> Paddle {
		Paddle {
			side,
			width: 1.0,
			height: 1.0
		}
	}
}

impl Component for Paddle {
	type Storage = DenseVecStorage<Self>;
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("{}/texture/pong_spritesheet.png", application_root_dir()),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

	let sprite_sheet = loader.load(
	    format!("{}/texture/pong_spritesheet.ron", application_root_dir()), // Here we load the associated ron file
	    SpriteSheetFormat,
	    texture_handle, // We pass it the handle of the texture we want it to use
	    (),
	    &sprite_sheet_store,
	);

	info!("{:?}", sprite_sheet);

	return sprite_sheet;

}

fn initialize_camera(world: &mut World) {
	let mut transform = Transform::default();
	transform.set_z(1.0);
	world
		.create_entity()
		.with(Camera::from(Projection::orthographic(
			0.0,
			ARENA_WIDTH,
			0.0,
			ARENA_HEIGHT
		)))
		.with(transform)
		.build();
}

fn initialize_paddles(world: &mut World, sprite_sheet: SpriteSheetHandle) {
	let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles.
    // let left_x = PADDLE_WIDTH * 0.5;
    // let right_x = ARENA_WIDTH - PADDLE_WIDTH * 0.5;
    let left_x = ARENA_WIDTH / 2.0 - 5.0;
	let right_x = ARENA_WIDTH / 2.0 + 5.0;
    let y = ARENA_HEIGHT / 2.0;
    
    left_transform.set_xyz(left_x, y, 0.0);
    right_transform.set_xyz(right_x, y, 0.0);

    info!("{:?}", sprite_sheet);

    // Assign the sprites for the paddles
	let sprite_render = SpriteRender {
	    sprite_sheet: sprite_sheet.clone(),
	    sprite_number: 0, // paddle is the first sprite in the sprite_sheet
	};

	info!("{:?}", sprite_render);

	// Create a left plank entity.
	world
	    .create_entity()
	    .with(sprite_render.clone())
	    // ... other components
	    .build();

	// Create right plank entity.
	world
	    .create_entity()
	    .with(sprite_render.clone())
	    // ... other components
	    .build();

	info!("{:?}", sprite_render);
}

