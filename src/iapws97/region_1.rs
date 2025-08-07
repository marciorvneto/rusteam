use crate::iapws97::constants;
#[cfg(feature = "nightly")]
use std::simd::prelude::*;

const REGION_1_COEFFS_II: [i32; 34] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 8, 8, 21, 23, 29,
    30, 31, 32,
];

const REGION_1_COEFFS_JI: [i32; 34] = [
    -2, -1, 0, 1, 2, 3, 4, 5, -9, -7, -1, 0, 1, 3, -3, 0, 1, 3, 17, -4, 0, 6, -5, -2, 10, -8, -11,
    -6, -29, -31, -38, -39, -40, -41,
];

const REGION_1_COEFFS_NI: [f64; 34] = [
    0.14632971213167,
    -0.84548187169114,
    -0.37563603672040e1,
    0.33855169168385e1,
    -0.95791963387872,
    0.15772038513228,
    -0.16616417199501e-1,
    0.81214629983568e-3,
    0.28319080123804e-3,
    -0.60706301565874e-3,
    -0.18990068218419e-1,
    -0.32529748770505e-1,
    -0.21841717175414e-1,
    -0.52838357969930e-4,
    -0.47184321073267e-3,
    -0.30001780793026e-3,
    0.47661393906987e-4,
    -0.44141845330846e-5,
    -0.72694996297594e-15,
    -0.31679644845054e-4,
    -0.28270797985312e-5,
    -0.85205128120103e-9,
    -0.22425281908000e-5,
    -0.65171222895601e-6,
    -0.14341729937924e-12,
    -0.40516996860117e-6,
    -0.12734301741641e-8,
    -0.17424871230634e-9,
    -0.68762131295531e-18,
    0.14478307828521e-19,
    0.26335781662795e-22,
    -0.11947622640071e-22,
    0.18228094581404e-23,
    -0.93537087292458e-25,
];

const REGION_1_BACK_COEFFS_PH_II: [i32; 20] =
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 3, 3, 4, 5, 6];

const REGION_1_BACK_COEFFS_PH_JI: [i32; 20] = [
    0, 1, 2, 6, 22, 32, 0, 1, 2, 3, 4, 10, 32, 10, 32, 10, 32, 32, 32, 32,
];

const REGION_1_BACK_COEFFS_PH_NI: [f64; 20] = [
    -0.23872489924521e+3,
    0.40421188637945e+3,
    0.11349746881718e+3,
    -0.58457616048039e+1,
    -0.15285482413140e-3,
    -0.10866707695377e-5,
    -0.13391744872602e+2,
    0.43211039183559e+2,
    -0.54010067170506e+2,
    0.30535892203916e+2,
    -0.65964749423638e+1,
    0.93965400878363e-2,
    0.11573647505340e-6,
    -0.25858641282073e-4,
    -0.40644363084799e-8,
    0.66456186191635e-7,
    0.80670734103027e-10,
    -0.93477771213947e-12,
    0.58265442020601e-14,
    -0.15020185953503e-16,
];

const REGION_1_BACK_COEFFS_PS_II: [i32; 20] =
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 4];

const REGION_1_BACK_COEFFS_PS_JI: [i32; 20] = [
    0, 1, 2, 3, 11, 31, 0, 1, 2, 3, 12, 31, 0, 1, 2, 9, 31, 10, 32, 32,
];

const REGION_1_BACK_COEFFS_PS_NI: [f64; 20] = [
    0.17478268058307e+03,
    0.34806930892873e+02,
    0.65292584978455e+01,
    0.33039981775489,
    -0.19281382923196e-06,
    -0.24909197244573e-22,
    -0.26107636489332,
    0.22592965981586,
    -0.64256463395226e-01,
    0.78876289270526e-02,
    0.35672110607366e-09,
    0.17332496994895e-23,
    0.56608900654837e-03,
    -0.32635483139717e-03,
    0.44778286690632e-04,
    -0.51322156908507e-09,
    -0.42522657042207e-25,
    0.26400441360689e-12,
    0.78124600459723e-28,
    -0.30732199903668e-30,
];

// ================    Region 1 ===================

