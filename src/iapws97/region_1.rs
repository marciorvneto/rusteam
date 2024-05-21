use crate::iapws97::constants;

const REGION_1_COEFFS: [[f64; 3]; 34] = [
    [0.0, -2.0, 0.14632971213167],
    [0.0, -1.0, -0.84548187169114],
    [0.0, 0.0, -0.37563603672040e1],
    [0.0, 1.0, 0.33855169168385e1],
    [0.0, 2.0, -0.95791963387872],
    [0.0, 3.0, 0.15772038513228],
    [0.0, 4.0, -0.16616417199501e-1],
    [0.0, 5.0, 0.81214629983568e-3],
    [1.0, -9.0, 0.28319080123804e-3],
    [1.0, -7.0, -0.60706301565874e-3],
    [1.0, -1.0, -0.18990068218419e-1],
    [1.0, 0.0, -0.32529748770505e-1],
    [1.0, 1.0, -0.21841717175414e-1],
    [1.0, 3.0, -0.52838357969930e-4],
    [2.0, -3.0, -0.47184321073267e-3],
    [2.0, 0.0, -0.30001780793026e-3],
    [2.0, 1.0, 0.47661393906987e-4],
    [2.0, 3.0, -0.44141845330846e-5],
    [2.0, 17.0, -0.72694996297594e-15],
    [3.0, -4.0, -0.31679644845054e-4],
    [3.0, 0.0, -0.28270797985312e-5],
    [3.0, 6.0, -0.85205128120103e-9],
    [4.0, -5.0, -0.22425281908000e-5],
    [4.0, -2.0, -0.65171222895601e-6],
    [4.0, 10.0, -0.14341729937924e-12],
    [5.0, -8.0, -0.40516996860117e-6],
    [8.0, -11.0, -0.12734301741641e-8],
    [8.0, -6.0, -0.17424871230634e-9],
    [21.0, -29.0, -0.68762131295531e-18],
    [23.0, -31.0, 0.14478307828521e-19],
    [29.0, -38.0, 0.26335781662795e-22],
    [30.0, -39.0, -0.11947622640071e-22],
    [31.0, -40.0, 0.18228094581404e-23],
    [32.0, -41.0, -0.93537087292458e-25],
];

const REGION_1_BACK_COEFFS_PH: [[f64; 3]; 20] = [
    [0.0, 0.0, -0.23872489924521e+3],
    [0.0, 1.0, 0.40421188637945e+3],
    [0.0, 2.0, 0.11349746881718e+3],
    [0.0, 6.0, -0.58457616048039e+1],
    [0.0, 22.0, -0.15285482413140e-3],
    [0.0, 32.0, -0.10866707695377e-5],
    [1.0, 0.0, -0.13391744872602e+2],
    [1.0, 1.0, 0.43211039183559e+2],
    [1.0, 2.0, -0.54010067170506e+2],
    [1.0, 3.0, 0.30535892203916e+2],
    [1.0, 4.0, -0.65964749423638e+1],
    [1.0, 10.0, 0.93965400878363e-2],
    [1.0, 32.0, 0.11573647505340e-6],
    [2.0, 10.0, -0.25858641282073e-4],
    [2.0, 32.0, -0.40644363084799e-8],
    [3.0, 10.0, 0.66456186191635e-7],
    [3.0, 32.0, 0.80670734103027e-10],
    [4.0, 32.0, -0.93477771213947e-12],
    [5.0, 32.0, 0.58265442020601e-14],
    [6.0, 32.0, -0.15020185953503e-16],
];
const REGION_1_BACK_COEFFS_PS: [[f64; 3]; 20] = [
    [0.0, 0.0, 0.17478268058307e+03],
    [0.0, 1.0, 0.34806930892873e+02],
    [0.0, 2.0, 0.65292584978455e+01],
    [0.0, 3.0, 0.33039981775489],
    [0.0, 11.0, -0.19281382923196e-06],
    [0.0, 31.0, -0.24909197244573e-22],
    [1.0, 0.0, -0.26107636489332],
    [1.0, 1.0, 0.22592965981586],
    [1.0, 2.0, -0.64256463395226e-01],
    [1.0, 3.0, 0.78876289270526e-02],
    [1.0, 12.0, 0.35672110607366e-09],
    [1.0, 31.0, 0.17332496994895e-23],
    [2.0, 0.0, 0.56608900654837e-03],
    [2.0, 1.0, -0.32635483139717e-03],
    [2.0, 2.0, 0.44778286690632e-04],
    [2.0, 9.0, -0.51322156908507e-09],
    [2.0, 31.0, -0.42522657042207e-25],
    [3.0, 10.0, 0.26400441360689e-12],
    [3.0, 32.0, 0.78124600459723e-28],
    [4.0, 32.0, -0.30732199903668e-30],
];

// ================    Region 1 ===================

/// Returns the region-1 gamma
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let pi = pi_1(p);
    let mut sum = 0.0;
    for coefficient in REGION_1_COEFFS {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * (7.1 - pi).powi(ii) * (tau - 1.222).powi(ji);
    }
    sum
}

