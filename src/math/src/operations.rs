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
