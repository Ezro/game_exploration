use amethyst;

use amethyst::{
    assets::{AssetStorage, Loader},
    core::{Transform, TransformBundle},
    ecs::{Component, Entity, Join, NullStorage, Read, ReadStorage, System, VecStorage, Write, WriteStorage},
    input::{InputBundle, InputHandler},
    prelude::*,
    renderer::{
        Camera, ColorMask, DebugLinesParams, DepthMode, DisplayConfig, DrawDebugLines, DrawFlat2D, Pipeline, PngFormat, PosColorNorm, Projection,
        RenderBundle, Rgba, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Stage,
        Texture, TextureMetadata, Transparent, ALPHA, DebugLines
    },
    utils::application_root_dir,
};

#[derive(Default)]
struct Player;
impl Component for Player {
    type Storage = NullStorage<Self>;
}

struct MovementSystem;
impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (players, mut transforms, input): Self::SystemData) {
        let x_move = input.axis_value("entity_x").unwrap();
        let y_move = input.axis_value("entity_y").unwrap();

        for (_, transform) in (&players, &mut transforms).join() {
            transform.translate_x(x_move as f32 * 5.0);
            transform.translate_y(y_move as f32 * 5.0);
        }
    }
}

#[derive(Default)]
struct PlayerCamera;
impl Component for PlayerCamera {
    type Storage = NullStorage<Self>;
}

struct CameraFollowSystem;
impl<'s> System<'s> for CameraFollowSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, PlayerCamera>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, (players, cameras, mut transforms): Self::SystemData) {
        let player_transform = {
            let mut player_transforms = (&transforms, &players).join();
            let player_transform = player_transforms
                .next()
                .map(|(transform, _)| transform.clone());
            if player_transforms.next().is_some() {
                // you probably want to do something to handle this case properly, but
                println!("WARN: More than one player entity!")
            }
            player_transform
        };

        if let Some(player_transform) = player_transform {
            for (camera_transform, _) in (&mut transforms, &cameras).join() {
                camera_transform.set_x(player_transform.translation().x);
                camera_transform.set_y(player_transform.translation().y);
            }
        }
    }
}

#[derive(Default)]
struct AABB {
    halfsize_x: u32,
    halfsize_y: u32
}

impl Component for AABB {
    type Storage = VecStorage<Self>;
}

struct DrawAABBSystem;
impl<'s> System<'s> for DrawAABBSystem {
    type SystemData = (
        Write<'s, DebugLines>,
        ReadStorage<'s, AABB>,
    );

    fn run(&mut self, (mut debug_lines_resource, aabbs): Self::SystemData) {
        for aabb in (&aabbs).join() {
            debug_lines_resource.draw_line(
                [-(aabb.halfsize_x as f32), aabb.halfsize_y as f32, 0.0].into(),
                [aabb.halfsize_x as f32, aabb.halfsize_y as f32, 0.0].into(),
                Rgba::black(),
            );
            debug_lines_resource.draw_line(
                [aabb.halfsize_x as f32, aabb.halfsize_y as f32, 0.0].into(),
                [aabb.halfsize_x as f32, -(aabb.halfsize_y as f32), 0.0].into(),
                Rgba::black(),
            );
            debug_lines_resource.draw_line(
                [aabb.halfsize_x as f32, -(aabb.halfsize_y as f32), 0.0].into(),
                [-(aabb.halfsize_x as f32), -(aabb.halfsize_y as f32), 0.0].into(),
                Rgba::black(),
            );
            debug_lines_resource.draw_line(
                [-(aabb.halfsize_x as f32), -(aabb.halfsize_y as f32), 0.0].into(),
                [-(aabb.halfsize_x as f32), aabb.halfsize_y as f32, 0.0].into(),
                Rgba::black(),
            );
        }
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
    transform.set_z(-10.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world.create_entity().with(transform).with(sprite).build()
}

// Initialize a sprite as a reference point at a fixed location
fn init_reference_sprite(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_x(100.0);
    transform.set_y(0.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Transparent)
        .build()
}

fn init_player(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_x(0.0);
    transform.set_y(0.0);
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
        .with(AABB { halfsize_x: 32, halfsize_y: 32 })
        .build()
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -960.0, 960.0, -540.0, 540.0,
        )))
        .with(transform)
        .with(PlayerCamera)
        .build();
}

struct Example;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let circle_sprite_sheet_handle =
            load_sprite_sheet(world, "Circle_Spritesheet.png", "Circle_Spritesheet.ron");
        let background_sprite_sheet_handle =
            load_sprite_sheet(world, "Background.png", "Background.ron");

        // let _background = init_background_sprite(world, &background_sprite_sheet_handle);
        let _reference = init_reference_sprite(world, &circle_sprite_sheet_handle);
        let _player = init_player(world, &circle_sprite_sheet_handle);
        init_camera(world);
        world.add_resource(DebugLines::new());
        world.add_resource(DebugLinesParams {
            line_width: 500.0
        });
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root = application_root_dir() + "/resources";
    let display_config_full_path = root.to_string() + "/display_config.ron";
    let config = DisplayConfig::load(&display_config_full_path);
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite), // Tells the pipeline to respect sprite z-depth
            ))
            .with_pass(DrawDebugLines::<PosColorNorm>::new()),
    );

    let input_full_path = root.to_string() + "/input.ron";
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(input_full_path)?,
        )?
        .with(MovementSystem, "movement", &[])
        .with(CameraFollowSystem, "camera_follow_system", &[])
        .with(DrawAABBSystem, "draw_aabb_system", &[])
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[]), // Let's us use the `Transparent` component
        )?;
    let mut game = Application::build(root, Example)?.build(game_data)?;
    game.run();
    Ok(())
}
