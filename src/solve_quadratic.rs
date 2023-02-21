
pub fn solve_quadratic(a:f64, b:f64, c:f64) -> (Option<f64>, Option<f64>) {
    let discriminant:f64 = b*b - 4.0 * a * c;
    if discriminant == 0.0 {
        let res:f64 = (-b) / (2.0 * a);
        return (Some(res), None)
    }
    if discriminant > 0.0 {
        let res1:f64 = (-b + discriminant.sqrt()) / (2.0 * a);
        let res2:f64 = (-b - discriminant.sqrt()) / (2.0 * a);
        return (Some(res1), Some(res2))
    }
    (None, None)
}
