// Region 2
use crate::iapws97::constants;

const REGION_2_COEFFS_RES: [[f64; 3]; 43] = [
    [1.0, 0.0, -0.0017731742473213],
    [1.0, 1.0, -0.017834862292358],
    [1.0, 2.0, -0.045996013696365],
    [1.0, 3.0, -0.057581259083432],
    [1.0, 6.0, -0.05032527872793],
    [2.0, 1.0, -0.000033032641670203],
    [2.0, 2.0, -0.00018948987516315],
    [2.0, 4.0, -0.0039392777243355],
    [2.0, 7.0, -0.043797295650573],
    [2.0, 36.0, -0.000026674547914087],
    [3.0, 0.0, 2.0481737692309E-08],
    [3.0, 1.0, 4.3870667284435E-07],
    [3.0, 3.0, -0.00003227767723857],
    [3.0, 6.0, -0.0015033924542148],
    [3.0, 35.0, -0.040668253562649],
    [4.0, 1.0, -7.8847309559367E-10],
    [4.0, 2.0, 1.2790717852285E-08],
    [4.0, 3.0, 4.8225372718507E-07],
    [5.0, 7.0, 2.2922076337661E-06],
    [6.0, 3.0, -1.6714766451061E-11],
    [6.0, 16.0, -0.0021171472321355],
    [6.0, 35.0, -23.895741934104],
    [7.0, 0.0, -5.905956432427E-18],
    [7.0, 11.0, -1.2621808899101E-06],
    [7.0, 25.0, -0.038946842435739],
    [8.0, 8.0, 1.1256211360459E-11],
    [8.0, 36.0, -8.2311340897998],
    [9.0, 13.0, 1.9809712802088E-08],
    [10.0, 4.0, 1.0406965210174E-19],
    [10.0, 10.0, -1.0234747095929E-13],
    [10.0, 14.0, -1.0018179379511E-09],
    [16.0, 29.0, -8.0882908646985E-11],
    [16.0, 50.0, 0.10693031879409],
    [18.0, 57.0, -0.33662250574171],
    [20.0, 20.0, 8.9185845355421E-25],
    [20.0, 35.0, 3.0629316876232E-13],
    [20.0, 48.0, -4.2002467698208E-06],
    [21.0, 21.0, -5.9056029685639E-26],
    [22.0, 53.0, 3.7826947613457E-06],
    [23.0, 39.0, -1.2768608934681E-15],
    [24.0, 26.0, 7.3087610595061E-29],
    [24.0, 40.0, 5.5414715350778E-17],
    [24.0, 58.0, -9.436970724121E-07],
];

const REGION_2_COEFFS_IDEAL: [[f64; 2]; 9] = [
    [0.0, -0.96927686500217e1],
    [1.0, 0.10086655968018e2],
    [-5.0, -0.56087911283020e-2],
    [-4.0, 0.71452738081455e-1],
    [-3.0, -0.40710498223928],
    [-2.0, 0.14240819171444e1],
    [-1.0, -0.43839511319450e1],
    [2.0, -0.28408632460772],
    [3.0, 0.21268463753307e-1],
];

// ================    Region 2 ===================

/// Returns the region-2 tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn tau_2(t: f64) -> f64 {
    540.0 / t
}

/// Returns the region-2 pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn pi_2(p: f64) -> f64 {
    p / 1e6
}

/// Returns the region-2 ideal gamma
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_2_ideal(t: f64, p: f64) -> f64 {
    let pi = pi_2(p);
    let mut sum: f64 = 0.0;
    let tau = tau_2(t);
    for coefficient in REGION_2_COEFFS_IDEAL {
        let ji = coefficient[0];
        let ni = coefficient[1];
        sum += ni * tau.powf(ji);
    }
    pi.ln() + sum
}

/// Returns the region-2 residual gamma
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_2_res(t: f64, p: f64) -> f64 {
    let pi = pi_2(p);
    let mut sum: f64 = 0.0;
    let tau = tau_2(t);
    for coefficient in REGION_2_COEFFS_RES {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * pi.powi(ii) * (tau - 0.5).powi(ji);
    }
    sum
}

/// Returns the region-2 ideal gamma_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_2_ideal(t: f64, _: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau = tau_2(t);
    for coefficient in REGION_2_COEFFS_IDEAL {
        let ji = coefficient[0];
        let ni = coefficient[1];
        sum += ni * ji * tau.powf(ji - 1.0);
    }
    sum
}

