const _R: f64 = 0.461526;

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
            [0.0,  0.0, -0.23872489924521e+3],
            [0.0,  1.0,  0.40421188637945e+3],
            [0.0,  2.0,  0.11349746881718e+3],
            [0.0,  6.0, -0.58457616048039e+1],
            [0.0, 22.0, -0.15285482413140e-3],
            [0.0, 32.0, -0.10866707695377e-5],
            [1.0,  0.0, -0.13391744872602e+2],
            [1.0,  1.0,  0.43211039183559e+2],
            [1.0,  2.0, -0.54010067170506e+2],
            [1.0,  3.0,  0.30535892203916e+2],
            [1.0,  4.0, -0.65964749423638e+1],
            [1.0, 10.0,  0.93965400878363e-2],
            [1.0, 32.0,  0.11573647505340e-6],
            [2.0, 10.0, -0.25858641282073e-4],
            [2.0, 32.0, -0.40644363084799e-8],
            [3.0, 10.0,  0.66456186191635e-7],
            [3.0, 32.0,  0.80670734103027e-10],
            [4.0, 32.0, -0.93477771213947e-12],
            [5.0, 32.0,  0.58265442020601e-14],
            [6.0, 32.0, -0.15020185953503e-16]

];
const REGION_1_BACK_COEFFS_PS: [[f64; 3]; 20] = [
            [0.0,  0.0,  0.17478268058307e+03],
            [0.0,  1.0,  0.34806930892873e+02],
            [0.0,  2.0,  0.65292584978455e+01],
            [0.0,  3.0,  0.33039981775489],
            [0.0, 11.0, -0.19281382923196e-06],
            [0.0, 31.0, -0.24909197244573e-22],
            [1.0,  0.0, -0.26107636489332],
            [1.0,  1.0,  0.22592965981586],
            [1.0,  2.0, -0.64256463395226e-01],
            [1.0,  3.0,  0.78876289270526e-02],
            [1.0, 12.0,  0.35672110607366e-09],
            [1.0, 31.0,  0.17332496994895e-23],
            [2.0,  0.0,  0.56608900654837e-03],
            [2.0,  1.0, -0.32635483139717e-03],
            [2.0,  2.0,  0.44778286690632e-04],
            [2.0,  9.0, -0.51322156908507e-09],
            [2.0, 31.0, -0.42522657042207e-25],
            [3.0, 10.0,  0.26400441360689e-12],
            [3.0, 32.0,  0.78124600459723e-28],
            [4.0, 32.0, -0.30732199903668e-30]
];


// Region 2

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

pub enum Region {
    Region1,
    Region2,
    Region3,
    Region4,
    Region5,
}

#[derive(Debug)]
pub enum IAPWSError {
    OutOfBounds(f64, f64),
    NotImplemented()
}

// ===============     Main API ===================

pub fn region(t: f64, p: f64) -> Result<Region,IAPWSError> {
    if t < 273.15 || t > 2273.15{
        return Err(IAPWSError::OutOfBounds(t, p));
    }
    if p > 100e6 || p < 0.0{
        return Err(IAPWSError::OutOfBounds(t, p));
    }

    if t < 623.15 && p > p_sat(t) {
        if p > p_sat(t){
            return Ok(Region::Region1);
        }else if p < p_sat(t) {
            return Ok(Region::Region2);
        }
        return Ok(Region::Region4);
    }

    if t >= 1073.15 {
        if p > 50e6 {
            return Err(IAPWSError::OutOfBounds(t, p));
        }
        return Ok(Region::Region5);
    }
    

    Err(IAPWSError::OutOfBounds(t, p))
}

/// Calculates the water enthalpy in kJ/kg at a given
/// temperature and pressure.
///
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
///
/// Example
///
/// ```rust
/// use rust_steam::{h_tp};
/// let h = h_tp(300.0, 101325.0).unwrap();
/// ```
pub fn h_tp(t: f64, p: f64) -> Result<f64, IAPWSError> {
    let region = region(t, p)?;
    return match region {
        Region::Region1 => Ok(h_tp_1(t, p)),
        Region::Region2 => Ok(h_tp_2(t, p)),
        _ => Err(IAPWSError::NotImplemented())
    }
}

