use crate::iapws97::constants;

const REGION_5_COEFFS_RES_II: [i32; 6] = [1, 1, 1, 2, 2, 3];

const REGION_5_COEFFS_RES_JI: [i32; 6] = [1, 2, 3, 3, 9, 7];

const REGION_5_COEFFS_RES_NI: [f64; 6] = [
    0.15736404855259e-2,
    0.90153761673944e-3,
    -0.50270077677648e-2,
    0.22440037409485e-5,
    -0.41163275453471e-5,
    0.37919454822955e-7,
];

const REGION_5_COEFFS_IDEAL_JI: [i32; 6] = [0, 1, -3, -2, -1, 2];

const REGION_5_COEFFS_IDEAL_NI: [f64; 6] = [
    -0.13179983674201e2,
    0.68540841634434e1,
    -0.24805148933466e-1,
    0.36901534980333,
    -0.31161318213925e1,
    -0.32961626538917,
];

// ================    Region 5 ===================

/// Returns the region-5 tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline(always)]
fn tau_5(t: f64) -> f64 {
    1000.0_f64 / t
}

/// Returns the region-5 pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline(always)]
fn pi_5(p: f64) -> f64 {
    p / 1e6
}

/// Returns the region-5 ideal gamma
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_5_ideal(t: f64, p: f64) -> f64 {
    let pi = pi_5(p);
    let tau: f64 = tau_5(t);
    let mut sum = 0.0_f64;
    for x in 0..REGION_5_COEFFS_IDEAL_JI.len() {
        let ji = REGION_5_COEFFS_IDEAL_JI[x];
        let ni = REGION_5_COEFFS_IDEAL_NI[x];
        sum += ni * tau.powi(ji);
    }
    sum + pi.ln()
}

/// Returns the region-2 residual gamma
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_5_res(t: f64, p: f64) -> f64 {
    let pi = pi_5(p);
    let tau = tau_5(t);
    let mut sum = 0.0_f64;
    for x in 0..REGION_5_COEFFS_RES_II.len() {
        let ii = REGION_5_COEFFS_RES_II[x];
        let ji = REGION_5_COEFFS_RES_JI[x];
        let ni = REGION_5_COEFFS_RES_NI[x];
        sum += ni * pi.powi(ii) * tau.powi(ji);
    }
    sum
}

/// Returns the region-5 ideal gamma_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_5_ideal(t: f64, _: f64) -> f64 {
    let tau = tau_5(t);
    let mut sum = 0.0_f64;
    for x in 0..REGION_5_COEFFS_IDEAL_JI.len() {
        let ji = REGION_5_COEFFS_IDEAL_JI[x];
        let ni = REGION_5_COEFFS_IDEAL_NI[x];
        sum += ni * ji as f64 * tau.powi(ji - 1);
    }
    sum
}

/// Returns the region-5 ideal gamma_tau_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_tau_5_ideal(t: f64, _: f64) -> f64 {
    let tau = tau_5(t);
    let mut sum = 0.0_f64;
    for x in 0..REGION_5_COEFFS_IDEAL_JI.len() {
        let ji = REGION_5_COEFFS_IDEAL_JI[x];
        let ni = REGION_5_COEFFS_IDEAL_NI[x];
        sum += ni * (ji * (ji - 1)) as f64 * tau.powi(ji - 2);
    }
    sum
}

/// Returns the region-5 ideal gamma_pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline(always)]
fn gamma_pi_5_ideal(_: f64, p: f64) -> f64 {
    1.0_f64 / pi_5(p)
}

/// Returns the region-5 residual gamma_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_5_res(t: f64, p: f64) -> f64 {
    let tau = tau_5(t);
    let pi = pi_5(p);
    let mut sum: f64 = 0.0;
    for x in 0..REGION_5_COEFFS_RES_II.len() {
        let ii = REGION_5_COEFFS_RES_II[x];
        let ji = REGION_5_COEFFS_RES_JI[x];
        let ni = REGION_5_COEFFS_RES_NI[x];
        sum += ni * pi.powi(ii) * ji as f64 * tau.powi(ji - 1);
    }
    sum
}

/// Returns the region-5 residual gamma_tau_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_tau_5_res(t: f64, p: f64) -> f64 {
    let tau = tau_5(t);
    let pi = pi_5(p);
    let mut sum: f64 = 0.0_f64;
    for x in 0..REGION_5_COEFFS_RES_II.len() {
        let ii = REGION_5_COEFFS_RES_II[x];
        let ji = REGION_5_COEFFS_RES_JI[x];
        let ni = REGION_5_COEFFS_RES_NI[x];
        sum += ni * pi.powi(ii) * (ji * (ji - 1)) as f64 * tau.powi(ji - 2);
    }
    sum
}

