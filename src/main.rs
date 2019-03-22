#[macro_use]
extern crate lazy_static;

use amethyst;

pub mod components;
pub mod states;
pub mod systems;

use self::components::*;
use self::states::*;
use self::systems::*;
use self::systems::physics::*;

use amethyst::{
    core::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        DisplayConfig, DrawDebugLines, DrawFlat2D, Pipeline, PosColorNorm, RenderBundle, Stage,
    },
    utils::application_root_dir,
};

use amethyst_editor_sync::*;
use tap::*;

use std::time::UNIX_EPOCH;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let editor_sync_bundle = SyncEditorBundle::default()
        .tap(SyncEditorBundle::sync_default_types)
        .tap(|bundle| sync_components!(bundle, AABB, Heading, PlayerCamera, Player, Position, Size, Speed));

    let root = application_root_dir()?.join("resources");
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawDebugLines::<PosColorNorm>::new()),
    );
    let input_full_path = root.join("input.ron");
    let display_config_full_path = root.join(DISPLAY_CONFIG_FILENAME);
    let display_config = DisplayConfig::load(&display_config_full_path);
    let config = CustomConfig {
        last_check_time: UNIX_EPOCH,
    };
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        // Config
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(input_full_path)?,
        )?
        .with(ConfigSystem { config }, "config_system", &[])
        // Movement
        .with(MovementSystem, "movement_system", &[])
        .with(HeadingSystem, "heading_system", &[])
        .with(AABBFollowSystem, "aabb_follow_system", &[])
        .with(CameraFollowSystem, "camera_follow_system", &[])
        // Physics
        .with(GenerateCollisionManifoldsSystem, "generate_collision_manifolds_system", &[])
        .with(GenerateConstraintsSystem, "generate_constraints_system", &[])
        // Debug
        .with(DrawAABBSystem, "draw_aabb_system", &[])
        .with(DrawHeadingSystem, "draw_heading_system", &[])
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[]), // Let's us use the `Transparent` component
        )?
        .with_bundle(editor_sync_bundle)?;
    let mut game = Application::build(root, Example)?.build(game_data)?;
    game.run();
    Ok(())
}