/// Calculates the water internal energy in kJ/kg at a given
/// temperature and pressure.
///
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
///
/// Example
///
/// ```rust
/// use rust_steam::{u_tp};
/// let u = u_tp(300.0, 101325.0).unwrap();
/// ```
pub fn u_tp(t: f64, p: f64) -> Result<f64, IAPWSError> {
    let region = region(t, p)?;
    return match region {
        Region::Region1 => Ok(u_tp_1(t, p)),
        Region::Region2 => Ok(u_tp_2(t, p)),
        _ => Err(IAPWSError::NotImplemented())
    }
}

// ================    Region 1 ===================

/// Returns the region-1 gamma
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let pi = pi_1(p);
    let mut sum = 0.0;
    for i in 0..REGION_1_COEFFS.len() {
        let ii = REGION_1_COEFFS[i][0] as i32;
        let ji = REGION_1_COEFFS[i][1] as i32;
        let ni = REGION_1_COEFFS[i][2];
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
    for i in 0..REGION_1_COEFFS.len() {
        let ii = REGION_1_COEFFS[i][0] as i32;
        let ji = REGION_1_COEFFS[i][1] as i32;
        let ni = REGION_1_COEFFS[i][2];
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
    for i in 0..REGION_1_COEFFS.len() {
        let ii = REGION_1_COEFFS[i][0] as i32;
        let ji = REGION_1_COEFFS[i][1] as i32;
        let ni = REGION_1_COEFFS[i][2];
        sum += ni * f64::from(ii*(ii-1)) * (7.1 - pi).powi(ii - 2) * (tau - 1.222).powi(ji);
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
    for i in 0..REGION_1_COEFFS.len() {
        let ii = REGION_1_COEFFS[i][0] as i32;
        let ji = REGION_1_COEFFS[i][1] as i32;
        let ni = REGION_1_COEFFS[i][2];
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
    for i in 0..REGION_1_COEFFS.len() {
        let ii = REGION_1_COEFFS[i][0] as i32;
        let ji = REGION_1_COEFFS[i][1] as i32;
        let ni = REGION_1_COEFFS[i][2];
        sum += ni * f64::from(ji * (ji-1)) * (7.1 - pi).powi(ii) * (tau - 1.222).powi(ji - 2);
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
    for i in 0..REGION_1_COEFFS.len() {
        let ii = REGION_1_COEFFS[i][0] as i32;
        let ji = REGION_1_COEFFS[i][1] as i32;
        let ni = REGION_1_COEFFS[i][2];
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
pub fn h_tp_1(t: f64, p: f64) -> f64 {
    _R * t * tau_1(t) * gamma_tau_1(t, p)
}

/// Returns the region-1 specific volume
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub fn v_tp_1(t: f64, p: f64) -> f64 {
    // The multiplication by 1000 is necessary to convert R from kJ/kg.K to J/kg.K
    ((_R * 1000.0) * t / p) * pi_1(p) * gamma_pi_1(t, p)
}

/// Returns the region-1 specific internal energy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub fn u_tp_1(t: f64, p: f64) -> f64 {
    _R * t * (tau_1(t) * gamma_tau_1(t, p) - pi_1(p) * gamma_pi_1(t,p))
}

/// Returns the region-1 specific internal energy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub fn s_tp_1(t: f64, p: f64) -> f64 {
    _R * (tau_1(t) * gamma_tau_1(t, p) - gamma_1(t,p))
}

/// Returns the region-1 specific isobaric heat capacity
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub fn cp_tp_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    _R * (-tau.powi(2) * gamma_tau_tau_1(t, p))
}

/// Returns the region-1 specific isochoric heat capacity
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub fn cv_tp_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let corr = (gamma_pi_1(t, p) - tau * gamma_pi_tau_1(t, p)).powi(2)/gamma_pi_pi_1(t, p);
    _R * (-tau.powi(2) * gamma_tau_tau_1(t, p) + corr)
}

/// Returns the region-1 speed of sound
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub fn w_tp_1(t: f64, p: f64) -> f64 {
    let tau = tau_1(t);
    let gamma_pi = gamma_pi_1(t, p);
    let gamma_pi_tau = gamma_pi_tau_1(t, p);
    let gamma_pi_pi = gamma_pi_pi_1(t, p);
    let gamma_tau_tau = gamma_tau_tau_1(t, p);
    let term = (gamma_pi - tau * gamma_pi_tau).powi(2)/(tau.powi(2) * gamma_tau_tau);

    // The multiplication by 1000 is necessary to convert R from kJ/kg.K to J/kg.K
    let square = (_R * 1000.0) * t * (gamma_pi.powi(2)/(term - gamma_pi_pi));
    square.sqrt()
}

/// Returns the region-1 theta for backwards calculations
/// Temperature is assumed to be in K
fn theta_1_back(t: f64) -> f64 {
    t
}

/// Returns the region-1 eta for backwards calculations
/// Enthalpy is assumed to be in kJ/kg
fn eta_1_back(h: f64) -> f64 {
    h / 2500.0 
}

/// Returns the region-1 pi for backwards calculations
/// Pressure is assumed to be in Pa
fn pi_1_back(p: f64) -> f64{
    p / 1e6
}

/// Returns the region-1 sigma for backwards calculations
/// Entropy is assumed to be in kJ/kg.K
fn sigma_1_back(s: f64) -> f64{
    s
}

/// Returns the region-1 backward correlation for T(p,s)
/// Entropy is assumed to be in kJ/kg.K
/// Pressure is assumed to be in Pa
pub fn t_ps_1(p: f64, s: f64) -> f64 {
    let sig = sigma_1_back(s);
    let pi = pi_1_back(p);
    let mut sum = 0.0;
    for i in 0..REGION_1_BACK_COEFFS_PS.len() {
        let ii = REGION_1_BACK_COEFFS_PS[i][0] as i32;
        let ji = REGION_1_BACK_COEFFS_PS[i][1] as i32;
        let ni = REGION_1_BACK_COEFFS_PS[i][2];
        sum += ni * pi.powi(ii) * (sig + 2.0).powi(ji);
    }
    sum
}

/// Returns the region-1 backward correlation for T(p,h)
/// Enthalpy is assumed to be in kJ/kg
/// Pressure is assumed to be in Pa
pub fn t_ph_1(p: f64, h: f64) -> f64 {
    let eta = eta_1_back(h);
    let pi = pi_1_back(p);
    let mut sum = 0.0;
    for i in 0..REGION_1_BACK_COEFFS_PH.len() {
        let ii = REGION_1_BACK_COEFFS_PH[i][0] as i32;
        let ji = REGION_1_BACK_COEFFS_PH[i][1] as i32;
        let ni = REGION_1_BACK_COEFFS_PH[i][2];
        sum += ni * pi.powi(ii) * (eta + 1.0).powi(ji);
    }
    sum
}

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
    for i in 0..REGION_2_COEFFS_IDEAL.len() {
        let ji = REGION_2_COEFFS_IDEAL[i][0];
        let ni = REGION_2_COEFFS_IDEAL[i][1];
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
    for i in 0..REGION_2_COEFFS_RES.len() {
        let ii = REGION_2_COEFFS_RES[i][0] as i32;
        let ji = REGION_2_COEFFS_RES[i][1] as i32;
        let ni = REGION_2_COEFFS_RES[i][2];
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
    for i in 0..REGION_2_COEFFS_IDEAL.len() {
        let ji = REGION_2_COEFFS_IDEAL[i][0] as i32;
        let ni = REGION_2_COEFFS_IDEAL[i][1];
        sum += ni * f64::from(ji) * tau.powi(ji - 1);
    }
    sum
}

/// Returns the region-2 ideal gamma_tau_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_tau_2_ideal(t: f64, _: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau = tau_2(t);
    for i in 0..REGION_2_COEFFS_IDEAL.len() {
        let ji = REGION_2_COEFFS_IDEAL[i][0] as i32;
        let ni = REGION_2_COEFFS_IDEAL[i][1];
        sum += ni * f64::from(ji * (ji - 1)) * tau.powi(ji - 2);
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
    for i in 0..REGION_2_COEFFS_RES.len() {
        let ii = REGION_2_COEFFS_RES[i][0] as i32;
        let ji = REGION_2_COEFFS_RES[i][1] as i32;
        let ni = REGION_2_COEFFS_RES[i][2];
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
    for i in 0..REGION_2_COEFFS_RES.len() {
        let ii = REGION_2_COEFFS_RES[i][0] as i32;
        let ji = REGION_2_COEFFS_RES[i][1] as i32;
        let ni = REGION_2_COEFFS_RES[i][2];
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
    for i in 0..REGION_2_COEFFS_RES.len() {
        let ii = REGION_2_COEFFS_RES[i][0] as i32;
        let ji = REGION_2_COEFFS_RES[i][1] as i32;
        let ni = REGION_2_COEFFS_RES[i][2];
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
    for i in 0..REGION_2_COEFFS_RES.len() {
        let ii = REGION_2_COEFFS_RES[i][0] as i32;
        let ji = REGION_2_COEFFS_RES[i][1] as i32;
        let ni = REGION_2_COEFFS_RES[i][2];
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
    for i in 0..REGION_2_COEFFS_RES.len() {
        let ii = REGION_2_COEFFS_RES[i][0] as i32;
        let ji = REGION_2_COEFFS_RES[i][1] as i32;
        let ni = REGION_2_COEFFS_RES[i][2];
        sum += ni * pi.powi(ii - 1) * f64::from(ii * ji) * (tau - 0.5).powi(ji - 1);
    }
    sum
}

/// Returns the region-2 specific volume
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub fn v_tp_2(t: f64, p: f64) -> f64 {
    ((_R * 1000.0) * t / p) * pi_2(p) * (gamma_pi_2_ideal(t, p) + gamma_pi_2_res(t, p))
}

/// Returns the region-2 enthalpy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub fn h_tp_2(t: f64, p: f64) -> f64 {
    _R * t * tau_2(t) * (gamma_tau_2_ideal(t, p) + gamma_tau_2_res(t, p))
}

/// Returns the region-2 internal energy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub fn u_tp_2(t: f64, p: f64) -> f64 {
    let tau = tau_2(t);
    let pi = pi_2(p);
    let tau_term = gamma_tau_2_ideal(t, p) + gamma_tau_2_res(t, p);
    let pi_term = gamma_pi_2_ideal(t, p) + gamma_pi_2_res(t, p);
    _R * t * (tau * tau_term - pi * pi_term)
}

/// Returns the region-2 entropy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub fn s_tp_2(t: f64, p: f64) -> f64 {
    let tau = tau_2(t);
    let tau_term = gamma_tau_2_ideal(t, p) + gamma_tau_2_res(t, p);
    let pi_term = gamma_2_ideal(t, p) + gamma_2_res(t, p);
    _R * (tau * tau_term - pi_term)
}

/// Returns the region-2 isobaric specific heat
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub fn cp_tp_2(t: f64, p: f64) -> f64 {
    let tau = tau_2(t);
    - _R  * tau.powi(2) * (gamma_tau_tau_2_ideal(t, p) + gamma_tau_tau_2_res(t, p))
}

/// Returns the region-2 isochoric specific heat
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub fn cv_tp_2(t: f64, p: f64) -> f64 {
    let tau = tau_2(t);
    let pi = pi_2(p);
    let num = (1.0 + pi*gamma_pi_2_res(t, p) - tau * pi * gamma_pi_tau_2_res(t,p)).powi(2);
    let den = 1.0 - pi.powi(2) * gamma_pi_pi_2_res(t,p);
    cp_tp_2(t, p) - _R * num/ den
}

/// Returns the region-2 sound velocity
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
pub fn w_tp_2(t: f64, p: f64) -> f64 {
    let tau = tau_2(t);
    let pi = pi_2(p);
    let num = 1.0 + 2.0*pi*gamma_pi_2_res(t, p) +  pi.powi(2) * gamma_pi_2_res(t,p).powi(2);
    let subnum = (1.0 + pi * gamma_pi_2_res(t, p) - tau * pi * gamma_pi_tau_2_res(t, p)).powi(2);
    let subden = tau.powi(2)*(gamma_tau_tau_2_ideal(t, p) + gamma_tau_tau_2_res(t, p));
    let den = 1.0 - pi.powi(2) * gamma_pi_pi_2_res(t,p) + subnum / subden;
    ((_R * 1000.0 * t) * num / den).sqrt()
}

// ================    Region 4 ===================

/// Returns the saturation pressure in Pa
/// Temperature is assumed to be in K
pub fn p_sat(t: f64) -> f64 {
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
/// Pressure is assumed to be in Pa
pub fn t_sat(p: f64) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn is_close(x: f64, y: f64, tol: f64) -> bool {
        (x - y).abs() < tol
    }

    #[test]
    fn canary() {}

    #[test]
    fn region_1_enthalpy() {
        let h1 = h_tp_1(300.0, 3e6);

        assert!(is_close(h1 / 1000.0, 0.115331273, 1e-9));

        let h1 = h_tp_1(300.0, 80e6);
        assert!(is_close(h1 / 1000.0, 0.184142828, 1e-9));

        let h1 = h_tp_1(500.0, 3e6);
        assert!(is_close(h1 / 1000.0, 0.975542239, 1e-9));
    }

    #[test]
    fn region_1_internal_energy() {
        let u1 = u_tp_1(300.0, 3e6);

        assert!(is_close(u1 / 1000.0, 0.112324818, 1e-9));

        let u1 = u_tp_1(300.0, 80e6);
        assert!(is_close(u1 / 1000.0, 0.106448356, 1e-9));

        let u1 = u_tp_1(500.0, 3e6);
        assert!(is_close(u1 / 1000.0, 0.971934985, 1e-9));
    }

    #[test]
    fn region_1_entropy() {
        let s1 = s_tp_1(300.0, 3e6);
        assert!(is_close(s1 , 0.392294792, 1e-9));

        let s1 = s_tp_1(300.0, 80e6);
        assert!(is_close(s1 , 0.368563852, 1e-9));

        let s1 = s_tp_1(500.0, 3e6);
        assert!(is_close(s1 , 0.258041912e1, 1e-9));
    }

    #[test]
    fn region_1_cp() {
        let cp1 = cp_tp_1(300.0, 3e6);
        println!("{}", cp1);
        assert!(is_close(cp1/10.0 , 0.417301218, 1e-9));

        let cp1 = cp_tp_1(300.0, 80e6);
        assert!(is_close(cp1/10.0 , 0.401008987, 1e-9));

        let cp1 = cp_tp_1(500.0, 3e6);
        assert!(is_close(cp1/10.0 , 0.465580682, 1e-9));
    }

    #[test]
    fn region_1_sound_velocity() {
        let w1 = w_tp_1(300.0, 3e6);
        assert!(is_close(w1/10000.0 , 0.150773921 , 1e-9));

        let w1 = w_tp_1(300.0, 80e6);
        assert!(is_close(w1/10000.0 , 0.163469054, 1e-9));

        let w1 = w_tp_1(500.0, 3e6);
        assert!(is_close(w1/10000.0 , 0.124071337, 1e-9));
    }

    #[test]
    fn region_1_specific_volume() {
        let v1 = v_tp_1(300.0, 3e6);
        assert!(is_close(v1 * 100.0, 0.100215168, 1e-9));

        let v1 = v_tp_1(300.0, 80e6);
        assert!(is_close(v1 * 1000.0 , 0.971180894, 1e-9));

        let v1 = v_tp_1(500.0, 3e6);
        assert!(is_close(v1 * 100.0, 0.120241800, 1e-9));
    }

    #[test]
    fn region_1_backwards_t_ph() {
        let t = t_ph_1( 3e6, 500.0);
        assert!(is_close(t / 1000.0, 0.391798509, 1e-9));

        let t = t_ph_1( 80e6, 500.0);
        assert!(is_close(t / 1000.0, 0.378108626, 1e-9));

        let t = t_ph_1( 80e6, 1500.0);
        assert!(is_close(t / 1000.0, 0.611041229, 1e-9));

    }

    #[test]
    fn region_1_backwards_t_ps() {
        let t = t_ps_1( 3e6, 0.5);
        assert!(is_close(t / 1000.0, 0.307842258, 1e-9));

        let t = t_ps_1( 80e6, 0.5);
        assert!(is_close(t / 1000.0, 0.309979785, 1e-9));

        let t = t_ps_1( 80e6, 3.0);
        assert!(is_close(t / 1000.0, 0.565899909, 1e-9));

    }

    #[test]
    fn region_2_specific_volume() {
        let v = v_tp_2(300.0, 0.0035e6);
        assert!(is_close(v / 100.0, 0.394913866, 1e-9));

        let v = v_tp_2(700.0, 0.0035e6);
        assert!(is_close(v / 100.0, 0.923015898 , 1e-9));

        let v = v_tp_2(700.0, 30e6);
        assert!(is_close(v / 0.01, 0.542946619, 1e-9));
    }

    #[test]
    fn region_2_enthalpy() {
        let h = h_tp_2(300.0, 0.0035e6);
        assert!(is_close(h / 10000.0, 0.254991145, 1e-9));

        let h = h_tp_2(700.0, 0.0035e6);
        assert!(is_close(h / 10000.0, 0.333568375, 1e-9));

        let h = h_tp_2(700.0, 30e6);
        assert!(is_close(h / 10000.0, 0.263149474, 1e-9));
    }

    #[test]
    fn region_2_internal_energy() {
        let u = u_tp_2(300.0, 0.0035e6);
        assert!(is_close(u / 10000.0, 0.241169160, 1e-9));

        let u = u_tp_2(700.0, 0.0035e6);
        assert!(is_close(u / 10000.0, 0.301262819, 1e-9));

        let u = u_tp_2(700.0, 30e6);
        assert!(is_close(u / 10000.0, 0.246861076, 1e-9));
    }

    #[test]
    fn region_2_entropy() {
        let s = s_tp_2(300.0, 0.0035e6);
        println!("{}", s);
        assert!(is_close(s / 10.0, 0.852238967, 1e-9));

        let s = s_tp_2(700.0, 0.0035e6);
        assert!(is_close(s / 100.0, 0.101749996, 1e-9));

        let s = s_tp_2(700.0, 30e6);
        assert!(is_close(s / 10.0, 0.517540298, 1e-9));
    }

    #[test]
    fn region_2_cp() {
        let cp = cp_tp_2(300.0, 0.0035e6);
        println!("{}", cp);
        assert!(is_close(cp / 10.0, 0.191300162, 1e-9));

        let cp = cp_tp_2(700.0, 0.0035e6);
        assert!(is_close(cp / 10.0, 0.208141274 , 1e-9));

        let cp = cp_tp_2(700.0, 30e6);
        assert!(is_close(cp / 100.0, 0.103505092, 1e-9));
    }

    #[test]
    fn region_2_sound_velocity() {
        let w = w_tp_2(300.0, 0.0035e6);
        println!("{}", w);
        assert!(is_close(w / 1000.0, 0.427920172, 1e-9));

        let w = w_tp_2(700.0, 0.0035e6);
        assert!(is_close(w / 1000.0, 0.644289068, 1e-9));

        let w = w_tp_2(700.0, 30e6);
        assert!(is_close(w / 1000.0, 0.480386523, 1e-9));
    }


    // Region 4

    #[test]
    fn region_4_saturation_pressure() {
        let ps = p_sat(300.0);
        assert!(is_close(ps / 10000.0, 0.353658941, 1e-9));

        let ps = p_sat(500.0);
        assert!(is_close(ps / 1e7, 0.263889776, 1e-9));

        let ps = p_sat(600.0);
        assert!(is_close(ps / 1e8, 0.123443146, 1e-9));
    }

    #[test]
    fn region_4_saturation_temperature() {
        let ts = t_sat(0.1e6);
        assert!(is_close(ts / 1000.0, 0.372755919, 1e-9));

        let ts = t_sat(1e6);
        assert!(is_close(ts / 1000.0, 0.453035632, 1e-9));

        let ts = t_sat(10e6);
        assert!(is_close(ts / 1000.0, 0.584149488, 1e-9));
    }
}