/// Returns the region-1 gamma
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let pi = pi_1(p);
    #[cfg(feature = "nightly")]
    {
        let (tau, pi): ([f64; 34], [f64; 34]) = (
            std::array::from_fn(|x| (tau - 1.222).powi(REGION_1_COEFFS_JI[x])),
            std::array::from_fn(|x| (7.1 - pi).powi(REGION_1_COEFFS_II[x])),
        );
        let ni = Simd::<f64, 64>::load_or_default(&REGION_1_COEFFS_NI);
        let tau = Simd::<f64, 64>::load_or_default(&tau);
        let pi = Simd::<f64, 64>::load_or_default(&pi);
        (ni * tau * pi).reduce_sum()
    }
    #[cfg(not(feature = "nightly"))]
    {
        let mut sum = 0.0;
        for x in 0..REGION_1_COEFFS_II.len() {
            let ii = REGION_1_COEFFS_II[x];
            let ji = REGION_1_COEFFS_JI[x];
            let ni = REGION_1_COEFFS_NI[x];
            sum += ni * (7.1 - pi).powi(ii) * (tau - 1.222).powi(ji);
        }
        sum
    }
}

/// Returns the region-1 gamma_pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let pi = pi_1(p);
    #[cfg(feature = "nightly")]
    {
        let (tau, pi): ([f64; 34], [f64; 34]) = (
            std::array::from_fn(|x| (tau - 1.222).powi(REGION_1_COEFFS_JI[x])),
            std::array::from_fn(|x| (7.1 - pi).powi(REGION_1_COEFFS_II[x] - 1)),
        );
        let ii = Simd::<i32, 64>::load_or_default(&REGION_1_COEFFS_II).cast::<f64>();
        let ni = Simd::<f64, 64>::load_or_default(&REGION_1_COEFFS_NI);
        let tau = Simd::<f64, 64>::load_or_default(&tau);
        let pi = Simd::<f64, 64>::load_or_default(&pi);
        (-ni * ii * tau * pi).reduce_sum()
    }
    #[cfg(not(feature = "nightly"))]
    {
        let mut sum = 0.0;
        for x in 0..REGION_1_COEFFS_II.len() {
            let ii = REGION_1_COEFFS_II[x];
            let ji = REGION_1_COEFFS_JI[x];
            let ni = REGION_1_COEFFS_NI[x];
            sum += -ni * f64::from(ii) * (7.1 - pi).powi(ii - 1) * (tau - 1.222).powi(ji);
        }
        sum
    }
}

/// Returns the region-1 gamma_pi_pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_pi_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let pi = pi_1(p);
    #[cfg(feature = "nightly")]
    {
        let (tau, pi): ([f64; 34], [f64; 34]) = (
            std::array::from_fn(|x| (tau - 1.222).powi(REGION_1_COEFFS_JI[x])),
            std::array::from_fn(|x| (7.1 - pi).powi(REGION_1_COEFFS_II[x] - 2)),
        );
        let ii = Simd::<i32, 64>::load_or_default(&REGION_1_COEFFS_II).cast::<f64>();
        let ni = Simd::<f64, 64>::load_or_default(&REGION_1_COEFFS_NI);
        let tau = Simd::<f64, 64>::load_or_default(&tau);
        let pi = Simd::<f64, 64>::load_or_default(&pi);
        (ni * ii * (ii - f64x64::splat(1.0)) * tau * pi).reduce_sum()
    }
    #[cfg(not(feature = "nightly"))]
    {
        let mut sum = 0.0;
        for x in 0..REGION_1_COEFFS_II.len() {
            let ii = REGION_1_COEFFS_II[x];
            let ji = REGION_1_COEFFS_JI[x];
            let ni = REGION_1_COEFFS_NI[x];
            sum += ni * f64::from(ii * (ii - 1)) * (7.1 - pi).powi(ii - 2) * (tau - 1.222).powi(ji);
        }
        sum
    }
}

/// Returns the region-1 gamma_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let pi = pi_1(p);
    #[cfg(feature = "nightly")]
    {
        let (tau, pi): ([f64; 34], [f64; 34]) = (
            std::array::from_fn(|x| (tau - 1.222).powi(REGION_1_COEFFS_JI[x] - 1)),
            std::array::from_fn(|x| (7.1 - pi).powi(REGION_1_COEFFS_II[x])),
        );
        let ji = Simd::<i32, 64>::load_or_default(&REGION_1_COEFFS_JI).cast::<f64>();
        let ni = Simd::<f64, 64>::load_or_default(&REGION_1_COEFFS_NI);
        let tau = Simd::<f64, 64>::load_or_default(&tau);
        let pi = Simd::<f64, 64>::load_or_default(&pi);
        (ni * ji * tau * pi).reduce_sum()
    }
    #[cfg(not(feature = "nightly"))]
    {
        let mut sum = 0.0;
        for x in 0..REGION_1_COEFFS_II.len() {
            let ii = REGION_1_COEFFS_II[x];
            let ji = REGION_1_COEFFS_JI[x];
            let ni = REGION_1_COEFFS_NI[x];
            sum += ni * f64::from(ji) * (7.1 - pi).powi(ii) * (tau - 1.222).powi(ji - 1);
        }
        sum
    }
}