/// Returns the region-5 residual gamma_pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_5_res(t: f64, p: f64) -> f64 {
    let tau = tau_5(t);
    let pi = pi_5(p);
    let mut sum = 0.0_f64;
    for x in 0..REGION_5_COEFFS_RES_II.len() {
        let ii = REGION_5_COEFFS_RES_II[x];
        let ji = REGION_5_COEFFS_RES_JI[x];
        let ni = REGION_5_COEFFS_RES_NI[x];
        sum += ni * ii as f64 * pi.powi(ii - 1) * tau.powi(ji);
    }
    sum
}

/// Returns the region-5 residual gamma_pi_pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_pi_5_res(t: f64, p: f64) -> f64 {
    let tau = tau_5(t);
    let pi = pi_5(p);
    let mut sum = 0.0_f64;
    for x in 0..REGION_5_COEFFS_RES_II.len() {
        let ii = REGION_5_COEFFS_RES_II[x];
        let ji = REGION_5_COEFFS_RES_JI[x];
        let ni = REGION_5_COEFFS_RES_NI[x];
        sum += ni * (ii * (ii - 1)) as f64 * pi.powi(ii - 2) * tau.powi(ji);
    }
    sum
}

/// Returns the region-5 residual gamma_pi_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_tau_5_res(t: f64, p: f64) -> f64 {
    let tau = tau_5(t);
    let pi = pi_5(p);
    let mut sum = 0.0_f64;
    for x in 0..REGION_5_COEFFS_RES_II.len() {
        let ii = REGION_5_COEFFS_RES_II[x];
        let ji = REGION_5_COEFFS_RES_JI[x];
        let ni = REGION_5_COEFFS_RES_NI[x];
        sum += ni * ii as f64 * pi.powi(ii - 1) * ji as f64 * tau.powi(ji - 1);
    }
    sum
}

/// Returns the region-5 specific volume
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline]
pub(crate) fn v_tp_5(t: f64, p: f64) -> f64 {
    ((constants::_R * 1000.0) * t / p) * pi_5(p) * (gamma_pi_5_ideal(t, p) + gamma_pi_5_res(t, p))
}

/// Returns the region-5 enthalpy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline]
pub(crate) fn h_tp_5(t: f64, p: f64) -> f64 {
    constants::_R * t * tau_5(t) * (gamma_tau_5_ideal(t, p) + gamma_tau_5_res(t, p))
}

/// Returns the region-5 internal energy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline]
pub(crate) fn u_tp_5(t: f64, p: f64) -> f64 {
    let tau = tau_5(t);
    let pi = pi_5(p);
    constants::_R
        * t
        * (tau * (gamma_tau_5_ideal(t, p) + gamma_tau_5_res(t, p))
            - pi * (gamma_pi_5_ideal(t, p) + gamma_pi_5_res(t, p)))
}

/// Returns the region-5 entropy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline]
pub(crate) fn s_tp_5(t: f64, p: f64) -> f64 {
    let tau = tau_5(t);
    constants::_R
        * (tau * (gamma_tau_5_ideal(t, p) + gamma_tau_5_res(t, p))
            - (gamma_5_ideal(t, p) + gamma_5_res(t, p)))
}

/// Returns the region-5 isobaric specific heat
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline]
pub(crate) fn cp_tp_5(t: f64, p: f64) -> f64 {
    -constants::_R * tau_5(t).powi(2) * (gamma_tau_tau_5_ideal(t, p) + gamma_tau_tau_5_res(t, p))
}

/// Returns the region-5 isochoric specific heat
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline]
pub(crate) fn cv_tp_5(t: f64, p: f64) -> f64 {
    let pi = pi_5(p);
    cp_tp_5(t, p)
        - constants::_R
            * (((1.0 + pi * gamma_pi_5_res(t, p) - tau_5(t) * pi * gamma_pi_tau_5_res(t, p))
                .powi(2))
                / (1.0 - pi.powi(2) * gamma_pi_pi_5_res(t, p)))
}

/// Returns the region-5 sound velocity
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline]
pub(crate) fn w_tp_5(t: f64, p: f64) -> f64 {
    let tau = tau_5(t);
    let pi = pi_5(p);
    let num =
        1.0_f64 + 2.0_f64 * pi * gamma_pi_5_res(t, p) + pi.powi(2) * gamma_pi_5_res(t, p).powi(2);
    let subnum: f64 =
        (1.0_f64 + pi * gamma_pi_5_res(t, p) - tau * pi * gamma_pi_tau_5_res(t, p)).powi(2);
    let subden: f64 = tau.powi(2) * (gamma_tau_tau_5_ideal(t, p) + gamma_tau_tau_5_res(t, p));
    let den: f64 = 1.0 - pi.powi(2) * gamma_pi_pi_5_res(t, p) + subnum / subden;
    ((constants::_R * 1000.0_f64 * t) * num / den).sqrt()
}
