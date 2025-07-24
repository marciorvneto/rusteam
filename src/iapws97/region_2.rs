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

#[allow(dead_code)]
fn t_ps_2a(pi: f64, s: f64) -> f64 {
    let i: [f64; 46] = [
        -1.5, -1.5, -1.5, -1.5, -1.5, -1.5, -1.25, -1.25, -1.25, -1.0, -1.0, -1.0, -1.0, -1.0,
        -1.0, -0.75, -0.75, -0.5, -0.5, -0.5, -0.5, -0.25, -0.25, -0.25, -0.25, 0.25, 0.25, 0.25,
        0.25, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.75, 0.75, 0.75, 0.75, 1.0, 1.0, 1.25, 1.25, 1.5,
        1.5,
    ];
    let j: [i32; 46] = [
        -24, -23, -19, -13, -11, -10, -19, -15, -6, -26, -21, -17, -16, -9, -8, -15, -14, -26, -13,
        -9, -7, -27, -25, -11, -6, 1, 4, 8, 11, 0, 1, 5, 6, 10, 14, 16, 0, 4, 9, 17, 7, 18, 3, 15,
        5, 18,
    ];
    let n: [f64; 46] = [
        -0.39235983861984e6,
        0.51526573827270e6,
        0.40482443161048e5,
        -0.32193790923902e3,
        0.96961424218694e2,
        -0.22867846371773e2,
        -0.44942914124357e6,
        -0.50118336020166e4,
        0.35684463560015,
        0.44235335848190e5,
        -0.13673388811708e5,
        0.42163260207864e6,
        0.22516925837475e5,
        0.47442144865646e3,
        -0.14931130797647e3,
        -0.19781126320452e6,
        -0.23554399470760e5,
        -0.19070616302076e5,
        0.55375669883164e5,
        0.38293691437363e4,
        -0.60391860580567e3,
        0.19363102620331e4,
        0.42660643698610e4,
        -0.59780638872718e4,
        -0.70401463926862e3,
        0.33836784107553e3,
        0.20862786635187e2,
        0.33834172656196e-1,
        -0.43124428414893e-4,
        0.16653791356412e3,
        -0.13986292055898e3,
        -0.78849547999872,
        0.72132411753872e-1,
        -0.59754839398283e-2,
        -0.12141358953904e-4,
        0.23227096733871e-6,
        -0.10538463566194e2,
        0.20718925496502e1,
        -0.72193155260427e-1,
        0.20749887081120e-6,
        -0.18340657911379e-1,
        0.29036272348696e-6,
        0.21037527893619,
        0.25681239729999e-3,
        -0.12799002933781e-1,
        -0.82198102652018e-5,
    ];

    // Calculate T
    let x: [usize; 46] = core::array::from_fn(|i| i + 1);
    x.into_iter()
        .map(|x| n[x - 1] * pi.powf(i[x - 1]) * ((s / 2.0) - 2.0).powi(j[x - 1]))
        .sum()
}

#[allow(dead_code)]
fn t_ps_2b(pi: f64, s: f64) -> f64 {
    let i: [i32; 44] = [
        -6, -6, -5, -5, -4, -4, -4, -3, -3, -3, -3, -2, -2, -2, -2, -1, -1, -1, -1, -1, 0, 0, 0, 0,
        0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 5,
    ];
    let j: [i32; 44] = [
        0, 11, 0, 11, 0, 1, 11, 0, 1, 11, 12, 0, 1, 6, 10, 0, 1, 5, 8, 9, 0, 1, 2, 4, 5, 6, 9, 0,
        1, 2, 3, 7, 8, 0, 1, 5, 0, 1, 3, 0, 1, 0, 1, 2,
    ];
    let n: [f64; 44] = [
        0.31687665083497e6,
        0.20864175881858e2,
        -0.39859399803599e6,
        -0.21816058518877e2,
        0.22369785194242e6,
        -0.27841703445817e4,
        0.99207436071480e1,
        -0.75197512299157e5,
        0.29708605951158e4,
        -0.34406878548526e1,
        0.38815564249115,
        0.17511295085750e5,
        -0.14237112854449e4,
        0.10943803364167e1,
        0.89971619308495,
        -0.33759740098958e4,
        0.47162885818355e3,
        -0.19188241993679e1,
        0.41078580492196,
        -0.33465378172097,
        0.13870034777505e4,
        -0.40663326195838e3,
        0.41727347159610e2,
        0.21932549434532e1,
        -0.10320050009077e1,
        0.35882943516703,
        0.52511453726066e-2,
        0.12838916450705e2,
        -0.28642437219381e1,
        0.56912683664855,
        -0.99962954584931e-1,
        -0.32632037778459e-2,
        0.23320922576723e-3,
        -0.15334809857450,
        0.29072288239902e-1,
        0.37534702741167e-3,
        0.17296691702411e-2,
        -0.38556050844504e-3,
        -0.35017712292608e-4,
        -0.14566393631492e-4,
        0.56420857267269e-5,
        0.41286150074605e-7,
        -0.20684671118824e-7,
        0.16409393674725e-8,
    ];

    // Calculate T
    let x: [usize; 44] = core::array::from_fn(|i| i + 1);
    #[allow(clippy::approx_constant)]
    x.into_iter()
        .map(|x| n[x - 1] * pi.powi(i[x - 1]) * (10.0 - s / 0.7853).powi(j[x - 1]))
        .sum()
}

