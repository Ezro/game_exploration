use amethyst::{
    assets::{AssetStorage, Loader},
    core::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{
        Camera, DebugLines, DebugLinesParams, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata, Transparent,
    },
};

use crate::components::*;

pub struct Example;
impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let circle_sprite_sheet_handle =
            load_sprite_sheet(world, "Circle_Spritesheet.png", "Circle_Spritesheet.ron");
        let background_sprite_sheet_handle =
            load_sprite_sheet(world, "Background.png", "Background.ron");

        let _background = init_background_sprite(world, &background_sprite_sheet_handle);
        let _reference = init_reference_sprite(world, &circle_sprite_sheet_handle);
        let _player = init_player(world, &circle_sprite_sheet_handle);
        init_camera(world);
        world.add_resource(DebugLines::new());
        world.add_resource(DebugLinesParams { line_width: 10.0 });
    }
}

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            png_path,
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

// Initialize a background
fn init_background_sprite(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_z(-10.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world.create_entity().with(transform).with(sprite).build()
}

// Initialize a sprite as a reference point at a fixed location
fn init_reference_sprite(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_x(100.0);
    transform.set_translation_y(0.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Transparent)
        .with(AABB { x: 0.0, y: 0.0 })
        .with(Size { x: 64.0, y: 64.0 })
        .build()
}

fn init_player(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_x(0.0);
    transform.set_translation_y(0.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
    };
    world
        .create_entity()
        .with(transform)
        .with(Player)
        .with(sprite)
        .with(Transparent)
        .with(AABB { x: 0.0, y: 0.0 })
        .with(Speed { speed: 5.0 })
        .with(Position { x: 0.0, y: 0.0 })
        .with(Heading { x: 0.0, y: -1.0 })
        .with(Size { x: 64.0, y: 64.0 })
        .build()
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_z(10.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -960.0, 960.0, -540.0, 540.0,
        )))
        .with(transform)
        .with(PlayerCamera)
        .build();
}