/// Returns the region-1 gamma_tau_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_tau_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let pi = pi_1(p);
    #[cfg(feature = "nightly")]
    {
        let (tau, pi): ([f64; 34], [f64; 34]) = (
            std::array::from_fn(|x| (tau - 1.222).powi(REGION_1_COEFFS_JI[x] - 2)),
            std::array::from_fn(|x| (7.1 - pi).powi(REGION_1_COEFFS_II[x])),
        );
        let ji = Simd::<i32, 64>::load_or_default(&REGION_1_COEFFS_JI).cast::<f64>();
        let ni = Simd::<f64, 64>::load_or_default(&REGION_1_COEFFS_NI);
        let tau = Simd::<f64, 64>::load_or_default(&tau);
        let pi = Simd::<f64, 64>::load_or_default(&pi);
        (ni * ji * (ji - f64x64::splat(1.0)) * tau * pi).reduce_sum()
    }
    #[cfg(not(feature = "nightly"))]
    {
        let mut sum = 0.0;
        for x in 0..REGION_1_COEFFS_II.len() {
            let ii = REGION_1_COEFFS_II[x];
            let ji = REGION_1_COEFFS_JI[x];
            let ni = REGION_1_COEFFS_NI[x];
            sum += ni * f64::from(ji * (ji - 1)) * (7.1 - pi).powi(ii) * (tau - 1.222).powi(ji - 2);
        }
        sum
    }
}

/// Returns the region-1 gamma_pi_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_pi_tau_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let pi = pi_1(p);
    #[cfg(feature = "nightly")]
    {
        let (tau, pi): ([f64; 34], [f64; 34]) = (
            std::array::from_fn(|x| (tau - 1.222).powi(REGION_1_COEFFS_JI[x] - 1)),
            std::array::from_fn(|x| (7.1 - pi).powi(REGION_1_COEFFS_II[x] - 1)),
        );
        let ii = Simd::<i32, 64>::load_or_default(&REGION_1_COEFFS_II).cast::<f64>();
        let ji = Simd::<i32, 64>::load_or_default(&REGION_1_COEFFS_JI).cast::<f64>();
        let ni = Simd::<f64, 64>::load_or_default(&REGION_1_COEFFS_NI);
        let tau = Simd::<f64, 64>::load_or_default(&tau);
        let pi = Simd::<f64, 64>::load_or_default(&pi);
        (-ni * ii * ji * tau * pi).reduce_sum()
    }
    #[cfg(not(feature = "nightly"))]
    {
        let mut sum = 0.0;
        for x in 0..REGION_1_COEFFS_II.len() {
            let ii = REGION_1_COEFFS_II[x];
            let ji = REGION_1_COEFFS_JI[x];
            let ni = REGION_1_COEFFS_NI[x];
            sum += -ni * f64::from(ii * ji) * (7.1 - pi).powi(ii - 1) * (tau - 1.222).powi(ji - 1);
        }
        sum
    }
}

/// Returns the region-1 tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline(always)]
fn tau_1(t: f64) -> f64 {
    1386.0_f64 / t
}

/// Returns the region-1 pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline(always)]
fn pi_1(p: f64) -> f64 {
    p / (16.53e6)
}

/// Returns the region-1 specific enthalpy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline]
pub(crate) fn h_tp_1(t: f64, p: f64) -> f64 {
    constants::_R * t * tau_1(t) * gamma_tau_1(t, p)
}

/// Returns the region-1 specific volume
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline]
pub(crate) fn v_tp_1(t: f64, p: f64) -> f64 {
    // The multiplication by 1000 is necessary to convert R from kJ/kg.K to J/kg.K
    ((constants::_R * 1000.0_f64) * t / p) * pi_1(p) * gamma_pi_1(t, p)
}

/// Returns the region-1 specific internal energy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline]
pub(crate) fn u_tp_1(t: f64, p: f64) -> f64 {
    constants::_R * t * (tau_1(t) * gamma_tau_1(t, p) - pi_1(p) * gamma_pi_1(t, p))
}

/// Returns the region-1 specific internal energy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline]
pub(crate) fn s_tp_1(t: f64, p: f64) -> f64 {
    constants::_R * (tau_1(t) * gamma_tau_1(t, p) - gamma_1(t, p))
}

/// Returns the region-1 specific isobaric heat capacity
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline]
pub(crate) fn cp_tp_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    constants::_R * (-tau.powi(2) * gamma_tau_tau_1(t, p))
}

