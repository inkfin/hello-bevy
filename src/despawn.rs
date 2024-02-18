use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_far_away_entities);
    }
}

fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, gtransform) in query.iter() {
        if gtransform.translation().distance(Vec3::ZERO) > DESPAWN_DISTANCE {
            // recursivly despawn will generate many errors (doesn't exist in this world)
            commands.entity(entity).despawn_recursive();
        }
    }
}