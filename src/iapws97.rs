pub mod iapws97 {

    mod constants;
    mod region_1;
    mod region_2;
    mod region_3;
    mod region_5;
    use crate::iapws97::region_1::{cp_tp_1, cv_tp_1, h_tp_1, s_tp_1, u_tp_1, v_tp_1, w_tp_1};
    use crate::iapws97::region_2::{cp_tp_2, cv_tp_2, h_tp_2, s_tp_2, u_tp_2, v_tp_2, w_tp_2};
    use crate::iapws97::region_3::{cp_tp_3, cv_tp_3, h_tp_3, s_tp_3, u_tp_3, v_tp_3};
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
        n[3] + (((p*1e-6)-n[4])/n[2]).sqrt()
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

    /// Calculates the water entropy in J/kg/K at a given
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
            Region::Region3 => Ok(v_tp_3(t, p).unwrap()),
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

    #[cfg(test)]
    mod tests {

        use crate::iapws97::t_boundary_2_3;

        use super::{
            cpmass_tp, cvmass_tp, hmass_tp, p_boundary_2_3, psat97, smass_tp, speed_sound_tp,
            tsat97, umass_tp, vmass_tp,
        };
        extern crate float_cmp;
        use float_cmp::ApproxEq;

        struct TestData {
            temperature: f64,
            pressure: f64,
            divisor: f64,
            expected_result: f64,
        }

        #[test]
        fn enthalpy_temperature_pressure() {
            let test_set = [
                TestData {
                    temperature: 300.0,
                    pressure: 3e6,
                    divisor: 1000.0,
                    expected_result: 0.115331273,
                },
                TestData {
                    temperature: 300.0,
                    pressure: 80e6,
                    divisor: 1000.0,
                    expected_result: 0.184142828,
                },
                TestData {
                    temperature: 500.0,
                    pressure: 3e6,
                    divisor: 1000.0,
                    expected_result: 0.975542239,
                },
                TestData {
                    temperature: 300.0,
                    pressure: 0.0035e6,
                    divisor: 10000.0,
                    expected_result: 0.254991145,
                },
                TestData {
                    temperature: 700.0,
                    pressure: 0.0035e6,
                    divisor: 10000.0,
                    expected_result: 0.333568375,
                },
                TestData {
                    temperature: 700.0,
                    pressure: 30e6,
                    divisor: 10000.0,
                    expected_result: 0.263149474,
                },
            ];

            for test_data in test_set.iter() {
                let enthalpy = hmass_tp(test_data.temperature, test_data.pressure).unwrap()
                    / test_data.divisor;
                assert!(enthalpy.approx_eq(test_data.expected_result, (1e-9, 2)));
            }
        }

        #[test]
        fn internal_energy_temperature_pressure() {
            let test_set = [
                TestData {
                    temperature: 300.0,
                    pressure: 3e6,
                    divisor: 1000.0,
                    expected_result: 0.112324818,
                },
                TestData {
                    temperature: 300.0,
                    pressure: 80e6,
                    divisor: 1000.0,
                    expected_result: 0.106448356,
                },
                TestData {
                    temperature: 500.0,
                    pressure: 3e6,
                    divisor: 1000.0,
                    expected_result: 0.971934985,
                },
                TestData {
                    temperature: 300.0,
                    pressure: 0.0035e6,
                    divisor: 10000.0,
                    expected_result: 0.241169160,
                },
                TestData {
                    temperature: 700.0,
                    pressure: 0.0035e6,
                    divisor: 10000.0,
                    expected_result: 0.301262819,
                },
                TestData {
                    temperature: 700.0,
                    pressure: 30e6,
                    divisor: 10000.0,
                    expected_result: 0.246861076,
                },
            ];
            for test_data in test_set.iter() {
                let internal_energy = umass_tp(test_data.temperature, test_data.pressure).unwrap()
                    / test_data.divisor;
                assert!(internal_energy.approx_eq(test_data.expected_result, (1e-9, 2)));
            }
        }

        #[test]
        fn entropy_temperature_pressure() {
            let test_set = [
                TestData {
                    temperature: 300.0,
                    pressure: 3e6,
                    divisor: 1.0,
                    expected_result: 0.392294792,
                },
                TestData {
                    temperature: 300.0,
                    pressure: 80e6,
                    divisor: 1.0,
                    expected_result: 0.368563852,
                },
                TestData {
                    temperature: 500.0,
                    pressure: 3e6,
                    divisor: 1.0,
                    expected_result: 0.258041912e1,
                },
                TestData {
                    temperature: 300.0,
                    pressure: 0.0035e6,
                    divisor: 10.0,
                    expected_result: 0.852238967,
                },
                TestData {
                    temperature: 700.0,
                    pressure: 0.0035e6,
                    divisor: 100.0,
                    expected_result: 0.101749996,
                },
                TestData {
                    temperature: 700.0,
                    pressure: 30e6,
                    divisor: 10.0,
                    expected_result: 0.517540298,
                },
            ];
            for test_data in test_set.iter() {
                let entropy = smass_tp(test_data.temperature, test_data.pressure).unwrap()
                    / test_data.divisor;
                assert!(
                    entropy.approx_eq(test_data.expected_result, (1e-9, 2)),
                    "Expected: {} Result: {}",
                    test_data.expected_result,
                    entropy
                );
            }
        }

        #[test]
        fn isobaric_heat_pressure_temperature() {
            let test_set = [
                TestData {
                    temperature: 300.0,
                    pressure: 3e6,
                    divisor: 10.0,
                    expected_result: 0.417301218,
                },
                TestData {
                    temperature: 300.0,
                    pressure: 80e6,
                    divisor: 10.0,
                    expected_result: 0.401008987,
                },
                TestData {
                    temperature: 500.0,
                    pressure: 3e6,
                    divisor: 10.0,
                    expected_result: 0.465580682,
                },
                TestData {
                    temperature: 300.0,
                    pressure: 0.0035e6,
                    divisor: 10.0,
                    expected_result: 0.191300162,
                },
                TestData {
                    temperature: 700.0,
                    pressure: 0.0035e6,
                    divisor: 10.0,
                    expected_result: 0.208141274,
                },
                TestData {
                    temperature: 700.0,
                    pressure: 30e6,
                    divisor: 100.0,
                    expected_result: 0.103505092,
                },
            ];

            for test_data in test_set.iter() {
                let isobaricheat = cpmass_tp(test_data.temperature, test_data.pressure).unwrap()
                    / test_data.divisor;
                assert!(
                    isobaricheat.approx_eq(test_data.expected_result, (1e-9, 2)),
                    "Expected: {} Result: {}",
                    test_data.expected_result,
                    isobaricheat
                );
            }
        }

        #[test]
        fn isochoric_heat_pressure_temperature() {
            let test_set = [
                TestData {
                    temperature: 300.0,
                    pressure: 3e6,
                    divisor: 1.0,
                    expected_result: 4.121201603,
                },
                TestData {
                    temperature: 300.0,
                    pressure: 80e6,
                    divisor: 1.0,
                    expected_result: 3.917366061,
                },
                TestData {
                    temperature: 500.0,
                    pressure: 3e6,
                    divisor: 1.0,
                    expected_result: 3.221392229,
                },
                TestData {
                    temperature: 300.0,
                    pressure: 0.0035e6,
                    divisor: 10.0,
                    expected_result: 0.1441326618,
                },
                TestData {
                    temperature: 700.0,
                    pressure: 0.0035e6,
                    divisor: 10.0,
                    expected_result: 0.1619783325,
                },
                TestData {
                    temperature: 700.0,
                    pressure: 30e6,
                    divisor: 100.0,
                    expected_result: 0.0297553836,
                },
            ];

            for test_data in test_set.iter() {
                let isochoricheat = cvmass_tp(test_data.temperature, test_data.pressure).unwrap()
                    / test_data.divisor;
                assert!(
                    isochoricheat.approx_eq(test_data.expected_result, (1e-9, 2)),
                    "Expected: {} Result: {}",
                    test_data.expected_result,
                    isochoricheat
                );
            }
        }

        #[test]
        fn volume_heat_pressure() {
            let test_set = [
                TestData {
                    temperature: 300.0,
                    pressure: 3e6,
                    divisor: 0.01,
                    expected_result: 0.100215168,
                },
                TestData {
                    temperature: 300.0,
                    pressure: 80e6,
                    divisor: 0.001,
                    expected_result: 0.971180894,
                },
                TestData {
                    temperature: 500.0,
                    pressure: 3e6,
                    divisor: 0.01,
                    expected_result: 0.120241800,
                },
                TestData {
                    temperature: 300.0,
                    pressure: 0.0035e6,
                    divisor: 100.0,
                    expected_result: 0.394913866,
                },
                TestData {
                    temperature: 700.0,
                    pressure: 0.0035e6,
                    divisor: 100.0,
                    expected_result: 0.923015898,
                },
                TestData {
                    temperature: 700.0,
                    pressure: 30e6,
                    divisor: 0.01,
                    expected_result: 0.542946619,
                },
            ];

            for test_data in test_set.iter() {
                let volume = vmass_tp(test_data.temperature, test_data.pressure).unwrap()
                    / test_data.divisor;
                assert!(
                    volume.approx_eq(test_data.expected_result, (1e-9, 2)),
                    "Expected: {} Result: {}",
                    test_data.expected_result,
                    volume
                );
            }
        }

        #[test]
        fn speed_sound_temperature_pressure() {
            let test_set = [
                TestData {
                    temperature: 300.0,
                    pressure: 3e6,
                    divisor: 10000.0,
                    expected_result: 0.150773921,
                },
                TestData {
                    temperature: 300.0,
                    pressure: 80e6,
                    divisor: 10000.0,
                    expected_result: 0.163469054,
                },
                TestData {
                    temperature: 500.0,
                    pressure: 3e6,
                    divisor: 10000.0,
                    expected_result: 0.124071337,
                },
                TestData {
                    temperature: 300.0,
                    pressure: 0.0035e6,
                    divisor: 1000.0,
                    expected_result: 0.427920172,
                },
                TestData {
                    temperature: 700.0,
                    pressure: 0.0035e6,
                    divisor: 1000.0,
                    expected_result: 0.644289068,
                },
                TestData {
                    temperature: 700.0,
                    pressure: 30e6,
                    divisor: 1000.0,
                    expected_result: 0.480386523,
                },
            ];
            for test_data in test_set.iter() {
                let speed_sound = speed_sound_tp(test_data.temperature, test_data.pressure)
                    .unwrap()
                    / test_data.divisor;
                assert!(
                    speed_sound.approx_eq(test_data.expected_result, (1e-9, 2)),
                    "Expected: {} Result: {}",
                    test_data.expected_result,
                    speed_sound
                );
            }
        }

        #[test]
        fn saturation_pressure() {
            let ps = psat97(&300.0) / 1e4;
            assert!(ps.approx_eq(0.353658941, (1e-9, 2)));

            let ps = psat97(&500.0) / 1e7;
            assert!(ps.approx_eq(0.263889776, (1e-9, 2)));

            let ps = psat97(&600.0) / 1e8;
            assert!(ps.approx_eq(0.123443146, (1e-9, 2)));
        }

        #[test]
        fn saturation_temperature() {
            let ts = tsat97(&0.1e6) / 1e3;
            assert!(ts.approx_eq(0.372755919, (1e-9, 2)));

            let ts = tsat97(&1e6) / 1e3;
            assert!(ts.approx_eq(0.453035632, (1e-9, 2)));

            let ts = tsat97(&10e6) / 1e3;
            assert!(ts.approx_eq(0.584149488, (1e-9, 2)));
        }

        #[test]
        fn region_2_3_auxiliary_boundary() {
            let p = p_boundary_2_3(&623.15) * 1e-8;
            assert!(p.approx_eq(0.165291643, (1e-9, 2)));


            let t = t_boundary_2_3(&16.5291643e6) / 1e3;
            assert!(t.approx_eq(0.623150000, (1e-9, 2)));
        }
    }
}
