use bevy::{prelude::*, utils::HashMap};

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entites: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entites: vec![],
        }
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entites: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // First phase: Detect collisions.
    for (entity_a, gtransform_a, collider_a) in query.iter() {
        for (entity_b, gtransform_b, collider_b) in query.iter() {
            if entity_a == entity_b {
                continue;
            }

            let distance = gtransform_a
                .translation()
                .distance(gtransform_b.translation());

            if distance < collider_a.radius + collider_b.radius {
                colliding_entites
                    .entry(entity_a)
                    .or_insert_with(Vec::new)
                    .push(entity_b);
            }
        }
    }

    // Second phase: Update colliders.
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entites.clear();
        if let Some(collisions) = colliding_entites.get(&entity) {
            collider.colliding_entites.extend(collisions.iter());
        }
    }

}
