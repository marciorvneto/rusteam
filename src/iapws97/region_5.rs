use crate::iapws97::constants;

const REGION_5_COEFFS_RES: [[f64; 3]; 6] = [
    [1.0, 1.0, 0.15736404855259e-2],
    [1.0, 2.0, 0.90153761673944e-3],
    [1.0, 3.0, -0.50270077677648e-2],
    [2.0, 3.0, 0.22440037409485e-5],
    [2.0, 9.0, -0.41163275453471e-5],
    [3.0, 7.0, 0.37919454822955e-7],
];

const REGION_5_COEFFS_IDEAL: [[f64; 2]; 6] = [
    [0.0, -0.13179983674201e2],
    [1.0, 0.68540841634434e1],
    [-3.0, -0.24805148933466e-1],
    [-2.0, 0.36901534980333],
    [-1.0, -0.31161318213925e1],
    [2.0, -0.32961626538917],
];

// ================    Region 5 ===================

/// Returns the region-5 tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn tau_5(t: f64) -> f64 {
    1000.0 / t
}

/// Returns the region-5 pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn pi_5(p: f64) -> f64 {
    p / 1e6
}

/// Returns the region-5 ideal gamma
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_5_ideal(t: f64, p: f64) -> f64 {
    let pi: f64 = pi_5(p);
    let mut sum: f64 = 0.0;
    let tau: f64 = tau_5(t);
    for coefficient in REGION_5_COEFFS_IDEAL {
        let ji: i32 = coefficient[0] as i32;
        let ni: f64 = coefficient[1];
        sum += ni * tau.powi(ji);
    }
    pi.ln() + sum
}

/// Returns the region-2 residual gamma
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_5_res(t: f64, p: f64) -> f64 {
    let pi: f64 = pi_5(p);
    let mut sum: f64 = 0.0;
    let tau: f64 = tau_5(t);
    for coefficient in REGION_5_COEFFS_RES {
        let ii: i32 = coefficient[0] as i32;
        let ji: i32 = coefficient[1] as i32;
        let ni: f64 = coefficient[2];
        sum += ni * pi.powi(ii) * tau.powi(ji);
    }
    sum
}

/// Returns the region-5 ideal gamma_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_5_ideal(t: f64, _: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau: f64 = tau_5(t);
    for coefficient in REGION_5_COEFFS_IDEAL {
        let ji: f64 = coefficient[0];
        let ni: f64 = coefficient[1];
        sum += ni * ji * tau.powi(ji as i32 - 1);
    }
    sum
}

/// Returns the region-5 ideal gamma_tau_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_tau_5_ideal(t: f64, _: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau: f64 = tau_5(t);
    for coefficient in REGION_5_COEFFS_IDEAL {
        let ji: f64 = coefficient[0];
        let ni: f64 = coefficient[1];
        sum += ni * ji * (ji - 1.0) * tau.powi(ji as i32 - 2);
    }
    sum
}

/// Returns the region-5 ideal gamma_pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_5_ideal(_: f64, p: f64) -> f64 {
    1.0 / pi_5(p)
}

/// Returns the region-5 residual gamma_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_5_res(t: f64, p: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau: f64 = tau_5(t);
    let pi: f64 = pi_5(p);
    for coefficient in REGION_5_COEFFS_RES {
        let ii: i32 = coefficient[0] as i32;
        let ji: f64 = coefficient[1];
        let ni: f64 = coefficient[2];
        sum += ni * pi.powi(ii) * ji * tau.powi(ji as i32 - 1);
    }
    sum
}

/// Returns the region-5 residual gamma_tau_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_tau_5_res(t: f64, p: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau: f64 = tau_5(t);
    let pi: f64 = pi_5(p);
    for coefficient in REGION_5_COEFFS_RES {
        let ii: i32 = coefficient[0] as i32;
        let ji: f64 = coefficient[1];
        let ni: f64 = coefficient[2];
        sum += ni * pi.powi(ii) * ji * (ji - 1.0) * tau.powi(ji as i32 - 2);
    }
    sum
}

