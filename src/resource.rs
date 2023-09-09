use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Money(pub f32);

impl Default for Money {
    fn default() -> Self {
        Money(0.0)
    }
}
