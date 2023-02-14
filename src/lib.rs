use std::num;

const _R: f64 = 0.461526;

const REGION_1_COEFFS: [[f64; 3]; 34] = [
    [0.0, -2.0, 0.14632971213167],
    [0.0, -1.0, -0.84548187169114],
    [0.0, 0.0, -0.37563603672040 * 1e1],
    [0.0, 1.0, 0.33855169168385 * 1e1],
    [0.0, 2.0, -0.95791963387872],
    [0.0, 3.0, 0.15772038513228],
    [0.0, 4.0, -0.16616417199501 * 1e-1],
    [0.0, 5.0, 0.81214629983568 * 1e-3],
    [1.0, -9.0, 0.28319080123804 * 1e-3],
    [1.0, -7.0, -0.60706301565874 * 1e-3],
    [1.0, -1.0, -0.18990068218419 * 1e-1],
    [1.0, 0.0, -0.32529748770505 * 1e-1],
    [1.0, 1.0, -0.21841717175414 * 1e-1],
    [1.0, 3.0, -0.52838357969930 * 1e-4],
    [2.0, -3.0, -0.47184321073267 * 1e-3],
    [2.0, 0.0, -0.30001780793026 * 1e-3],
    [2.0, 1.0, 0.47661393906987 * 1e-4],
    [2.0, 3.0, -0.44141845330846 * 1e-5],
    [2.0, 17.0, -0.72694996297594 * 1e-15],
    [3.0, -4.0, -0.31679644845054 * 1e-4],
    [3.0, 0.0, -0.28270797985312 * 1e-5],
    [3.0, 6.0, -0.85205128120103 * 1e-9],
    [4.0, -5.0, -0.22425281908000 * 1e-5],
    [4.0, -2.0, -0.65171222895601 * 1e-6],
    [4.0, 10.0, -0.14341729937924 * 1e-12],
    [5.0, -8.0, -0.40516996860117 * 1e-6],
    [8.0, -11.0, -0.12734301741641 * 1e-8],
    [8.0, -6.0, -0.17424871230634 * 1e-9],
    [21.0, -29.0, -0.68762131295531 * 1e-18],
    [23.0, -31.0, 0.14478307828521 * 1e-19],
    [29.0, -38.0, 0.26335781662795 * 1e-22],
    [30.0, -39.0, -0.11947622640071 * 1e-22],
    [31.0, -40.0, 0.18228094581404 * 1e-23],
    [24.0, -41.0, -0.93537087292458 * 1e-25],
];

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

// ================    Region 1 ===================

/// Returns the region-1 gamma
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_1(T: f64, P: f64) -> f64 {
    let tau = tau_1(T);
    let pi = pi_1(P);
    let mut sum = 0.0;
    for i in 0..34 {
        let ii = REGION_1_COEFFS[i][0];
        let ji = REGION_1_COEFFS[i][1];
        let ni = REGION_1_COEFFS[i][2];
        sum += -1.0 * ni * ii * f64::powf(7.1 - pi, ii - 1.0) * f64::powf(tau - 1.222, ji);
    }
    sum
}

/// Returns the region-1 gamma_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_1(T: f64, P: f64) -> f64 {
    let tau = tau_1(T);
    let pi = pi_1(P);
    println!("P = {}, T = {}", P, T);
    println!("pi = {}, tau = {}", pi, tau);
    let mut sum = 0.0;
    for i in 0..34 {
        let ii = REGION_1_COEFFS[i][0] as i32;
        let ji = REGION_1_COEFFS[i][1] as i32;
        let ni = REGION_1_COEFFS[i][2];
        sum += ni * f64::from(ji) * (7.1 - pi).powi(ii) * (tau - 1.222).powi(ji - 1);
    }
    sum
}

/// Returns the region-1 tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn tau_1(T: f64) -> f64 {
    1386.0 / T
}

/// Returns the region-1 pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn pi_1(P: f64) -> f64 {
    P / 16.53e6
}

/// Returns the region-1 enthalpy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn h_TP_1(T: f64, P: f64) -> f64 {
    _R * T * tau_1(T) * gamma_tau_1(T, P)
}

// ================    Region 2 ===================

/// Returns the region-2 tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn tau_2(T: f64) -> f64 {
    540.0 / T
}

/// Returns the region-2 pi
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn pi_2(P: f64) -> f64 {
    P / 1e6
}

/// Returns the region-2 ideal gamma_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_2_ideal(T: f64, P: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau = tau_2(T);
    for i in 0..REGION_2_COEFFS_IDEAL.len() {
        let ji = REGION_2_COEFFS_IDEAL[i][0];
        let ni = REGION_2_COEFFS_IDEAL[i][1];
        sum += ni * ji * tau.powf(ji - 1.0);
    }
    sum
}

/// Returns the region-2 residual gamma_tau
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn gamma_tau_2_res(T: f64, P: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let tau = tau_2(T);
    let pi = pi_2(P);
    for i in 0..REGION_2_COEFFS_RES.len() {
        let ii = REGION_2_COEFFS_RES[i][0];
        let ji = REGION_2_COEFFS_RES[i][1];
        let ni = REGION_2_COEFFS_RES[i][2];
        sum += ni * pi.powf(ii) * ji * (tau - 0.5).powf(ji - 1.0);
    }
    sum
}

/// Returns the region-2 enthalpy
/// Temperature is assumed to be in K
/// Pressure is assumed to be in Pa
fn h_TP_2(T: f64, P: f64) -> f64 {
    _R * T * tau_2(T) * (gamma_tau_2_ideal(T, P) + gamma_tau_2_res(T, P))
}

// ================    Region 4 ===================

/// Returns the saturation pressure in Pa
/// Temperature is assumed to be in K
fn p_sat(T: f64) -> f64 {
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

    let theta = T + n9 / (T - n10);

    let A = theta * theta + n1 * theta + n2;
    let B = n3 * theta * theta + n4 * theta + n5;
    let C = n6 * theta * theta + n7 * theta + n8;
    (2.0 * C / (-B + (B * B - 4.0 * A * C).sqrt())).powi(4) * 1e6
}

/// Returns the saturation temperature in K
/// Pressure is assumed to be in Pa
fn t_sat(P: f64) -> f64 {
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

    let beta = (P / 1e6).powf(0.25);

    let E = beta * beta + n3 * beta + n6;
    let F = n1 * beta * beta + n4 * beta + n7;
    let G = n2 * beta * beta + n5 * beta + n8;

    let D = 2.0 * G / (-F - (F * F - 4.0 * E * G).sqrt());

    (n10 + D - ((n10 + D).powi(2) - 4.0 * (n9 + n10 * D)).sqrt()) / 2.0
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
        let h1 = h_TP_1(300.0, 3e6);

        println!("H(300, 3e6) = {}", h1);

        assert!(is_close(h1 / 1000.0, 0.115331273, 1e-9));

        let h1 = h_TP_1(300.0, 80e6);
        assert!(is_close(h1 / 1000.0, 0.184142828, 1e-9));

        let h1 = h_TP_1(500.0, 3e6);
        assert!(is_close(h1 / 1000.0, 0.975542239, 1e-9));
    }

    #[test]
    fn region_2_enthalpy() {
        let h2 = h_TP_2(300.0, 0.0035e6);
        assert!(is_close(h2 / 10000.0, 0.254991145, 1e-9));

        let h2 = h_TP_2(700.0, 0.0035e6);
        assert!(is_close(h2 / 10000.0, 0.333568375, 1e-9));

        let h2 = h_TP_2(700.0, 30e6);
        assert!(is_close(h2 / 10000.0, 0.263149474, 1e-9));
    }

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