#[allow(dead_code)]
fn t_ps_2c(pi: f64, s: f64) -> f64 {
    let i: [i32; 30] = [
        -2, -2, -1, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 7, 7, 7, 7, 7,
    ];
    let j: [i32; 30] = [
        0, 1, 0, 0, 1, 2, 3, 0, 1, 3, 4, 0, 1, 2, 0, 1, 5, 0, 1, 4, 0, 1, 2, 0, 1, 0, 1, 3, 4, 5,
    ];
    let n: [f64; 30] = [
        0.90968501005365e3,
        0.24045667088420e4,
        -0.59162326387130e3,
        0.54145404128074e3,
        -0.27098308411192e3,
        0.97976525097926e3,
        -0.46966772959435e3,
        0.14399274604723e2,
        -0.19104204230429e2,
        0.53299167111971e1,
        -0.21252975375934e2,
        -0.31147334413760,
        0.60334840894623,
        -0.42764839702509e-1,
        0.58185597255259e-2,
        -0.14597008284753e-1,
        0.56631175631027e-2,
        -0.76155864584577e-4,
        0.22440342919332e-3,
        -0.12561095013413e-4,
        0.63323132660934e-6,
        -0.20541989675375e-5,
        0.36405370390082e-7,
        -0.29759897789215e-8,
        0.10136618529763e-7,
        0.59925719692351e-11,
        -0.20677870105164e-10,
        -0.20874278181886e-10,
        0.10162166825089e-9,
        -0.16429828281347e-9,
    ];

    // Calculate T
    let x: [usize; 30] = core::array::from_fn(|i| i + 1);
    x.into_iter()
        .map(|x| n[x - 1] * pi.powi(i[x - 1]) * (2.0 - (s / 2.9251)).powi(j[x - 1]))
        .sum()
}

/// Returns the region-1 backward correlation for T(p,s)
/// Entropy is assumed to be in kJ/kg.K
/// Pressure is assumed to be in Pa
#[allow(dead_code)]
pub fn t_ps_2(p: f64, s: f64) -> f64 {
    let pi = p / 1e6;

    match (p, s) {
        (pres, ..) if (0.0..=4e6).contains(&pres) => t_ps_2a(pi, s),
        (pres, ent) if (4e6..=100e6).contains(&pres) && ent >= 5.85 => t_ps_2b(pi, s),
        _ => t_ps_2c(pi, s),
    }
}

#[allow(dead_code)]
fn t_ph_2a(pi: f64, eta: f64) -> f64 {
    let i: [i32; 34] = [
        0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 4, 4, 4, 5, 5,
        5, 6, 6, 7,
    ];
    let j: [i32; 34] = [
        0, 1, 2, 3, 7, 20, 0, 1, 2, 3, 7, 9, 11, 18, 44, 0, 2, 7, 36, 38, 40, 42, 44, 24, 44, 12,
        32, 44, 32, 36, 42, 34, 44, 28,
    ];
    let n: [f64; 34] = [
        0.10898952318288e4,
        0.84951654495535e3,
        -0.10781748091826e3,
        0.33153654801263e2,
        -0.74232016790248e1,
        0.11765048724356e2,
        0.18445749355790e1,
        -0.41792700549624e1,
        0.62478196935812e1,
        -0.17344563108114e2,
        -0.20058176862096e3,
        0.27196065473796e3,
        -0.45511318285818e3,
        0.30919688604755e4,
        0.25226640357872e6,
        -0.61707422868339e-2,
        -0.31078046629583,
        0.11670873077107e2,
        0.12812798404046e9,
        -0.98554909623276e9,
        0.28224546973002e10,
        -0.35948971410703e10,
        0.17227349913197e10,
        -0.13551334240775e5,
        0.12848734664650e8,
        0.13865724283226e1,
        0.23598832556514e6,
        -0.13105236545054e8,
        0.73999835474766e4,
        -0.55196697030060e6,
        0.37154085996233e7,
        0.19127729239660e5,
        -0.41535164835634e6,
        -0.62459855192507e2,
    ];

    // Calculate T
    let x: [usize; 34] = core::array::from_fn(|i| i + 1);
    x.into_iter()
        .map(|x| n[x - 1] * pi.powi(i[x - 1]) * (eta - 2.1).powi(j[x - 1]))
        .sum()
}

