// ================    Region 4 ===================

const REGION_4_SATURATION_COEFFS: [f64; 10] = [
    0.11670521452767e4,
    -0.72421316703206e6,
    -0.17073846940092e2,
    0.12020824702470e5,
    -0.32325550322333e7,
    0.14915108613530e2,
    -0.48232657361591e4,
    0.40511340542057e6,
    -0.23855557567849,
    0.65017534844798e3,
];

/// Returns the saturation pressure in Pa
/// Temperature is assumed to be in K
pub fn psat97(t: f64) -> f64 {
    let n1 = REGION_4_SATURATION_COEFFS[0];
    let n2 = REGION_4_SATURATION_COEFFS[1];
    let n3 = REGION_4_SATURATION_COEFFS[2];
    let n4 = REGION_4_SATURATION_COEFFS[3];
    let n5 = REGION_4_SATURATION_COEFFS[4];
    let n6 = REGION_4_SATURATION_COEFFS[5];
    let n7 = REGION_4_SATURATION_COEFFS[6];
    let n8 = REGION_4_SATURATION_COEFFS[7];
    let n9 = REGION_4_SATURATION_COEFFS[8];
    let n10 = REGION_4_SATURATION_COEFFS[9];

    let theta = t + n9 / (t - n10);

    let coef_a = theta * theta + n1 * theta + n2;
    let coef_b = n3 * theta * theta + n4 * theta + n5;
    let coef_c = n6 * theta * theta + n7 * theta + n8;
    (2.0 * coef_c / (-coef_b + (coef_b * coef_b - 4.0 * coef_a * coef_c).sqrt())).powi(4) * 1e6
}

/// Returns the saturation temperature in K
/// dead_codePressure is assumed to be in Pa
#[allow(dead_code)]
pub fn tsat97(p: f64) -> f64 {
    let n1 = REGION_4_SATURATION_COEFFS[0];
    let n2 = REGION_4_SATURATION_COEFFS[1];
    let n3 = REGION_4_SATURATION_COEFFS[2];
    let n4 = REGION_4_SATURATION_COEFFS[3];
    let n5 = REGION_4_SATURATION_COEFFS[4];
    let n6 = REGION_4_SATURATION_COEFFS[5];
    let n7 = REGION_4_SATURATION_COEFFS[6];
    let n8 = REGION_4_SATURATION_COEFFS[7];
    let n9 = REGION_4_SATURATION_COEFFS[8];
    let n10 = REGION_4_SATURATION_COEFFS[9];

    let beta = (p / 1e6).powf(0.25);

    let coef_e = beta * beta + n3 * beta + n6;
    let coef_f = n1 * beta * beta + n4 * beta + n7;
    let coef_g = n2 * beta * beta + n5 * beta + n8;

    let coef_d = 2.0 * coef_g / (-coef_f - (coef_f * coef_f - 4.0 * coef_e * coef_g).sqrt());

    (n10 + coef_d - ((n10 + coef_d).powi(2) - 4.0 * (n9 + n10 * coef_d)).sqrt()) / 2.0
}