/// Returns the region-2 ideal gamma_tau_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_tau_2_ideal(t: f64, _: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau = tau_2(t);
    for coefficient in REGION_2_COEFFS_IDEAL {
        let ji = coefficient[0];
        let ni = coefficient[1];
        sum += ni * ji * (ji - 1.0) * tau.powf(ji - 2.0);
    }
    sum
}

/// Returns the region-2 ideal gamma_pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_2_ideal(_: f64, p: f64) -> f64 {
    1.0 / pi_2(p)
}

/// Returns the region-2 residual gamma_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_2_res(t: f64, p: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau = tau_2(t);
    let pi = pi_2(p);
    for coefficient in REGION_2_COEFFS_RES {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * pi.powi(ii) * f64::from(ji) * (tau - 0.5).powi(ji - 1);
    }
    sum
}

/// Returns the region-2 residual gamma_tau_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_tau_2_res(t: f64, p: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau = tau_2(t);
    let pi = pi_2(p);
    for coefficient in REGION_2_COEFFS_RES {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * pi.powi(ii) * f64::from(ji * (ji - 1)) * (tau - 0.5).powi(ji - 2);
    }
    sum
}

/// Returns the region-2 residual gamma_pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_2_res(t: f64, p: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau = tau_2(t);
    let pi = pi_2(p);
    for coefficient in REGION_2_COEFFS_RES {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * pi.powi(ii - 1) * f64::from(ii) * (tau - 0.5).powi(ji);
    }
    sum
}

/// Returns the region-2 residual gamma_pi_pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_pi_2_res(t: f64, p: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau = tau_2(t);
    let pi = pi_2(p);
    for coefficient in REGION_2_COEFFS_RES {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * pi.powi(ii - 2) * f64::from(ii * (ii - 1)) * (tau - 0.5).powi(ji);
    }
    sum
}

/// Returns the region-2 residual gamma_pi_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_tau_2_res(t: f64, p: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau = tau_2(t);
    let pi = pi_2(p);
    for coefficient in REGION_2_COEFFS_RES {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * pi.powi(ii - 1) * f64::from(ii * ji) * (tau - 0.5).powi(ji - 1);
    }
    sum
}

/// Returns the region-2 specific volume
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn v_tp_2(t: f64, p: f64) -> f64 {
    ((constants::_R * 1000.0) * t / p) * pi_2(p) * (gamma_pi_2_ideal(t, p) + gamma_pi_2_res(t, p))
}

/// Returns the region-2 enthalpy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn h_tp_2(t: f64, p: f64) -> f64 {
    constants::_R * t * tau_2(t) * (gamma_tau_2_ideal(t, p) + gamma_tau_2_res(t, p))
}

/// Returns the region-2 internal energy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn u_tp_2(t: f64, p: f64) -> f64 {
    let tau = tau_2(t);
    let pi = pi_2(p);
    let tau_term = gamma_tau_2_ideal(t, p) + gamma_tau_2_res(t, p);
    let pi_term = gamma_pi_2_ideal(t, p) + gamma_pi_2_res(t, p);
    constants::_R * t * (tau * tau_term - pi * pi_term)
}

/// Returns the region-2 entropy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn s_tp_2(t: f64, p: f64) -> f64 {
    let tau = tau_2(t);
    let tau_term = gamma_tau_2_ideal(t, p) + gamma_tau_2_res(t, p);
    let pi_term = gamma_2_ideal(t, p) + gamma_2_res(t, p);
    constants::_R * (tau * tau_term - pi_term)
}

/// Returns the region-2 isobaric specific heat
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn cp_tp_2(t: f64, p: f64) -> f64 {
    let tau = tau_2(t);
    -constants::_R * tau.powi(2) * (gamma_tau_tau_2_ideal(t, p) + gamma_tau_tau_2_res(t, p))
}

/// Returns the region-2 isochoric specific heat
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn cv_tp_2(t: f64, p: f64) -> f64 {
    let tau = tau_2(t);
    let pi = pi_2(p);
    let num = (1.0 + pi * gamma_pi_2_res(t, p) - tau * pi * gamma_pi_tau_2_res(t, p)).powi(2);
    let den = 1.0 - pi.powi(2) * gamma_pi_pi_2_res(t, p);
    cp_tp_2(t, p) - constants::_R * num / den
}