/// Returns the region-1 gamma_pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let pi = pi_1(p);
    let mut sum = 0.0;
    for coefficient in REGION_1_COEFFS {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += -ni * f64::from(ii) * (7.1 - pi).powi(ii - 1) * (tau - 1.222).powi(ji);
    }
    sum
}

/// Returns the region-1 gamma_pi_pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_pi_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let pi = pi_1(p);
    let mut sum = 0.0;
    for coefficient in REGION_1_COEFFS {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * f64::from(ii * (ii - 1)) * (7.1 - pi).powi(ii - 2) * (tau - 1.222).powi(ji);
    }
    sum
}

/// Returns the region-1 gamma_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let pi = pi_1(p);
    let mut sum = 0.0;
    for coefficient in REGION_1_COEFFS {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * f64::from(ji) * (7.1 - pi).powi(ii) * (tau - 1.222).powi(ji - 1);
    }
    sum
}

/// Returns the region-1 gamma_tau_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_tau_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let pi = pi_1(p);
    let mut sum = 0.0;
    for coefficient in REGION_1_COEFFS {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * f64::from(ji * (ji - 1)) * (7.1 - pi).powi(ii) * (tau - 1.222).powi(ji - 2);
    }
    sum
}

/// Returns the region-1 gamma_pi_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_tau_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let pi = pi_1(p);
    let mut sum = 0.0;
    for coefficient in REGION_1_COEFFS {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += -ni * f64::from(ii * ji) * (7.1 - pi).powi(ii - 1) * (tau - 1.222).powi(ji - 1);
    }
    sum
}

/// Returns the region-1 tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn tau_1(t: f64) -> f64 {
    1386.0 / t
}

/// Returns the region-1 pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn pi_1(p: f64) -> f64 {
    p / (16.53e6)
}

/// Returns the region-1 specific enthalpy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn h_tp_1(t: f64, p: f64) -> f64 {
    constants::_R * t * tau_1(t) * gamma_tau_1(t, p)
}

/// Returns the region-1 specific volume
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn v_tp_1(t: f64, p: f64) -> f64 {
    // The multiplication by 1000 is necessary to convert R from kJ/kg.K to J/kg.K
    ((constants::_R * 1000.0) * t / p) * pi_1(p) * gamma_pi_1(t, p)
}

/// Returns the region-1 specific internal energy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn u_tp_1(t: f64, p: f64) -> f64 {
    constants::_R * t * (tau_1(t) * gamma_tau_1(t, p) - pi_1(p) * gamma_pi_1(t, p))
}

/// Returns the region-1 specific internal energy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn s_tp_1(t: f64, p: f64) -> f64 {
    constants::_R * (tau_1(t) * gamma_tau_1(t, p) - gamma_1(t, p))
}

/// Returns the region-1 specific isobaric heat capacity
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn cp_tp_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    constants::_R * (-tau.powi(2) * gamma_tau_tau_1(t, p))
}

/// Returns the region-1 specific isochoric heat capacity
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn cv_tp_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let corr = (gamma_pi_1(t, p) - tau * gamma_pi_tau_1(t, p)).powi(2) / gamma_pi_pi_1(t, p);
    constants::_R * (-tau.powi(2) * gamma_tau_tau_1(t, p) + corr)
}

/// Returns the region-1 speed of sound
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub(crate) fn w_tp_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let gamma_pi = gamma_pi_1(t, p);
    let gamma_pi_tau = gamma_pi_tau_1(t, p);
    let gamma_pi_pi = gamma_pi_pi_1(t, p);
    let gamma_tau_tau = gamma_tau_tau_1(t, p);
    let term = (gamma_pi - tau * gamma_pi_tau).powi(2) / (tau.powi(2) * gamma_tau_tau);

    // The multiplication by 1000 is necessary to convert R from kJ/kg.K to J/kg.K
    let square = (constants::_R * 1000.0) * t * (gamma_pi.powi(2) / (term - gamma_pi_pi));
    square.sqrt()
}

/// Returns the region-1 eta for backwards calculations
/// Enthalpy is assumed to be in kJ/kg
fn eta_1_back(h: f64) -> f64 {
    h / 2500.0
}

/// Returns the region-1 pi for backwards calculations
/// Pressure is assumed to be in Pa
fn pi_1_back(p: f64) -> f64 {
    p / 1e6
}

/// Returns the region-1 sigma for backwards calculations
/// Entropy is assumed to be in kJ/kg.K
fn sigma_1_back(s: f64) -> f64 {
    s
}

/// Returns the region-1 backward correlation for T(p,s)
/// Entropy is assumed to be in kJ/kg.K
/// Pressure is assumed to be in Pa
#[allow(dead_code)]
fn t_ps_1(p: f64, s: f64) -> f64 {
    let sig = sigma_1_back(s);
    let pi = pi_1_back(p);
    let mut sum = 0.0;
    for coefficient in REGION_1_BACK_COEFFS_PS {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * pi.powi(ii) * (sig + 2.0).powi(ji);
    }
    sum
}

