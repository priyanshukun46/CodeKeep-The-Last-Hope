use bevy::prelude::*;

/// Brief laser beam tracer drawn when a turret discharges.
#[derive(Component)]
pub struct LaserTracer {
    pub from: Vec2,
    pub to: Vec2,
    pub timer: Timer,
}

/// Expanding plasma spark on projectile impact.
#[derive(Component)]
pub struct HitSpark {
    pub position: Vec2,
    pub timer: Timer,
}

/// Updates laser tracer lifecycles and renders their bright energy beams.
pub fn draw_laser_tracers_system(
    mut commands: Commands,
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut query: Query<(Entity, &mut LaserTracer)>,
) {
    for (entity, mut tracer) in query.iter_mut() {
        tracer.timer.tick(time.delta());
        let fraction = 1.0
            - (tracer.timer.elapsed_secs() / tracer.timer.duration().as_secs_f32()).clamp(0.0, 1.0);
        let alpha = fraction * 0.9;

        gizmos.line_2d(tracer.from, tracer.to, Color::srgba(1.0, 0.85, 0.25, alpha));

        if tracer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Updates impact sparks and renders expanding shock rings.
pub fn draw_hit_sparks_system(
    mut commands: Commands,
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut query: Query<(Entity, &mut HitSpark)>,
) {
    for (entity, mut spark) in query.iter_mut() {
        spark.timer.tick(time.delta());
        let progress =
            (spark.timer.elapsed_secs() / spark.timer.duration().as_secs_f32()).clamp(0.0, 1.0);
        let alpha = 1.0 - progress;
        let radius = 4.0 + progress * 16.0;

        gizmos.circle_2d(spark.position, radius, Color::srgba(1.0, 0.45, 0.15, alpha));

        if spark.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
