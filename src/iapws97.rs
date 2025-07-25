#![no_std]
#![allow(unused_imports)]
use intrinsics;
pub mod iapws97 {
    mod constants;
    mod region_1;
    mod region_2;
    mod region_3;
    mod region_5;
    use crate::iapws97::region_1::{
        cp_tp_1, cv_tp_1, h_tp_1, s_tp_1, t_ph_1, t_ps_1, u_tp_1, v_tp_1, w_tp_1,
    };
    use crate::iapws97::region_2::{
        cp_tp_2, cv_tp_2, h_tp_2, s_tp_2, t_ph_2, t_ps_2, u_tp_2, v_tp_2, w_tp_2,
    };
    use crate::iapws97::region_3::{cp_tp_3, cv_tp_3, h_tp_3, s_tp_3, u_tp_3, v_tp_3, w_tp_3};
    use crate::iapws97::region_5::{cp_tp_5, cv_tp_5, h_tp_5, s_tp_5, u_tp_5, v_tp_5, w_tp_5};

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
        NotImplemented(),
    }

    fn p_boundary_2_3(t: &f64) -> f64 {
        let n: [f64; 5] = [
            0.34805185628969e3,
            -0.11671859879975e1,
            0.10192970039326e-2,
            0.57254459862746e3,
            0.13918839778870e2,
        ];
        1e6 * (n[0] + n[1] * t + n[2] * t.powi(2))
    }

    #[allow(dead_code)]
    fn t_boundary_2_3(p: &f64) -> f64 {
        let n: [f64; 5] = [
            0.34805185628969e3,
            -0.11671859879975e1,
            0.10192970039326e-2,
            0.57254459862746e3,
            0.13918839778870e2,
        ];
        n[3] + (((p * 1e-6) - n[4]) / n[2]).sqrt()
    }

    // ===============     Main API ===================

    /// Determines which region of the pT chart
    /// a point belongs to.
    ///
    /// Returns an error if the point is outside the
    /// bounds of the IAPWS-IF97 correlations.
    ///
    /// Temperature is assumed to be in K
    /// Pressure is assumed to be in Pa
    ///
    /// Example
    ///
    /// ```compile_fail
    /// use rust_steam::iapws97::{region};
    /// let region = region(300.0, 101325.0).unwrap();
    /// ```
    fn region(t: f64, p: f64) -> Result<Region, IAPWSError> {
        let p_sat = psat97(&t);

        let p_boundary_23 = p_boundary_2_3(&t);

        match (t, p) {
            (temp, pres)
                if (1073.15..=2273.15).contains(&temp) && (0.0..=50.0e6).contains(&pres) =>
            {
                Ok(Region::Region5)
            }
            (temp, pres) if (273.15..647.096).contains(&temp) && pres == p_sat => {
                Ok(Region::Region4)
            }
            (temp, pres)
                if (623.15..=863.15).contains(&temp) && (p_boundary_23..100e6).contains(&pres) =>
            {
                Ok(Region::Region3)
            }
            (temp, pres)
                if ((273.15..=623.15).contains(&temp) && (0.0..=p_sat).contains(&pres))
                    || ((623.15..=863.15).contains(&temp)
                        && (0.0..=p_boundary_23).contains(&pres))
                    || ((863.15..=1073.15).contains(&temp) && (0.0..100e6).contains(&pres)) =>
            {
                Ok(Region::Region2)
            }
            (temp, pres)
                if (273.15..=623.15).contains(&temp) && (p_sat..=100e6).contains(&pres) =>
            {
                Ok(Region::Region1)
            }
            _ => Err(IAPWSError::OutOfBounds(t, p)),
        }
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
    /// use rust_steam::iapws97::{hmass_tp};
    /// let h = hmass_tp(300.0, 101325.0).unwrap();
    /// ```
    pub fn hmass_tp(t: f64, p: f64) -> Result<f64, IAPWSError> {
        let region = region(t, p)?;
        match region {
            Region::Region1 => Ok(h_tp_1(t, p)),
            Region::Region2 => Ok(h_tp_2(t, p)),
            Region::Region3 => Ok(h_tp_3(t, p)),
            Region::Region5 => Ok(h_tp_5(t, p)),
            _ => Err(IAPWSError::NotImplemented()),
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
    /// use rust_steam::iapws97::{umass_tp};
    /// let u = umass_tp(300.0, 101325.0).unwrap();
    /// ```
    pub fn umass_tp(t: f64, p: f64) -> Result<f64, IAPWSError> {
        let region = region(t, p)?;
        match region {
            Region::Region1 => Ok(u_tp_1(t, p)),
            Region::Region2 => Ok(u_tp_2(t, p)),
            Region::Region3 => Ok(u_tp_3(t, p)),
            Region::Region5 => Ok(u_tp_5(t, p)),
            _ => Err(IAPWSError::NotImplemented()),
        }
    }

    /// Calculates the water entropy in J/kg.K at a given
    /// temperature and pressure.
    ///
    /// Temperature is assumed to be in K
    /// Pressure is assumed to be in Pa
    ///
    /// Example
    ///
    /// ```rust
    /// use rust_steam::iapws97::{smass_tp};
    /// let s = smass_tp(300.0, 101325.0).unwrap();
    /// ```
    pub fn smass_tp(t: f64, p: f64) -> Result<f64, IAPWSError> {
        let region = region(t, p)?;
        match region {
            Region::Region1 => Ok(s_tp_1(t, p)),
            Region::Region2 => Ok(s_tp_2(t, p)),
            Region::Region3 => Ok(s_tp_3(t, p)),
            Region::Region5 => Ok(s_tp_5(t, p)),
            _ => Err(IAPWSError::NotImplemented()),
        }
    }

    /// Calculates the mass constant pressure heat in J/kg/K at a given
    /// temperature and pressure.
    ///
    /// Temperature is assumed to be in K
    /// Pressure is assumed to be in Pa
    ///
    /// Example
    ///
    /// ```rust
    /// use rust_steam::iapws97::{cpmass_tp};
    /// let cp = cpmass_tp(300.0, 101325.0).unwrap();
    /// ```
    pub fn cpmass_tp(t: f64, p: f64) -> Result<f64, IAPWSError> {
        let region = region(t, p)?;
        match region {
            Region::Region1 => Ok(cp_tp_1(t, p)),
            Region::Region2 => Ok(cp_tp_2(t, p)),
            Region::Region3 => Ok(cp_tp_3(t, p)),
            Region::Region5 => Ok(cp_tp_5(t, p)),
            _ => Err(IAPWSError::NotImplemented()),
        }
    }

    /// Calculates the mass constant volume heat in J/kg/K at a given
    /// temperature and pressure.
    ///
    /// Temperature is assumed to be in K
    /// Pressure is assumed to be in Pa
    ///
    /// Example
    ///
    /// ```rust
    /// use rust_steam::iapws97::{cvmass_tp};
    /// let cv = cvmass_tp(300.0, 101325.0).unwrap();
    /// ```
    pub fn cvmass_tp(t: f64, p: f64) -> Result<f64, IAPWSError> {
        let region = region(t, p)?;
        match region {
            Region::Region1 => Ok(cv_tp_1(t, p)),
            Region::Region2 => Ok(cv_tp_2(t, p)),
            Region::Region3 => Ok(cv_tp_3(t, p)),
            Region::Region5 => Ok(cv_tp_5(t, p)),
            _ => Err(IAPWSError::NotImplemented()),
        }
    }

    /// Calculates the mass volume in m^3/kg at a given
    /// temperature and pressure.
    ///
    /// Temperature is assumed to be in K
    /// Pressure is assumed to be in Pa
    ///
    /// Example
    ///
    /// ```rust
    /// use rust_steam::iapws97::{vmass_tp};
    /// let v = vmass_tp(300.0, 101325.0).unwrap();
    /// ```
    pub fn vmass_tp(t: f64, p: f64) -> Result<f64, IAPWSError> {
        let region = region(t, p)?;
        match region {
            Region::Region1 => Ok(v_tp_1(t, p)),
            Region::Region2 => Ok(v_tp_2(t, p)),
            Region::Region3 => Ok(v_tp_3(t, p)),
            Region::Region5 => Ok(v_tp_5(t, p)),
            _ => Err(IAPWSError::NotImplemented()),
        }
    }

    /// Calculates the speed of sound in m/s at a given
    /// temperature and pressure.
    ///
    /// Temperature is assumed to be in K
    /// Pressure is assumed to be in Pa
    ///
    /// Example
    ///
    /// ```rust
    /// use rust_steam::iapws97::{speed_sound_tp};
    /// let w = speed_sound_tp(300.0, 101325.0).unwrap();
    /// ```
    pub fn speed_sound_tp(t: f64, p: f64) -> Result<f64, IAPWSError> {
        let region = region(t, p)?;
        match region {
            Region::Region1 => Ok(w_tp_1(t, p)),
            Region::Region2 => Ok(w_tp_2(t, p)),
            Region::Region3 => Ok(w_tp_3(t, p)),
            Region::Region5 => Ok(w_tp_5(t, p)),
            _ => Err(IAPWSError::NotImplemented()),
        }
    }

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
    pub fn psat97(t: &f64) -> f64 {
        // Calulate additional values
        let theta: f64 = t + REGION_4_SATURATION_COEFFS[8] / (t - REGION_4_SATURATION_COEFFS[9]);
        let coef_a: f64 =
            theta.powi(2) + REGION_4_SATURATION_COEFFS[0] * theta + REGION_4_SATURATION_COEFFS[1];
        let coef_b: f64 = REGION_4_SATURATION_COEFFS[2] * theta.powi(2)
            + REGION_4_SATURATION_COEFFS[3] * theta
            + REGION_4_SATURATION_COEFFS[4];
        let coef_c: f64 = REGION_4_SATURATION_COEFFS[5] * theta.powi(2)
            + REGION_4_SATURATION_COEFFS[6] * theta
            + REGION_4_SATURATION_COEFFS[7];

        // Return p_sat
        (2.0 * coef_c / (-coef_b + (coef_b.powi(2) - 4.0 * coef_a * coef_c).sqrt())).powi(4) * 1e6
    }

    /// Returns the saturation temperature in K
    /// Pressure is assumed to be in Pa
    pub fn tsat97(p: &f64) -> f64 {
        // Calulate additional values
        let beta: f64 = (p * 1e-6).powf(0.25);
        let coef_e: f64 =
            beta.powi(2) + REGION_4_SATURATION_COEFFS[2] * beta + REGION_4_SATURATION_COEFFS[5];
        let coef_f: f64 = REGION_4_SATURATION_COEFFS[0] * beta.powi(2)
            + REGION_4_SATURATION_COEFFS[3] * beta
            + REGION_4_SATURATION_COEFFS[6];
        let coef_g: f64 = REGION_4_SATURATION_COEFFS[1] * beta.powi(2)
            + REGION_4_SATURATION_COEFFS[4] * beta
            + REGION_4_SATURATION_COEFFS[7];
        let coef_d: f64 =
            2.0 * coef_g / (-coef_f - (coef_f.powi(2) - 4.0 * coef_e * coef_g).sqrt());

        // Return t_sat
        (REGION_4_SATURATION_COEFFS[9] + coef_d
            - ((REGION_4_SATURATION_COEFFS[9] + coef_d).powi(2)
                - 4.0 * (REGION_4_SATURATION_COEFFS[8] + REGION_4_SATURATION_COEFFS[9] * coef_d))
                .sqrt())
            * 0.5
    }

    /// Calculates the water Temperature in K at a given
    /// pressure and entropy.
    ///
    /// Pressure is assumed to be in Pa
    /// Entropy is assumed to be in J/kg.K
    ///
    /// Example
    ///
    /// ```rust
    /// use rust_steam::iapws97::{temperature_ps};
    /// let t = temperature_ps(0.1e6, 7.5).unwrap();
    /// ```
    pub fn temperature_ps(p: f64, s: f64) -> Result<f64, IAPWSError> {
        let t1 = t_ps_1(p, s);
        let t2 = t_ps_2(p, s);
        let region1 = region(t1, p);
        let region2 = region(t2, p);

        match (region1, region2) {
            (Ok(Region::Region1), _) => Ok(t1),
            (_, Ok(Region::Region2)) => Ok(t2),
            _ => Err(IAPWSError::NotImplemented()),
        }
    }

    /// Calculates the water Temperature in K at a given
    /// pressure and enthalpy.
    ///
    /// Pressure is assumed to be in Pa
    /// Enthalpy is assumed to be in kJ/kg
    ///
    /// Example
    ///
    /// ```rust
    /// use rust_steam::iapws97::{temperature_ph};
    /// let t = temperature_ph(0.001e6, 3000.0).unwrap();
    /// ```
    pub fn temperature_ph(p: f64, h: f64) -> Result<f64, IAPWSError> {
        let t1 = t_ph_1(p, h);
        let t2 = t_ph_2(p, h);

        let region1 = region(t1, p);
        let region2 = region(t2, p);

        match (region1, region2) {
            (Ok(Region::Region1), _) => Ok(t1),
            (_, Ok(Region::Region2)) => Ok(t2),
            _ => Err(IAPWSError::NotImplemented()),
        }
    }
}
