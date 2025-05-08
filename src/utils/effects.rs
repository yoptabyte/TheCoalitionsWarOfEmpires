use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use crate::game::ClickEffectHandle;

/// setup particle effect.
pub fn setup_particle_effect(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(1.0, 1.0, 0.0, 1.0)); // yellow start
    color_gradient.add_key(0.5, Vec4::new(1.0, 0.5, 0.0, 1.0)); // orange middle
    color_gradient.add_key(1.0, Vec4::new(1.0, 0.0, 0.0, 0.0)); // red and transparent end

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec2::splat(0.1));
    size_gradient.add_key(0.8, Vec2::splat(0.05));
    size_gradient.add_key(1.0, Vec2::splat(0.0));

    let writer = ExprWriter::new();

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(0.1).expr(), // small radius
        dimension: ShapeDimension::Volume,
    };

    // particles shoot mainly up
    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::NEG_Y * 0.5).expr(), // center *below* the creation point
        speed: writer.lit(1.0).uniform(writer.lit(2.0)).expr(),
    };

    let lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        writer.lit(0.6).uniform(writer.lit(1.0)).expr(), // random lifetime
    );

    let effect = effects.add(
        EffectAsset::new(
            32, // effect capacity
            Spawner::once(30.0.into(), true), // create 30 particles once
            writer.finish(),
        )
        .with_name("click_effect")
        .init(init_pos)
        .init(init_vel)
        .init(lifetime)
        .render(ColorOverLifetimeModifier { gradient: color_gradient })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false, // size in world units
        }),
    );

    commands.insert_resource(ClickEffectHandle(effect));
}