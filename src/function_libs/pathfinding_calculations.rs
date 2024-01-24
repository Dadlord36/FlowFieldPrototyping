use bevy::math::{Quat, Vec3};
use bevy::prelude::{Transform, UVec2, Vec2};
use crate::components::grid_components::GridParameters;

use crate::components::movement_components::{Maneuver, SurfaceCoordinate};

impl Maneuver {
    pub fn new(surface_coordinates: Vec<SurfaceCoordinate>) -> Self {
        Self {
            path_points: surface_coordinates,
            progress: 0.0,
        }
    }

    pub fn zigzag(grid_parameters: &GridParameters) -> Self{
        let mut maneuver_points = vec![];

        for i in 0..25 {
            if i % 2 == 0 {
                for j in 0..25 {
                    maneuver_points.push(grid_parameters.calculate_flat_surface_coordinate_from(UVec2::new(i, j)));
                }
            } else {
                for j in (0..25).rev() {
                    maneuver_points.push(grid_parameters.calculate_flat_surface_coordinate_from(UVec2::new(i, j)));
                }
            }
        }
        maneuver_points.shrink_to_fit();

        Maneuver{
            path_points: maneuver_points,
            progress: 0.0
        }
    }

    /*    pub fn straight_interpolate_along_path(&self, progress: f32) -> Transform {
            interpolate_straight(progress, &self.path_points)
        }*/

    pub fn catmull_rom_interpolate_along_path_ping_pong(&mut self, progress: f32) -> SurfaceCoordinate {
        self.progress = ping_pong(progress, 1.0);
        catmull_rom_interpolate_coordinates(self.progress, &self.path_points)
    }

    pub fn catmull_rom_interpolate_along_path(&mut self, progress: f32) -> SurfaceCoordinate {
        self.progress = (self.progress + progress).clamp(0.0, 1.0);
        catmull_rom_interpolate_coordinates(self.progress, &self.path_points)
    }

    pub fn is_done(&self) -> bool {
        1.0 - self.progress < 0.01
    }
}

/// Interpolates transforms along a Bezier curve at a given parameter value.
///
/// # Arguments
///
/// * `t` - The parameter value between 0 and 1.
/// * `transforms` - A vector of transforms representing control points.
///
/// # Returns
///
/// The interpolated transform at the given parameter value.
pub fn bezier_interpolate(t: f32, transforms: &[Transform]) -> Transform {
    let count = transforms.len();
    let mut new_transform = Transform::IDENTITY;

    let precomputed_values: Vec<f32> = (0..count).map(|i| {
        let binom = binomial_coefficient((count - 1) as i32, i as i32);
        let power = (1.0 - t).powf((count - 1 - i) as f32) * t.powf(i as f32);
        binom * power
    }).collect();

    new_transform.translation = transforms.iter().zip(&precomputed_values).map(|(transform, &precomputed_value)| {
        precomputed_value * transform.translation
    }).sum();

    new_transform.rotation = transforms.iter().zip(&precomputed_values).map(|(transform, &precomputed_value)| {
        fast_pow(transform.rotation, precomputed_value)
    }).product();

    new_transform
}

#[inline]
pub fn interpolate_straight(t: f32, waypoints: &[Transform]) -> Transform {
    let t = t * (waypoints.len() - 1) as f32;
    let index = t as usize;
    let t = t.fract();

    let a = &waypoints[index];
    let b = &waypoints[index + 1];

    Transform {
        translation: a.translation.lerp(b.translation, t),
        rotation: a.rotation.lerp(b.rotation, t),
        scale: a.scale.lerp(b.scale, t),
    }
}

/// Calculates the binomial coefficient (n choose k)
///
/// # Arguments
///
/// * `n` - The total number of items
/// * `k` - The number of items to choose
///
/// # Returns
///
/// The binomial coefficient
///
/// # Examples
///
/// ```
/// let result = binomial_coefficient(5, 2);
/// assert_eq!(result, 10.0);
/// ```
#[inline]
pub fn binomial_coefficient(n: i32, k: i32) -> f32 {
    let mut result = 1.0;
    for i in 1..=k {
        result *= n as f32 - k as f32 + i as f32;
        result /= i as f32;
    }
    result
}

#[inline]
pub fn faster_pow(a: f32, b: f32) -> f32 {
    (b * a.log10()).exp2()
}

#[inline]
pub fn fast_pow(quat: Quat, power: f32) -> Quat {
    let (axis, angle) = quat.to_axis_angle();
    Quat::from_axis_angle(axis, angle * power)
}

#[inline]
fn ping_pong(value: f32, max_value: f32) -> f32 {
    let modulo = value % (max_value * 2.0);
    if modulo < max_value {
        modulo
    } else {
        max_value * 2.0 - modulo
    }
}

