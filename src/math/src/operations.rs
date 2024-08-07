pub fn multiply_quaternions(
    q1: (f64, f64, f64, f64),
    q2: (f64, f64, f64, f64),
) -> (f64, f64, f64, f64) {
    let q1 = normalize_quaternion(q1);
    let q2 = normalize_quaternion(q2);
    (
        q1.3 * q2.0 + q1.0 * q2.3 + q1.1 * q2.2 - q1.2 * q2.1,
        q1.3 * q2.1 - q1.0 * q2.2 + q1.1 * q2.3 + q1.2 * q2.0,
        q1.3 * q2.2 + q1.0 * q2.1 - q1.1 * q2.0 + q1.2 * q2.3,
        q1.3 * q2.3 - q1.0 * q2.0 - q1.1 * q2.1 - q1.2 * q2.2,
    )
}

fn normalize_quaternion(q: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
    let len = (q.0 * q.0 + q.1 * q.1 + q.2 * q.2 + q.3 * q.3).sqrt();
    (q.0 / len, q.1 / len, q.2 / len, q.3 / len)
}

pub fn quaternion_to_euler(q: (f64, f64, f64, f64)) -> (f64, f64, f64) {
    let (x, y, z, w) = normalize_quaternion(q);
    let ysqr = y * y;
    
    let t0 = 2.0 * (w * x + y * z);
    let t1 = 1.0 - 2.0 * (x * x + ysqr);
    let roll = t0.atan2(t1);
    
    let t2 = 2.0 * (w * y - z * x);
    let t2 = if t2 > 1.0 { 1.0 } else { t2 };
    let t2 = if t2 < -1.0 { -1.0 } else { t2 };
    let pitch = t2.asin();
    
    let t3 = 2.0 * (w * z + x * y);
    let t4 = 1.0 - 2.0 * (ysqr + z * z);
    let yaw = t3.atan2(t4);
    
    (roll, pitch, yaw)
}

pub fn euler_to_quaternion(roll: f64, pitch: f64, yaw: f64) -> (f64, f64, f64, f64) {
    let cy = (yaw * 0.5).cos();
    let sy = (yaw * 0.5).sin();
    let cp = (pitch * 0.5).cos();
    let sp = (pitch * 0.5).sin();
    let cr = (roll * 0.5).cos();
    let sr = (roll * 0.5).sin();

    let w = cr * cp * cy + sr * sp * sy;
    let x = sr * cp * cy - cr * sp * sy;
    let y = cr * sp * cy + sr * cp * sy;
    let z = cr * cp * sy - sr * sp * cy;

    (x, y, z, w)
}
