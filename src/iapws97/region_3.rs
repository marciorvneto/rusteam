use crate::iapws97::constants;
// Region 3

const REGION_3_COEFFS: [[f64; 3]; 40] = [
    [0.0, 0.0, 0.10658070028513e1],
    [0.0, 0.0, -0.15732845290239e2],
    [0.0, 1.0, 0.20944396974307e2],
    [0.0, 2.0, -0.76867707878716e1],
    [0.0, 7.0, 0.26185947787954e1],
    [0.0, 10.0, -0.28080781148620e1],
    [0.0, 12.0, 0.12053369696517e1],
    [0.0, 23.0, -0.84566812812502e-2],
    [1.0, 2.0, -0.12654315477714e1],
    [1.0, 6.0, -0.11524407806681e1],
    [1.0, 15.0, 0.88521043984318],
    [1.0, 17.0, -0.64207765181607],
    [2.0, 0.0, 0.38493460186671],
    [2.0, 2.0, -0.85214708824206],
    [2.0, 6.0, 0.48972281541877e1],
    [2.0, 7.0, -0.30502617256965e1],
    [2.0, 22.0, 0.39420536879154e-1],
    [2.0, 26.0, 0.12558408424308],
    [3.0, 0.0, -0.27999329698710],
    [3.0, 2.0, 0.13899799569460e1],
    [3.0, 4.0, -0.20189915023570e1],
    [3.0, 16.0, -0.82147637173963e-2],
    [3.0, 26.0, -0.47596035734923],
    [4.0, 0.0, 0.43984074473500e-1],
    [4.0, 2.0, -0.44476435428739],
    [4.0, 4.0, 0.90572070719733],
    [4.0, 26.0, 0.70522450087967],
    [5.0, 1.0, 0.10770512626332],
    [5.0, 3.0, -0.32913623258954],
    [5.0, 26.0, -0.50871062041158],
    [6.0, 0.0, -0.22175400873096e-1],
    [6.0, 2.0, 0.94260751665092e-1],
    [6.0, 26.0, 0.16436278447961],
    [7.0, 2.0, -0.13503372241348e-1],
    [8.0, 26.0, -0.14834345352472e-1],
    [9.0, 2.0, 0.57922953628084e-3],
    [9.0, 26.0, 0.32308904703711e-2],
    [10.0, 0.0, 0.80964802996215e-4],
    [10.0, 1.0, -0.16557679795037e-3],
    [11.0, 26.0, -0.44923899061815e-4],
];

// ================    Region 3 ===================

/// Returns the region-3 delta
/// Specific mass is assumed to be in kg/m3
fn delta_3(rho: f64) -> f64 {
    rho / constants::RHO_C
}

/// Returns the region-3 tau
/// Temperature is assumed to be in K
fn tau_3(t: f64) -> f64 {
    constants::T_C / t
}

fn phi_3(rho: f64, t: f64) -> f64 {
    let mut sum: f64 = REGION_3_COEFFS[0][2] * delta_3(rho).ln();
    let tau = tau_3(t);
    let delta = delta_3(rho);

    for coefficient in REGION_3_COEFFS.iter().skip(1) {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * delta.powi(ii) * tau.powi(ji);
    }

    sum
}

fn phi_delta_3(rho: f64, t: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau = tau_3(t);
    let delta = delta_3(rho);
    for coefficient in REGION_3_COEFFS.iter().skip(1) {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * delta.powi(ii - 1) * f64::from(ii) * tau.powi(ji);
    }
    sum + REGION_3_COEFFS[0][2] / delta
}

fn phi_delta_delta_3(rho: f64, t: f64) -> f64 {
    let mut sum: f64 = -REGION_3_COEFFS[0][2] / delta_3(rho).powi(2);
    let tau = tau_3(t);
    let delta = delta_3(rho);
    for coefficient in REGION_3_COEFFS.iter().skip(1) {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * f64::from(ii * (ii - 1)) * delta.powi(ii - 2) * tau.powi(ji);
    }
    sum
}

fn phi_delta_tau_3(rho: f64, t: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau = tau_3(t);
    let delta = delta_3(rho);
    for coefficient in REGION_3_COEFFS.iter().skip(1) {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * f64::from(ii) * delta.powi(ii - 1) * f64::from(ji) * tau.powi(ji - 1);
    }
    sum
}

fn phi_tau_3(rho: f64, t: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau = tau_3(t);
    let delta = delta_3(rho);
    for coefficient in REGION_3_COEFFS.iter().skip(1) {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * delta.powi(ii) * f64::from(ji) * tau.powi(ji - 1);
    }
    sum
}

fn phi_tau_tau_3(rho: f64, t: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau = tau_3(t);
    let delta = delta_3(rho);
    for coefficient in REGION_3_COEFFS.iter().skip(1) {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * delta.powi(ii) * f64::from(ji * (ji - 1)) * tau.powi(ji - 2);
    }
    sum
}

#[allow(dead_code)]
pub(crate) fn p_rho_t_3(rho: f64, t: f64) -> f64 {
    rho * (constants::_R * 1000.0) * t * delta_3(rho) * phi_delta_3(rho, t)
}

#[allow(dead_code)]
pub(crate) fn u_rho_t_3(rho: f64, t: f64) -> f64 {
    (constants::_R) * t * tau_3(t) * phi_tau_3(rho, t)
}

#[allow(dead_code)]
pub(crate) fn s_rho_t_3(rho: f64, t: f64) -> f64 {
    (constants::_R) * (tau_3(t) * phi_tau_3(rho, t) - phi_3(rho, t))
}

