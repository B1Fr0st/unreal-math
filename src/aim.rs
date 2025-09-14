use unreal_types::{FRotator,FVector};
pub fn calculate_aim(camera_position: &FVector, target_position: &FVector, z_offset:f32) -> FRotator {
    let mut newtarget = target_position.clone();
    newtarget.z += z_offset;
    let vec_delta = camera_position.sub(&newtarget);
    let hyp = (vec_delta.x * vec_delta.x + vec_delta.y * vec_delta.y).sqrt();

    let mut view_angles = FRotator {
        pitch: -(vec_delta.z / hyp).atan() * (180.0 / 3.14159),
        yaw: (vec_delta.y / vec_delta.x).atan() * (180.0 / 3.14159),
        roll: 0.0,
    };

    if vec_delta.x >= 0.0 {
        view_angles.yaw += 180.0;
    }

    view_angles
}

pub fn smooth_angle(current: f32, target: f32, smoothing: f32, max_delta: f32) -> f32 {
    let mut delta = target - current;

    if delta > 180.0 {
        delta -= 360.0;
    } else if delta < -180.0 {
        delta += 360.0;
    }
    delta *= smoothing;

    if delta.abs() > max_delta {
        current + max_delta * delta.signum()
    } else {
        current + delta
    }
}
pub fn normalize_angle(angle: f32) -> f32 {
    let normalized = angle % 360.0;
    if normalized < 0.0 {
        normalized + 360.0
    } else {
        normalized
    }
}

pub fn smooth_angles(current:FRotator, target:FRotator, smoothing: f32, max_delta:f32) -> FRotator{
    FRotator { 
        pitch: smooth_angle(current.pitch, target.pitch, smoothing, max_delta), 
        yaw: smooth_angle(current.yaw, target.yaw, smoothing, max_delta), 
        roll: 0.0 
    }
}