/// Returns the region-1 backward correlation for T(p,h)
/// Enthalpy is assumed to be in kJ/kg
/// Pressure is assumed to be in Pa
#[allow(dead_code)]
fn t_ph_1(p: f64, h: f64) -> f64 {
    let eta = eta_1_back(h);
    let pi = pi_1_back(p);
    let mut sum = 0.0;
    for coefficient in REGION_1_BACK_COEFFS_PH {
        let ii = coefficient[0] as i32;
        let ji = coefficient[1] as i32;
        let ni = coefficient[2];
        sum += ni * pi.powi(ii) * (eta + 1.0).powi(ji);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate float_cmp;
    use float_cmp::ApproxEq;

    #[test]
    fn specific_volume() {
        let v = v_tp_1(300.0, 3e6) * 1e2;
        assert!(v.approx_eq(0.100215168, (1e-9, 2)));

        let v = v_tp_1(300.0, 80e6) * 1e3;
        assert!(v.approx_eq(0.971180894, (1e-9, 2)));

        let v = v_tp_1(500.0, 3e6) * 1e2;
        assert!(v.approx_eq(0.120241800, (1e-9, 2)));
    }

    #[test]
    fn enthalpy() {
        let h = h_tp_1(300.0, 3e6) / 1e3;

        assert!(h.approx_eq(0.115331273, (1e-9, 2)));

        let h = h_tp_1(300.0, 80e6) / 1e3;
        assert!(h.approx_eq(0.184142828, (1e-9, 2)));

        let h = h_tp_1(500.0, 3e6) / 1e3;
        assert!(h.approx_eq(0.975542239, (1e-9, 2)));
    }

    #[test]
    fn internal_energy() {
        let u = u_tp_1(300.0, 3e6) / 1e3;
        assert!(u.approx_eq(0.112324818, (1e-9, 2)));

        let u = u_tp_1(300.0, 80e6) / 1e3;
        assert!(u.approx_eq(0.106448356, (1e-9, 2)));

        let u = u_tp_1(500.0, 3e6) / 1e3;
        assert!(u.approx_eq(0.971934985, (1e-9, 2)));
    }

    #[test]
    fn entropy() {
        let s = s_tp_1(300.0, 3e6);
        assert!(s.approx_eq(0.392294792, (1e-9, 2)));

        let s = s_tp_1(300.0, 80e6);
        assert!(s.approx_eq(0.368563852, (1e-9, 2)));

        let s = s_tp_1(500.0, 3e6);
        assert!(s.approx_eq(0.258041912e1, (1e-9, 2)));
    }

    #[test]
    fn cp() {
        let cp = cp_tp_1(300.0, 3e6) / 1e1;
        assert!(cp.approx_eq(0.417301218, (1e-9, 2)));

        let cp = cp_tp_1(300.0, 80e6) / 1e1;
        print!("{}", cp);
        assert!(cp.approx_eq(0.401008987, (1e-9, 2)));

        let cp = cp_tp_1(500.0, 3e6) / 1e1;
        assert!(cp.approx_eq(0.465580682, (1e-9, 2)));
    }

    #[test]
    fn sound_velocity() {
        let w = w_tp_1(300.0, 3e6) / 1e4;
        assert!(w.approx_eq(0.150773921, (1e-9, 2)));

        let w = w_tp_1(300.0, 80e6) / 1e4;
        assert!(w.approx_eq(0.163469054, (1e-9, 2)));

        let w = w_tp_1(500.0, 3e6) / 1e4;
        assert!(w.approx_eq(0.124071337, (1e-9, 2)));
    }

    // Extra test based on calculations from IAPWS Online Calculator
    #[test]
    fn cv() {
        let cv = cv_tp_1(300.0, 3e6);
        assert!(cv.approx_eq(4.121201603, (1e-9, 2)));

        let cv = cv_tp_1(300.0, 80e6);
        assert!(cv.approx_eq(3.917366061, (1e-9, 2)));

        let cv = cv_tp_1(500.0, 3e6);
        assert!(cv.approx_eq(3.221392229, (1e-9, 2)));
    }

    #[test]
    fn backwards_t_ph() {
        let t = t_ph_1(3e6, 500.0) / 1e3;
        assert!(t.approx_eq(0.391798509, (1e-9, 2)));

        let t = t_ph_1(80e6, 500.0) / 1e3;
        assert!(t.approx_eq(0.378108626, (1e-9, 2)));

        let t = t_ph_1(80e6, 1500.0) / 1e3;
        assert!(t.approx_eq(0.611041229, (1e-9, 2)));
    }

    #[test]
    fn backwards_t_ps() {
        let t = t_ps_1(3e6, 0.5) / 1e3;
        assert!(t.approx_eq(0.307842258, (1e-9, 2)));

        let t = t_ps_1(80e6, 0.5) / 1e3;
        assert!(t.approx_eq(0.309979785, (1e-9, 2)));

        let t = t_ps_1(80e6, 3.0) / 1e3;
        assert!(t.approx_eq(0.565899909, (1e-9, 2)));
    }
}