/// Returns the region-1 specific isochoric heat capacity
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline]
pub(crate) fn cv_tp_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let corr = (gamma_pi_1(t, p) - tau * gamma_pi_tau_1(t, p)).powi(2) / gamma_pi_pi_1(t, p);
    constants::_R * (-tau.powi(2) * gamma_tau_tau_1(t, p) + corr)
}

/// Returns the region-1 speed of sound
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
#[inline]
pub(crate) fn w_tp_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let gamma_pi = gamma_pi_1(t, p);
    let gamma_pi_tau = gamma_pi_tau_1(t, p);
    let gamma_pi_pi = gamma_pi_pi_1(t, p);
    let gamma_tau_tau = gamma_tau_tau_1(t, p);
    let term = (gamma_pi - tau * gamma_pi_tau).powi(2) / (tau.powi(2) * gamma_tau_tau);

    // The multiplication by 1000 is necessary to convert R from kJ/kg.K to J/kg.K
    let square = (constants::_R * 1000.0_f64) * t * (gamma_pi.powi(2) / (term - gamma_pi_pi));
    square.sqrt()
}

/// Returns the region-1 eta for backwards calculations
/// Enthalpy is assumed to be in kJ/kg
#[inline(always)]
fn eta_1_back(h: f64) -> f64 {
    h / 2500.0_f64
}

/// Returns the region-1 pi for backwards calculations
/// Pressure is assumed to be in Pa
#[inline(always)]
fn pi_1_back(p: f64) -> f64 {
    p / 1e6
}

/// Returns the region-1 sigma for backwards calculations
/// Entropy is assumed to be in kJ/kg.K
#[inline(always)]
fn sigma_1_back(s: f64) -> f64 {
    s
}

/// Returns the region-1 backward correlation for T(p,s)
/// Entropy is assumed to be in kJ/kg.K
/// Pressure is assumed to be in Pa
#[inline]
pub fn t_ps_1(p: f64, s: f64) -> f64 {
    let sig = sigma_1_back(s);
    let pi = pi_1_back(p);
    #[cfg(feature = "nightly")]
    {
        let (sig, pi): ([f64; 20], [f64; 20]) = (
            std::array::from_fn(|x| (sig + 2.0).powi(REGION_1_BACK_COEFFS_PS_JI[x])),
            std::array::from_fn(|x| pi.powi(REGION_1_BACK_COEFFS_PS_II[x])),
        );
        let ni = Simd::<f64, 32>::load_or_default(&REGION_1_BACK_COEFFS_PS_NI);
        let sig = Simd::<f64, 32>::load_or_default(&sig);
        let pi = Simd::<f64, 32>::load_or_default(&pi);
        (ni * sig * pi).reduce_sum()
    }
    #[cfg(not(feature = "nightly"))]
    {
        let mut sum = 0.0;
        for x in 0..REGION_1_BACK_COEFFS_PS_II.len() {
            let ii = REGION_1_BACK_COEFFS_PS_II[x];
            let ji = REGION_1_BACK_COEFFS_PS_JI[x];
            let ni = REGION_1_BACK_COEFFS_PS_NI[x];
            sum += ni * pi.powi(ii) * (sig + 2.0).powi(ji);
        }
        sum
    }
}

/// Returns the region-1 backward correlation for T(p,h)
/// Enthalpy is assumed to be in kJ/kg
/// Pressure is assumed to be in Pa
#[inline]
pub fn t_ph_1(p: f64, h: f64) -> f64 {
    let eta = eta_1_back(h);
    let pi = pi_1_back(p);
    #[cfg(feature = "nightly")]
    {
        let (eta, pi): ([f64; 20], [f64; 20]) = (
            std::array::from_fn(|x| (eta + 1.0).powi(REGION_1_BACK_COEFFS_PH_JI[x])),
            std::array::from_fn(|x| pi.powi(REGION_1_BACK_COEFFS_PH_II[x])),
        );
        let ni = Simd::<f64, 32>::load_or_default(&REGION_1_BACK_COEFFS_PH_NI);
        let eta = Simd::<f64, 32>::load_or_default(&eta);
        let pi = Simd::<f64, 32>::load_or_default(&pi);
        (ni * eta * pi).reduce_sum()
    }
    #[cfg(not(feature = "nightly"))]
    {
        let mut sum = 0.0;
        for x in 0..REGION_1_BACK_COEFFS_PH_II.len() {
            let ii = REGION_1_BACK_COEFFS_PH_II[x];
            let ji = REGION_1_BACK_COEFFS_PH_JI[x];
            let ni = REGION_1_BACK_COEFFS_PH_NI[x];
            sum += ni * pi.powi(ii) * (eta + 1.0).powi(ji);
        }
        sum
    }
}