/// Returns the region-2 sound velocity
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn w_tp_2(t: f64, p: f64) -> f64 {
    let tau = tau_2(t);
    let pi = pi_2(p);
    let num = 1.0 + 2.0 * pi * gamma_pi_2_res(t, p) + pi.powi(2) * gamma_pi_2_res(t, p).powi(2);
    let subnum = (1.0 + pi * gamma_pi_2_res(t, p) - tau * pi * gamma_pi_tau_2_res(t, p)).powi(2);
    let subden = tau.powi(2) * (gamma_tau_tau_2_ideal(t, p) + gamma_tau_tau_2_res(t, p));
    let den = 1.0 - pi.powi(2) * gamma_pi_pi_2_res(t, p) + subnum / subden;
    ((constants::_R * 1000.0 * t) * num / den).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate float_cmp;
    use float_cmp::ApproxEq;

    #[test]
    fn specific_volume() {
        let v = v_tp_2(300.0, 0.0035e6) / 100.0;
        assert!(v.approx_eq(0.394913866, (1e-9, 2)));

        let v = v_tp_2(700.0, 0.0035e6) / 100.0;
        assert!(v.approx_eq(0.923015898, (1e-9, 2)));

        let v = v_tp_2(700.0, 30e6) / 0.01;
        assert!(v.approx_eq(0.542946619, (1e-9, 2)));
    }

    #[test]
    fn enthalpy() {
        let h = h_tp_2(300.0, 0.0035e6) / 10000.0;
        assert!(h.approx_eq(0.254991145, (1e-9, 2)));

        let h = h_tp_2(700.0, 0.0035e6) / 10000.0;
        assert!(h.approx_eq(0.333568375, (1e-9, 2)));

        let h = h_tp_2(700.0, 30e6) / 10000.0;
        assert!(h.approx_eq(0.263149474, (1e-9, 2)));
    }

    #[test]
    fn internal_energy() {
        let u = u_tp_2(300.0, 0.0035e6) / 10000.0;
        assert!(u.approx_eq(0.241169160, (1e-9, 2)));

        let u = u_tp_2(700.0, 0.0035e6) / 10000.0;
        assert!(u.approx_eq(0.301262819, (1e-9, 2)));

        let u = u_tp_2(700.0, 30e6) / 10000.0;
        assert!(u.approx_eq(0.246861076, (1e-9, 2)));
    }

    #[test]
    fn entropy() {
        let s = s_tp_2(300.0, 0.0035e6) / 10.0;
        assert!(s.approx_eq(0.852238967, (1e-9, 2)));

        let s = s_tp_2(700.0, 0.0035e6) / 100.0;
        assert!(s.approx_eq(0.101749996, (1e-9, 2)));

        let s = s_tp_2(700.0, 30e6) / 10.0;
        assert!(s.approx_eq(0.517540298, (1e-9, 2)));
    }

    #[test]
    fn cp() {
        let cp = cp_tp_2(300.0, 0.0035e6) / 10.0;
        assert!(cp.approx_eq(0.191300162, (1e-9, 2)));

        let cp = cp_tp_2(700.0, 0.0035e6) / 10.0;
        assert!(cp.approx_eq(0.208141274, (1e-9, 2)));

        let cp = cp_tp_2(700.0, 30e6) / 100.0;
        assert!(cp.approx_eq(0.103505092, (1e-9, 2)));
    }

    ///Test results based on current implementation
    ///Verify against https://github.com/CoolProp/IF97
    #[test]
    fn cv() {
        let cv = cv_tp_2(300.0, 0.0035e6) / 10.0;
        assert!(cv.approx_eq(0.1441326618, (1e-9, 2)));

        let cv = cv_tp_2(700.0, 0.0035e6) / 10.0;
        assert!(cv.approx_eq(0.1619783325, (1e-9, 2)));

        let cv = cv_tp_2(700.0, 30e6) / 100.0;
        assert!(cv.approx_eq(0.0297553836, (1e-9, 2)));
    }

    #[test]
    fn sound_velocity() {
        let w = w_tp_2(300.0, 0.0035e6) / 1000.0;
        assert!(w.approx_eq(0.427920172, (1e-9, 2)));

        let w = w_tp_2(700.0, 0.0035e6) / 1000.0;
        assert!(w.approx_eq(0.644289068, (1e-9, 2)));

        let w = w_tp_2(700.0, 30e6) / 1000.0;
        assert!(w.approx_eq(0.480386523, (1e-9, 2)));
    }
}
