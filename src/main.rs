use amethyst;

pub mod components;
pub mod states;
pub mod systems;

use self::states::*;
use self::systems::*;

use amethyst::{
    core::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        DisplayConfig, DrawDebugLines, DrawFlat2D, Pipeline, PosColorNorm, RenderBundle, Stage,
    },
    utils::application_root_dir,
};

use std::time::UNIX_EPOCH;

fn main() -> amethyst::Result<()> {
    // amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
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
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(input_full_path)?,
        )?
        .with(ConfigSystem { config }, "config_system", &[])
        .with(MovementSystem, "movement_system", &[])
        .with(HeadingSystem, "heading_system", &[])
        .with(AABBFollowSystem, "aabb_follow_system", &[])
        .with(CameraFollowSystem, "camera_follow_system", &[])
        .with(DrawAABBSystem, "draw_aabb_system", &[])
        .with(DrawHeadingSystem, "draw_heading_system", &[])
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[]), // Let's us use the `Transparent` component
        )?;
    let mut game = Application::build(root, Example)?.build(game_data)?;
    game.run();
    Ok(())
}