/// Returns the region-5 residual gamma_pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_5_res(t: f64, p: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau: f64 = tau_5(t);
    let pi: f64 = pi_5(p);
    for coefficient in REGION_5_COEFFS_RES {
        let ii: f64 = coefficient[0];
        let ji: i32 = coefficient[1] as i32;
        let ni: f64 = coefficient[2];
        sum += ni * ii * pi.powi(ii as i32 - 1) * tau.powi(ji);
    }
    sum
}

/// Returns the region-5 residual gamma_pi_pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_pi_5_res(t: f64, p: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau: f64 = tau_5(t);
    let pi: f64 = pi_5(p);
    for coefficient in REGION_5_COEFFS_RES {
        let ii: f64 = coefficient[0];
        let ji: i32 = coefficient[1] as i32;
        let ni: f64 = coefficient[2];
        sum += ni * ii * (ii - 1.0) * pi.powi(ii as i32 - 2) * tau.powi(ji);
    }
    sum
}

/// Returns the region-5 residual gamma_pi_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_tau_5_res(t: f64, p: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau: f64 = tau_5(t);
    let pi: f64 = pi_5(p);
    for coefficient in REGION_5_COEFFS_RES {
        let ii: f64 = coefficient[0];
        let ji: f64 = coefficient[1];
        let ni: f64 = coefficient[2];
        sum += ni * ii * pi.powi(ii as i32 - 1) * ji * tau.powi(ji as i32 - 1);
    }
    sum
}

/// Returns the region-5 specific volume
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn v_tp_5(t: f64, p: f64) -> f64 {
    ((constants::_R * 1000.0) * t / p) * pi_5(p) * (gamma_pi_5_ideal(t, p) + gamma_pi_5_res(t, p))
}

/// Returns the region-5 enthalpy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn h_tp_5(t: f64, p: f64) -> f64 {
    constants::_R * t * tau_5(t) * (gamma_tau_5_ideal(t, p) + gamma_tau_5_res(t, p))
}

/// Returns the region-5 internal energy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn u_tp_5(t: f64, p: f64) -> f64 {
    let tau: f64 = tau_5(t);
    let pi: f64 = pi_5(p);
    constants::_R
        * t
        * (tau * (gamma_tau_5_ideal(t, p) + gamma_tau_5_res(t, p))
            - pi * (gamma_pi_5_ideal(t, p) + gamma_pi_5_res(t, p)))
}

/// Returns the region-5 entropy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn s_tp_5(t: f64, p: f64) -> f64 {
    let tau = tau_5(t);
    constants::_R
        * (tau * (gamma_tau_5_ideal(t, p) + gamma_tau_5_res(t, p))
            - (gamma_5_ideal(t, p) + gamma_5_res(t, p)))
}

/// Returns the region-5 isobaric specific heat
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn cp_tp_5(t: f64, p: f64) -> f64 {
    - constants::_R * tau_5(t).powi(2) * (gamma_tau_tau_5_ideal(t, p) + gamma_tau_tau_5_res(t, p))
}

/// Returns the region-5 isochoric specific heat
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn cv_tp_5(t: f64, p: f64) -> f64 {
    let pi: f64 = pi_5(p);
    cp_tp_5(t, p)
        - constants::_R
            * (((1.0 + pi * gamma_pi_5_res(t, p) - tau_5(t) * pi * gamma_pi_tau_5_res(t, p))
                .powi(2))
                / (1.0 - pi.powi(2) * gamma_pi_pi_5_res(t, p)))
}

