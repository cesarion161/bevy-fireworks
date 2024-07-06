use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Debug, Default, Clone)]
struct Particle {
    velocity: Vec3,
    life: f32,
}

#[derive(Component, Debug, Default, Clone)]
struct Firework {
    spawn_position: Vec3,
    particles_count: usize,
    life: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_particles)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // // Spawn initial particles
    for _ in 0..3 {
        spawn_firework(&mut commands);
    }

}

fn spawn_firework(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-300.0..300.0);
    let y = rng.gen_range(-100.0..300.0);
    let life = rng.gen_range(1.0..3.0);
    let particles_count = 100;

    let firework_position = Vec3::new(x, y, 0.0);

    // Create the firework entity
    commands.spawn((
        Firework {
            spawn_position: firework_position,
            particles_count,
            life,
        },
    ));

    for _ in 0..particles_count {
        let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
        let speed = rng.gen_range(0.1..5.); // Stronger impulse
        let velocity = Vec3::new(speed * angle.cos(), speed * angle.sin(), 0.0);

        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(firework_position),
            sprite: Sprite {
                custom_size: Some(Vec2::new(5.0, 5.0)),
                color: Color::srgb(
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                ),
                ..Default::default()
            },
            ..Default::default()
        })
            .insert(Particle {
                velocity,
                life,
            });
    }
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Particle)>,
) {
    for (entity, mut transform, mut particle) in query.iter_mut() {
        particle.life -= time.delta_seconds();
        if particle.life <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            transform.translation += particle.velocity;
            particle.velocity.y -= 0.05; // Gravity effect
        }
    }

    // If there are no more particles, spawn a new firework
    if query.is_empty() {
        spawn_firework(&mut commands);
    }
}
