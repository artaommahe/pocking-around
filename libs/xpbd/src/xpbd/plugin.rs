use bevy::prelude::*;

use super::{
    colliders::*,
    components::*,
    consts::*,
    contact::{ball_ball, ball_box, box_box, Contact},
    resources::*,
    xpdb_loop::{first_substep, last_substep, run_criteria, XpbdLoop},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

#[derive(SystemLabel)]
enum Step {
    SolvePositions,
    SolveVelocities,
}

pub struct XpbdPlugin;

impl Plugin for XpbdPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<XpbdLoop>()
            .init_resource::<Gravity>()
            .init_resource::<Contacts>()
            .init_resource::<StaticContacts>()
            .init_resource::<CollisionPairs>()
            .add_stage_before(
                CoreStage::Update,
                FixedUpdateStage,
                SystemStage::parallel()
                    // TODO: use https://github.com/IyesGames/iyes_loopless instead
                    .with_run_criteria(run_criteria)
                    .with_system_set(
                        SystemSet::new()
                            .before(XpbdPlugin::collect_collision_pairs)
                            .with_system(XpbdPlugin::update_aabb_box)
                            .with_system(XpbdPlugin::update_aabb_circle),
                    )
                    .with_system(
                        XpbdPlugin::collect_collision_pairs.with_run_criteria(first_substep),
                    )
                    .with_system(XpbdPlugin::integrate.after(XpbdPlugin::collect_collision_pairs))
                    .with_system(XpbdPlugin::clear_contacs.before(Step::SolvePositions))
                    .with_system_set(
                        SystemSet::new()
                            .label(Step::SolvePositions)
                            .after(XpbdPlugin::integrate)
                            .with_system(XpbdPlugin::solve_pos)
                            .with_system(XpbdPlugin::solve_pos_box_box)
                            .with_system(XpbdPlugin::sol_pos_statics)
                            .with_system(XpbdPlugin::solve_pos_static_boxes)
                            .with_system(XpbdPlugin::solve_pos_static_box_box),
                    )
                    .with_system(XpbdPlugin::update_vel.after(Step::SolvePositions))
                    .with_system_set(
                        SystemSet::new()
                            .label(Step::SolveVelocities)
                            .after(XpbdPlugin::update_vel)
                            .with_system(XpbdPlugin::solve_vel)
                            .with_system(XpbdPlugin::solve_vel_static),
                    )
                    .with_system(
                        XpbdPlugin::sync_transforms
                            .with_run_criteria(last_substep)
                            .after(Step::SolveVelocities),
                    ),
            );
    }
}

impl XpbdPlugin {
    fn update_aabb_circle(mut query: Query<(&mut Aabb, &Pos, &Vel, &CircleCollider)>) {
        for (mut aabb, pos, vel, circle) in query.iter_mut() {
            let margin = COLLISION_PAIR_VEL_MARGIN_FACTOR * vel.0.length();
            let half_extents = Vec2::splat(circle.radius + margin);

            aabb.min = pos.0 - half_extents;
            aabb.max = pos.0 + half_extents;
        }
    }

    fn update_aabb_box(mut query: Query<(&mut Aabb, &Pos, &Vel, &BoxCollider)>) {
        for (mut aabb, pos, vel, box_) in query.iter_mut() {
            let margin = COLLISION_PAIR_VEL_MARGIN_FACTOR * vel.0.length();
            let half_extents = box_.size / 2. + Vec2::splat(margin);

            aabb.min = pos.0 - half_extents;
            aabb.max = pos.0 + half_extents;
        }
    }

    // TODO: optimize with hash grids
    fn collect_collision_pairs(
        query: Query<(Entity, &Aabb)>,
        mut collision_pairs: ResMut<CollisionPairs>,
    ) {
        collision_pairs.0.clear();

        let mut iter = query.iter_combinations();

        while let Some([(entity_a, aabb_a), (entity_b, aabb_b)]) = iter.fetch_next() {
            if aabb_a.intersects(aabb_b) {
                collision_pairs.0.push((entity_a, entity_b));
            }
        }
    }

    fn integrate(
        mut query: Query<(&mut Pos, &mut PrevPos, &mut Vel, &mut PreSolveVel, &Mass)>,
        gravity: Res<Gravity>,
    ) {
        for (mut pos, mut prev_pos, mut vel, mut pre_solve_vel, mass) in query.iter_mut() {
            prev_pos.0 = pos.0;

            let gravitation_force = mass.0 * gravity.0;
            let external_forces = gravitation_force;

            vel.0 += SUB_DT * external_forces / mass.0;
            pos.0 += SUB_DT * vel.0;
            pre_solve_vel.0 = vel.0;
        }
    }

    fn clear_contacs(mut contacts: ResMut<Contacts>, mut static_contacts: ResMut<StaticContacts>) {
        contacts.0.clear();
        static_contacts.0.clear();
    }

    fn solve_pos(
        mut query: Query<(&mut Pos, &CircleCollider, &Mass)>,
        collision_pairs: Res<CollisionPairs>,
        mut contacts: ResMut<Contacts>,
    ) {
        for (entity_a, entity_b) in collision_pairs.0.iter().cloned() {
            if let Ok([(mut pos_a, circle_a, mass_a), (mut pos_b, circle_b, mass_b)]) =
                query.get_many_mut([entity_a, entity_b])
            {
                if let Some(Contact {
                    penetration,
                    normal,
                }) = ball_ball(pos_a.0, circle_a.radius, pos_b.0, circle_b.radius)
                {
                    constrain_body_positions(
                        &mut pos_a,
                        &mut pos_b,
                        mass_a,
                        mass_b,
                        normal,
                        penetration,
                    );

                    contacts.0.push((entity_a, entity_b, normal));
                }
            }
        }
    }