/// Returns the region-5 sound velocity
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn w_tp_5(t: f64, p: f64) -> f64 {
    let tau: f64 = tau_5(t);
    let pi: f64 = pi_5(p);
    let num: f64 =
        1.0 + 2.0 * pi * gamma_pi_5_res(t, p) + pi.powi(2) * gamma_pi_5_res(t, p).powi(2);
    let subnum: f64 =
        (1.0 + pi * gamma_pi_5_res(t, p) - tau * pi * gamma_pi_tau_5_res(t, p)).powi(2);
    let subden: f64 = tau.powi(2) * (gamma_tau_tau_5_ideal(t, p) + gamma_tau_tau_5_res(t, p));
    let den: f64 = 1.0 - pi.powi(2) * gamma_pi_pi_5_res(t, p) + subnum / subden;
    ((constants::_R * 1000.0 * t) * num / den).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate float_cmp;
    use float_cmp::ApproxEq;

    #[test]
    fn specific_volume() {
        let v = v_tp_5(2000.0, 30.0e6) * 10.0;
        assert!(v.approx_eq(0.3113852187, (1e-9, 2)));

        let v = v_tp_5(1273.15, 40e6) * 10.0;
        assert!(v.approx_eq(0.14360356511, (1e-9, 2)));

        let v = v_tp_5(2250.0, 1e6) / 10.0;
        assert!(v.approx_eq(0.1038886815243, (1e-9, 2)));
    }

    #[test]
    fn enthalpy() {
        let h = h_tp_5(2000.0, 30.0e6) / 10000.0;
        assert!(h.approx_eq(0.6571226038618478, (1e-9, 2)));

        let h = h_tp_5(1273.15, 40e6) / 10000.0;
        assert!(h.approx_eq(0.4527205192095566, (1e-9, 2)));

        let h = h_tp_5(2250.0, 1e6) / 10000.0;
        assert!(h.approx_eq(0.7308939035016572, (1e-9, 2)));
    }

    #[test]
    fn internal_energy() {
        let u = u_tp_5(2000.0, 30.0e6) / 10000.0;
        assert!(u.approx_eq(0.5637070382521894, (1e-9, 2)));

        let u = u_tp_5(1273.15, 40e6) / 10000.0;
        assert!(u.approx_eq(0.3952790931646828, (1e-9, 2)));

        let u = u_tp_5(2250.0, 1e6) / 10000.0;
        assert!(u.approx_eq(0.6270052219774057, (1e-9, 2)));
    }

    #[test]
    fn entropy() {
        let s = s_tp_5(2000.0, 30.0e6) / 10.0;
        assert!(s.approx_eq(0.8536405231138, (1e-9, 2)));

        let s = s_tp_5(1273.15, 40e6) / 10.0;
        assert!(s.approx_eq(0.7135299767673, (1e-9, 2)));

        let s = s_tp_5(2250.0, 1e6) / 100.0;
        assert!(s.approx_eq(0.10459345553803, (1e-9, 2)));
    }

    #[test]
    fn cp() {
        let cp = cp_tp_5(2000.0, 30.0e6) / 10.0;
        assert!(cp.approx_eq(0.2885698818781, (1e-9, 2)));

        let cp = cp_tp_5(1273.15, 40e6) / 10.0;
        assert!(cp.approx_eq(0.2737584738558, (1e-9, 2)));

        let cp = cp_tp_5(2250.0, 1e6) / 10.0;
        assert!(cp.approx_eq(0.2924605033638, (1e-9, 2)));
    }

    ///Test results based on current implementation
    #[test]
    fn cv() {
        let cv = cv_tp_5(2000.0, 30.0e6) / 10.0;
        assert!(cv.approx_eq(0.2395894362358, (1e-9, 2)));

        let cv = cv_tp_5(1273.15, 40e6) / 10.0;
        assert!(cv.approx_eq(0.2103269596467, (1e-9, 2)));

        let cv = cv_tp_5(2250.0, 1e6) / 10.0;
        assert!(cv.approx_eq(0.2462428953654, (1e-9, 2)));
    }

    #[test]
    fn sound_velocity() {
        let w = w_tp_5(2000.0, 30.0e6) / 10000.0;
        assert!(w.approx_eq(0.1067369478777425, (1e-9, 2)));

        let w = w_tp_5(1273.15, 40e6) / 1000.0;
        assert!(w.approx_eq(0.856918115142271, (1e-9, 2)));

        let w = w_tp_5(2250.0, 1e6) / 10000.0;
        assert!(w.approx_eq(0.111104265108342, (1e-9, 2)));
    }
}
