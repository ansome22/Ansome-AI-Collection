use glam::f32::Vec3;

pub struct Waypoint{
    // The position of the waypoint.
    position: Vec3,
    // The value of the waypoint for the tactic we are condensing.
    value: f64
}