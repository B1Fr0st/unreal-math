use unreal_types::{FVector, FRotator};

pub fn calculate_aim(camera_position: &FVector, target_position: &FVector) -> FRotator {
    let vec_delta = camera_position.sub(target_position);
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