/// Performs Catmull-Rom interpolation between control points to calculate the transform at a given progress.
///
/// # Arguments
///
/// * `progress` - A floating-point number representing the progress of the interpolation. Should be in the range [0, 1].
/// * `points` - A slice of `Transform` structs representing the control points for the interpolation.
///
/// # Returns
///
/// The interpolated `Transform` at the given progress.
///
/// # Examples
///
/// ```
/// use my_library::{catmull_rom_interpolate, Transform};
///
/// let points = [
///     Transform { translation: (0.0, 0.0, 0.0), rotation: (0.0, 0.0, 0.0), scale: (1.0, 1.0, 1.0) },
///     Transform { translation: (1.0, 1.0, 1.0), rotation: (0.0, 0.0, 0.0), scale: (1.0, 1.0, 1.0) },
///     Transform { translation: (2.0, 2.0, 2.0), rotation: (0.0, 0.0, 0.0), scale: (1.0, 1.0, 1.0) },
///     Transform { translation: (3.0, 3.0, 3.0), rotation: (0.0, 0.0, 0.0), scale: (1.0, 1.0, 1.0) },
/// ];
///
/// let interpolated_transform = catmull_rom_interpolate(0.5, &points);
/// ```
#[inline]
pub fn catmull_rom_interpolate_transforms(progress: f32, points: &[Transform]) -> Transform {
    let (t_sec, t, t0, t1, t2, t3) = calculate_interpolation_parameters(progress, points.len());

    let p0 = points[t0].translation;
    let p1 = points[t1].translation;
    let p2 = points[t2].translation;
    let p3 = points[t3].translation;

    let q0 = points[t0].rotation;
    let q1 = points[t1].rotation;
    let q2 = points[t2].rotation;

    let new_pos = catmull_rom_interp_vec3(p1, p0, p2, t, p3);
    let new_rot = interpolate_rotation(t_sec, t, t1, q0, q1, q2);

    Transform {
        translation: new_pos,
        rotation: new_rot,
        scale: Vec3::ONE,
    }
}

#[inline]
fn calculate_interpolation_parameters(progress: f32, points_num: usize) -> (usize, f32, usize, usize, usize, usize) {
    let num_sections = points_num - 1;
    let t_sec = f32::floor((progress * num_sections as f32)) as usize;
    let t = progress * num_sections as f32 - (t_sec as f32);

    // Control points adjustment
    let t0 = if t_sec == 0 { 0 } else { t_sec - 1 };
    let t1 = t_sec;
    let t2 = usize::min(num_sections, t_sec + 1);
    let t3 = usize::min(num_sections, t_sec + 2);
    (t_sec, t, t0, t1, t2, t3)
}

#[inline]
pub fn catmull_rom_interpolate_coordinates(progress: f32, points: &[SurfaceCoordinate]) -> SurfaceCoordinate {
    let (t_sec, t, t0, t1, t2, t3) = calculate_interpolation_parameters(progress, points.len());

    let p0: Vec2 = points[t0].into();
    let p1: Vec2 = points[t1].into();
    let p2: Vec2 = points[t2].into();
    let p3: Vec2 = points[t3].into();

    let new_pos = catmull_rom_interp_vec2(p1, p0, p2, t, p3);
    SurfaceCoordinate {
        latitude: new_pos.x,
        longitude: new_pos.y,
    }
}

#[inline]
fn interpolate_rotation(t_sec: usize, t: f32, t1: usize, q0: Quat, q1: Quat, q2: Quat) -> Quat {
    if t_sec == t1 {
        q1.lerp(q2, t)
    } else {
        q0.lerp(q1, t)
    }
}

/// Performs Catmull-Rom interpolation on 4 control points and a given parameter.
///
/// This function calculates the interpolated value between two control points, `p1` and `p2`,
/// using two neighboring control points `p0` and `p3`. The parameter `t` represents the
/// interpolation parameter, which determines how close the interpolated value is to `p1` and `p2`.
///
/// # Arguments
/// * `p1` - The second control point. This is the point to be interpolated towards.
/// * `p0` - The first control point. This is the point before `p1`.
/// * `p2` - The third control point. This is the point after `p1`.
/// * `t` - The interpolation parameter. This determines how close the interpolated value is to `p1`
///         and `p2`. It should be a value between 0.0 and 1.0.
/// * `p3` - The fourth control point. This is the point after `p2`.
///
/// # Returns
/// The interpolated value between `p1` and `p2` based on the `t` parameter.
#[inline]
fn catmull_rom_interp_vec3(p1: Vec3, p0: Vec3, p2: Vec3, t: f32, p3: Vec3) -> Vec3 {
    0.5 * (2.0 * p1 + (-p0 + p2) * t +
        (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t.powi(2) +
        (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t.powi(3))
}

#[inline]
fn catmull_rom_interp_vec2(p1: Vec2, p0: Vec2, p2: Vec2, t: f32, p3: Vec2) -> Vec2 {
    0.5 * (2.0 * p1 + (-p0 + p2) * t +
        (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t.powi(2) +
        (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t.powi(3))
}

#[inline]
fn catmull_rom_interp_f32(p1: f32, p0: f32, p2: f32, t: f32, p3: f32) -> f32 {
    0.5 * (2.0 * p1 + (-p0 + p2) * t +
        (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t.powi(2) +
        (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t.powi(3))
}