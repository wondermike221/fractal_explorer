pub fn interpolate(
    from_point: f64,
    from_start: f64,
    from_end: f64,
    to_start: f64,
    to_end: f64,
) -> f64 {
    to_start + (to_end - to_start) * ((from_point - from_start) / (from_end - from_start))
}
