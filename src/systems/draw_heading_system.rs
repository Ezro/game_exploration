// struct DrawHeadingSystem;
// impl<'s> System<'s> for DrawHeadingSystem {
//     type SystemData = (
//         Write<'s, DebugLines>,
//         ReadStorage<'s, Player>,
//         ReadStorage<'s, Position>,
//         ReadStorage<'s, Heading>,
//         ReadStorage<'s, PlayerCamera>,
//         ReadStorage<'s, Camera>,
//     );

//     fn run(
//         &mut self,
//         (mut debug_lines_resource, players, positions, headings, player_cameras, cameras): Self::SystemData,
//     ) {
//     }
// }