#[allow(dead_code)]
fn t_ph_2b(pi: f64, eta: f64) -> f64 {
    let i: [i32; 38] = [
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4,
        5, 5, 5, 6, 7, 7, 9, 9,
    ];
    let j: [i32; 38] = [
        0, 1, 2, 12, 18, 24, 28, 40, 0, 2, 6, 12, 18, 24, 28, 40, 2, 8, 18, 40, 1, 2, 12, 24, 2,
        12, 18, 24, 28, 40, 18, 24, 40, 28, 2, 28, 1, 40,
    ];
    let n: [f64; 38] = [
        0.14895041079516e4,
        0.74307798314034e3,
        -0.97708318797837e2,
        0.24742464705674e1,
        -0.63281320016026,
        0.11385952129658e1,
        -0.47811863648625,
        0.85208123431544e-2,
        0.93747147377932,
        0.33593118604916e1,
        0.33809355601454e1,
        0.16844539671904,
        0.73875745236695,
        -0.47128737436186,
        0.15020273139707,
        -0.21764114219750e-2,
        -0.21810755324761e-1,
        -0.10829784403677,
        -0.46333324635812e-1,
        0.71280351959551e-4,
        0.11032831789999e-3,
        0.18955248387902e-3,
        0.30891541160537e-2,
        0.13555504554949e-2,
        0.28640237477456e-6,
        -0.10779857357512e-4,
        -0.76462712454814e-4,
        0.14052392818316e-4,
        -0.31083814331434e-4,
        -0.10302738212103e-5,
        0.28217281635040e-6,
        0.12704902271945e-5,
        0.73803353468292e-7,
        -0.11030139238909e-7,
        -0.81456365207833e-13,
        -0.25180545682962e-10,
        -0.17565233969407e-17,
        0.86934156344163e-14,
    ];

    // Calculate T
    let x: [usize; 38] = core::array::from_fn(|i| i + 1);

    x.into_iter()
        .map(|x| n[x - 1] * (pi - 2.0).powi(i[x - 1]) * (eta - 2.6).powi(j[x - 1]))
        .sum()
}

#[allow(dead_code)]
fn t_ph_2c(pi: f64, eta: f64) -> f64 {
    let i: [i32; 23] = [
        -7, -7, -6, -6, -5, -5, -2, -2, -1, -1, 0, 0, 1, 1, 2, 6, 6, 6, 6, 6, 6, 6, 6,
    ];
    let j: [i32; 23] = [
        0, 4, 0, 2, 0, 2, 0, 1, 0, 2, 0, 1, 4, 8, 4, 0, 1, 4, 10, 12, 16, 20, 22,
    ];
    let n: [f64; 23] = [
        -0.32368398555242e13,
        0.73263350902181e13,
        0.35825089945447e12,
        -0.58340131851590e12,
        -0.10783068217470e11,
        0.20825544563171e11,
        0.61074783564516e6,
        0.85977722535580e6,
        -0.25745723604170e5,
        0.31081088422714e5,
        0.12082315865936e4,
        0.48219755109255e3,
        0.37966001272486e1,
        -0.10842984880077e2,
        -0.45364172676660e-1,
        0.14559115658698e-12,
        0.11261597407230e-11,
        -0.17804982240686e-10,
        0.12324579690832e-6,
        -0.11606921130984e-5,
        0.27846367088554e-4,
        -0.59270038474176e-3,
        0.12918582991878e-2,
    ];

    // Calculate T
    let x: [usize; 23] = core::array::from_fn(|i| i + 1);
    x.into_iter()
        .map(|x| n[x - 1] * (pi + 25.0).powi(i[x - 1]) * (eta - 1.8).powi(j[x - 1]))
        .sum()
}

/// Returns the region-2 back calculated T(p,h)
/// Pressure is assumed to be in Pa
/// Enthalpy is assumed to be in kJ/kg
#[allow(dead_code)]
pub fn t_ph_2(p: f64, h: f64) -> f64 {
    let pi = p / 1e6;
    let eta = h / 2.0e3;
    let p_2b2c =
        (0.90584278514723e3 + -0.67955786399241 * h + 0.12809002730136e-3 * h.powi(2)) * 1e6;

    match p {
        pres if (0.0..=4.0e6).contains(&pres) => t_ph_2a(pi, eta),
        pres if (4.0e6..=100.0e6).contains(&pres) && pres < p_2b2c => t_ph_2b(pi, eta),
        _ => t_ph_2c(pi, eta),
    }
}
