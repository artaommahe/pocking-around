use bevy::{prelude::*, time::FixedTimestep};

use super::{colliders::*, components::*, consts::*, resources::*};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

pub struct XpbdPlugin;

impl Plugin for XpbdPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Gravity>()
            .init_resource::<Contacts>()
            .add_stage_before(
                CoreStage::Update,
                FixedUpdateStage,
                SystemStage::parallel()
                    .with_run_criteria(FixedTimestep::step(DELTA_TIME as f64))
                    .with_system(XpbdPlugin::collect_collision_pairs)
                    .with_system(XpbdPlugin::integrate.after(XpbdPlugin::collect_collision_pairs))
                    .with_system(XpbdPlugin::solve_pos.after(XpbdPlugin::integrate))
                    .with_system(XpbdPlugin::update_vel.after(XpbdPlugin::solve_pos))
                    .with_system(XpbdPlugin::solve_vel.after(XpbdPlugin::update_vel))
                    .with_system(XpbdPlugin::sync_transforms.after(XpbdPlugin::solve_vel)),
            );
    }
}

impl XpbdPlugin {
    fn collect_collision_pairs() {}

    fn integrate(
        mut query: Query<(&mut Pos, &mut PrevPos, &mut Vel, &mut PreSolveVel, &Mass)>,
        gravity: Res<Gravity>,
    ) {
        for (mut pos, mut prev_pos, mut vel, mut pre_solve_vel, mass) in query.iter_mut() {
            prev_pos.0 = pos.0;

            let gravitation_force = mass.0 * gravity.0;
            let external_forces = gravitation_force;

            vel.0 += DELTA_TIME * external_forces / mass.0;
            pos.0 += DELTA_TIME * vel.0;
            pre_solve_vel.0 = vel.0;
        }
    }

    fn solve_pos(
        mut query: Query<(&mut Pos, &CircleCollider, &Mass, Entity)>,
        mut contacts: ResMut<Contacts>,
    ) {
        contacts.0.clear();

        let mut iter = query.iter_combinations_mut();

        while let Some(
            [(mut pos_a, circle_a, mass_a, entity_a), (mut pos_b, circle_b, mass_b, entity_b)],
        ) = iter.fetch_next()
        {
            let ab = pos_b.0 - pos_a.0;
            let combined_radius = circle_a.radius + circle_b.radius;
            let ab_sqr_len = ab.length_squared();

            if ab_sqr_len < combined_radius * combined_radius {
                let ab_length = ab_sqr_len.sqrt();
                let penetracion_depth = combined_radius - ab.length();
                let n = ab / ab_length;

                let w_a = 1. / mass_a.0;
                let w_b = 1. / mass_b.0;
                let w_sum = w_a + w_b;

                pos_a.0 -= n * penetracion_depth * w_a / w_sum;
                pos_b.0 += n * penetracion_depth * w_b / w_sum;

                contacts.0.push((entity_a, entity_b));
            }
        }
    }

    fn update_vel(mut query: Query<(&Pos, &PrevPos, &mut Vel, &Mass)>) {
        for (pos, prev_pos, mut vel, _mass) in query.iter_mut() {
            vel.0 = (pos.0 - prev_pos.0) / DELTA_TIME;
        }
    }

    fn solve_vel(
        mut query: Query<(&mut Vel, &PreSolveVel, &Pos, &Mass, &Restitution)>,
        contacts: Res<Contacts>,
    ) {
        for (entity_a, entity_b) in contacts.0.iter().cloned() {
            let [(mut vel_a, pre_solve_vel_a, pos_a, mass_a, restitution_a), (mut vel_b, pre_solve_vel_b, pos_b, mass_b, restitution_b)] =
                query.get_many_mut([entity_a, entity_b]).unwrap();

            let n = (pos_b.0 - pos_a.0).normalize();
            let pre_solve_relative_vel = pre_solve_vel_a.0 - pre_solve_vel_b.0;
            let pre_solve_normal_vel = Vec2::dot(pre_solve_relative_vel, n);

            let relative_vel = vel_a.0 - vel_b.0;
            let normal_vel = Vec2::dot(relative_vel, n);
            let restitution = (restitution_a.0 + restitution_b.0) / 2.;

            let w_a = 1. / mass_a.0;
            let w_b = 1. / mass_b.0;
            let w_sum = w_a + w_b;

            vel_a.0 += n * (-normal_vel - restitution * pre_solve_normal_vel) * w_a / w_sum;
            vel_b.0 -= n * (-normal_vel - restitution * pre_solve_normal_vel) * w_b / w_sum;
        }
    }

    fn sync_transforms(mut query: Query<(&mut Transform, &Pos)>) {
        for (mut transform, pos) in query.iter_mut() {
            transform.translation = pos.0.extend(0.);
        }
    }
}