use amethyst::{
    Result,
    core::bundle::SystemBundle,
    // core::ecs::WorldExt,
    ecs::DispatcherBuilder,
    prelude::World,
};

// use crate::component::{
//     block::Block,
//     ball::Ball,
//     bar::Bar,
// };

// use crate::system::{
//     translation::TranslationSystem,
//     player::PlayerSystem,
//     collision::CollisionSystem
// };

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, _world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        // _world.register::<Block>();
        // _world.register::<Ball>();
        // _world.reister::<Bar>();

        // builder.add(TranslationSystem, "translation_system", &[]);
        // builder.add(PlayerSystem, "player_system", &[]);
        // builder.add(CollisionSystem, "collision_system", &[]);
        Ok(())
    }
}