    fn solve_pos_box_box(
        mut query: Query<(&mut Pos, &BoxCollider, &Mass)>,
        mut contacts: ResMut<Contacts>,
        collision_paris: Res<CollisionPairs>,
    ) {
        for (entity_a, entity_b) in collision_paris.0.iter().cloned() {
            if let Ok([(mut pos_a, box_a, mass_a), (mut pos_b, box_b, mass_b)]) =
                query.get_many_mut([entity_a, entity_b])
            {
                if let Some(Contact {
                    normal,
                    penetration,
                }) = box_box(pos_a.0, box_a.size, pos_b.0, box_b.size)
                {
                    constrain_body_positions(
                        &mut pos_a,
                        &mut pos_b,
                        mass_a,
                        mass_b,
                        normal,
                        penetration,
                    );

                    contacts.0.push((entity_a, entity_b, normal));
                }
            }
        }
    }

    fn sol_pos_statics(
        mut dynamics: Query<(Entity, &mut Pos, &CircleCollider), With<Mass>>,
        statics: Query<(Entity, &Pos, &CircleCollider), Without<Mass>>,
        mut contacts: ResMut<StaticContacts>,
    ) {
        for (entity_a, mut pos_a, circle_a) in dynamics.iter_mut() {
            for (entity_b, pos_b, circle_b) in statics.iter() {
                if let Some(Contact {
                    penetration,
                    normal,
                }) = ball_ball(pos_a.0, circle_a.radius, pos_b.0, circle_b.radius)
                {
                    constrain_body_position(&mut pos_a, normal, penetration);

                    contacts.0.push((entity_a, entity_b, normal));
                }
            }
        }
    }

    fn solve_pos_static_boxes(
        mut dynamics: Query<(Entity, &mut Pos, &CircleCollider), With<Mass>>,
        statics: Query<(Entity, &Pos, &BoxCollider), Without<Mass>>,
        mut contacts: ResMut<StaticContacts>,
    ) {
        for (entity_a, mut pos_a, circle_a) in dynamics.iter_mut() {
            for (entity_b, pos_b, box_b) in statics.iter() {
                if let Some(Contact {
                    normal,
                    penetration,
                }) = ball_box(pos_a.0, circle_a.radius, pos_b.0, box_b.size)
                {
                    constrain_body_position(&mut pos_a, normal, penetration);

                    contacts.0.push((entity_a, entity_b, normal));
                }
            }
        }
    }

    fn solve_pos_static_box_box(
        mut dynamics: Query<(Entity, &mut Pos, &BoxCollider), With<Mass>>,
        statics: Query<(Entity, &Pos, &BoxCollider), Without<Mass>>,
        mut contacts: ResMut<StaticContacts>,
    ) {
        for (entity_a, mut pos_a, box_a) in dynamics.iter_mut() {
            for (entity_b, pos_b, box_b) in statics.iter() {
                if let Some(Contact {
                    normal,
                    penetration,
                }) = box_box(pos_a.0, box_a.size, pos_b.0, box_b.size)
                {
                    constrain_body_position(&mut pos_a, normal, penetration);

                    contacts.0.push((entity_a, entity_b, normal));
                }
            }
        }
    }

    fn update_vel(mut query: Query<(&Pos, &PrevPos, &mut Vel, &Mass)>) {
        for (pos, prev_pos, mut vel, _mass) in query.iter_mut() {
            vel.0 = (pos.0 - prev_pos.0) / SUB_DT;
        }
    }

    fn solve_vel(
        mut query: Query<(&mut Vel, &PreSolveVel, &Mass, &Restitution)>,
        contacts: Res<Contacts>,
    ) {
        for (entity_a, entity_b, n) in contacts.0.iter().cloned() {
            let [(mut vel_a, pre_solve_vel_a, mass_a, restitution_a), (mut vel_b, pre_solve_vel_b, mass_b, restitution_b)] =
                query.get_many_mut([entity_a, entity_b]).unwrap();

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

    fn solve_vel_static(
        mut dynamics: Query<(&mut Vel, &PreSolveVel, &Restitution), With<Mass>>,
        statics: Query<&Restitution, Without<Mass>>,
        contacts: Res<StaticContacts>,
    ) {
        for (entity_a, entity_b, n) in contacts.0.iter().cloned() {
            let (mut vel_a, pre_solve_vel_a, restituin_a) = dynamics.get_mut(entity_a).unwrap();
            let restituin_b = statics.get(entity_b).unwrap();

            let pre_solve_normal_vel = Vec2::dot(pre_solve_vel_a.0, n);
            let normal_vel = Vec2::dot(vel_a.0, n);
            let restitution = (restituin_a.0 + restituin_b.0) / 2.;

            vel_a.0 += n * (-normal_vel - restitution * pre_solve_normal_vel);
        }
    }

    fn sync_transforms(mut query: Query<(&mut Transform, &Pos)>) {
        for (mut transform, pos) in query.iter_mut() {
            transform.translation = pos.0.extend(0.);
        }
    }
}

fn constrain_body_positions(
    pos_a: &mut Pos,
    pos_b: &mut Pos,
    mass_a: &Mass,
    mass_b: &Mass,
    normal: Vec2,
    penetration: f32,
) {
    let w_a = 1. / mass_a.0;
    let w_b = 1. / mass_b.0;
    let w_sum = w_a + w_b;
    let pos_impulse = normal * (-penetration / w_sum);

    pos_a.0 += pos_impulse * w_a;
    pos_b.0 -= pos_impulse * w_b;
}

fn constrain_body_position(pos: &mut Pos, normal: Vec2, penetration: f32) {
    pos.0 -= normal * penetration;
}
