use amethyst::{
    SimpleState,
    StateData,
    GameData,
    SimpleTrans,
    Trans,
};

#[derive(Default)]
pub struct StartState;

impl SimpleState for StartState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;
        Trans::None
    }
}