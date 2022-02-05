use libm::cos;
// const _M: i32 = 5;

pub fn shuberts(x: f64, y: f64) -> f64 {
    let (mut s1, mut s2) = (0f64, 0f64);
    // for i in 1.._M + 1 {
    for i in 1..6 {
        let i = i as f64;
        s1 += i * cos((i + 1f64) * x + 1f64);
        s2 += i * cos((i + 1f64) * y + 1f64);
    }
    return -s1 * s2;
}
