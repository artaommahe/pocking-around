use bevy::prelude::*;

pub struct Contact {
    pub penetration: f32,
    pub normal: Vec2,
}

pub fn ball_ball(pos_a: Vec2, radius_a: f32, pos_b: Vec2, radius_b: f32) -> Option<Contact> {
    let ab = pos_b - pos_a;
    let combined_radius = radius_a + radius_b;
    let ab_sqr_len = ab.length_squared();

    if ab_sqr_len < combined_radius * combined_radius {
        let ab_length = ab_sqr_len.sqrt();
        let penetration = combined_radius - ab.length();
        let normal = ab / ab_length;

        Some(Contact {
            normal,
            penetration,
        })
    } else {
        None
    }
}

pub fn ball_box(pos_a: Vec2, radius_a: f32, pos_b: Vec2, size_b: Vec2) -> Option<Contact> {
    let box_to_circle = pos_a - pos_b;
    let box_to_circle_abs = box_to_circle.abs();
    let half_extends = size_b / 2.;
    let corner_to_center = box_to_circle_abs - half_extends;
    let r = radius_a;

    if corner_to_center.x > r || corner_to_center.y > r {
        return None;
    }

    let s = box_to_circle.signum();

    let (normal, penetration) = if corner_to_center.x > 0. && corner_to_center.y > 0. {
        // corner case
        let corner_to_center_sqr = corner_to_center.length_squared();

        if corner_to_center_sqr > r * r {
            return None;
        }

        let cornder_dist = corner_to_center_sqr.sqrt();
        let penetration = r - cornder_dist;
        let normal = corner_to_center / cornder_dist * -s;

        (normal, penetration)
    } else if corner_to_center.x > corner_to_center.y {
        // closer to vertical edge
        (Vec2::X * -s.x, -corner_to_center.x + r)
    } else {
        (Vec2::Y * -s.y, -corner_to_center.y + r)
    };

    Some(Contact {
        normal,
        penetration,
    })
}