#[allow(dead_code)]
pub(crate) fn h_rho_t_3(rho: f64, t: f64) -> f64 {
    (constants::_R) * t * (tau_3(t) * phi_tau_3(rho, t) + delta_3(rho) * phi_delta_3(rho, t))
}

#[allow(dead_code)]
pub(crate) fn cv_rho_t_3(rho: f64, t: f64) -> f64 {
    -tau_3(t).powi(2) * phi_tau_tau_3(rho, t) * (constants::_R)
}

#[allow(dead_code)]
pub(crate) fn cp_rho_t_3(rho: f64, t: f64) -> f64 {
    (-tau_3(t).powi(2) * phi_tau_tau_3(rho, t)
        + ((delta_3(rho) * phi_delta_3(rho, t)
            - delta_3(rho) * tau_3(t) * phi_delta_tau_3(rho, t))
        .powi(2)
            / (2.0 * delta_3(rho) * phi_delta_3(rho, t)
                + delta_3(rho).powi(2) * phi_delta_delta_3(rho, t))))
        * (constants::_R)
}

#[allow(dead_code)]
pub(crate) fn w_rho_t_3(rho: f64, t: f64) -> f64 {
    let w = ((2.0 * delta_3(rho) * phi_delta_3(rho, t)
        + delta_3(rho).powi(2) * phi_delta_delta_3(rho, t)
        - ((delta_3(rho) * phi_delta_3(rho, t)
            - delta_3(rho) * tau_3(t) * phi_delta_tau_3(rho, t))
        .powi(2)
            / (tau_3(t).powi(2) * phi_tau_tau_3(rho, t))))
        * t
        * (constants::_R))
        * 1000.0; // Took the times 1000 from https://github.com/CoolProp/IF97 else the test reuslts would not match
    w.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate float_cmp;
    use float_cmp::ApproxEq;

    #[test]
    fn region_3_p() {
        let p = p_rho_t_3(500.0, 650.0) / 1e8;
        assert!(p.approx_eq(0.255837018, (1e-9, 2)));

        let p = p_rho_t_3(200.0, 650.0) / 1e8;
        assert!(p.approx_eq(0.222930643, (1e-9, 2)));

        let p = p_rho_t_3(500.0, 750.0) / 1e8;
        assert!(p.approx_eq(0.783095639, (1e-9, 2)));
    }

    #[test]
    fn internal_energy() {
        let u = u_rho_t_3(500.0, 650.0) / 1e4;
        assert!(u.approx_eq(0.181226279, (1e-9, 2)));

        let u = u_rho_t_3(200.0, 650.0) / 1e4;
        assert!(u.approx_eq(0.226365868, (1e-9, 2)));

        let u = u_rho_t_3(500.0, 750.0) / 1e4;
        assert!(u.approx_eq(0.210206932, (1e-9, 2)));
    }

    #[test]
    fn specific_entropy() {
        let s = s_rho_t_3(500.0, 650.0) / 1e1;
        assert!(s.approx_eq(0.405427273, (1e-9, 2)));

        let s = s_rho_t_3(200.0, 650.0) / 1e1;
        assert!(s.approx_eq(0.485438792, (1e-9, 2)));

        let s = s_rho_t_3(500.0, 750.0) / 1e1;
        assert!(s.approx_eq(0.446971906, (1e-9, 2)));
    }

    #[test]
    fn specific_enthalpy() {
        let h = h_rho_t_3(500.0, 650.0) / 1e4;
        assert!(h.approx_eq(0.186343019, (1e-9, 2)));

        let h = h_rho_t_3(200.0, 650.0) / 1e4;
        assert!(h.approx_eq(0.237512401, (1e-9, 2)));

        let h = h_rho_t_3(500.0, 750.0) / 1e4;
        assert!(h.approx_eq(0.225868845, (1e-9, 2)));
    }
    ///
    ///Test results based on current implementation
    ///Verify against https://github.com/CoolProp/IF97
    #[test]
    fn cv() {
        let cv = cv_rho_t_3(500.0, 650.0) / 10.0;
        assert!(cv.approx_eq(0.3191317871889249, (1e-9, 2)));

        let cv = cv_rho_t_3(200.0, 650.0) / 10.0;
        assert!(cv.approx_eq(0.4041180759550155, (1e-9, 2)));

        let cv = cv_rho_t_3(500.0, 750.0) / 10.0;
        assert!(cv.approx_eq(0.2717016771210098, (1e-9, 2)));
    }

    #[test]
    fn cp() {
        let cp = cp_rho_t_3(500.0, 650.0) / 1e2;
        assert!(cp.approx_eq(0.138935717, (1e-9, 2)));

        let cp = cp_rho_t_3(200.0, 650.0) / 1e2;
        assert!(cp.approx_eq(0.446579342, (1e-9, 2)));

        let cp = cp_rho_t_3(500.0, 750.0) / 1e1;
        assert!(cp.approx_eq(0.634165359, (1e-9, 2)));
    }

    #[test]
    fn sound_velocity() {
        let w = w_rho_t_3(500.0, 650.0) / 1e3;
        assert!(w.approx_eq(0.502005554, (1e-9, 2)));

        let w = w_rho_t_3(200.0, 650.0) / 1e3;
        assert!(w.approx_eq(0.383444594, (1e-9, 2)));

        let w = w_rho_t_3(500.0, 750.0) / 1e3;
        assert!(w.approx_eq(0.760696041, (1e-9, 2)));
    }
}
