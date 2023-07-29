pub mod iapws97 {

    mod constants;
    mod region_1;
    mod region_2;
    mod region_3;
    mod region_4;
    use crate::iapws97::region_1::{cp_tp_1, cv_tp_1, h_tp_1, s_tp_1, u_tp_1, v_tp_1, w_tp_1};
    use crate::iapws97::region_2::{cp_tp_2, cv_tp_2, h_tp_2, s_tp_2, u_tp_2, v_tp_2, w_tp_2};
    #[allow(unused_imports)]
    use crate::iapws97::region_4::{psat97, tsat97};

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

    fn p_boundary_2_3(t: f64) -> f64 {
        let p_star = 1e6;
        let theta = t / 1.0;
        let n1 = 0.34805185628969e3;
        let n2 = -0.11671859879975e1;
        let n3 = 0.10192970039326e-2;
        p_star * (n1 + n2 * theta + n3 * theta * theta)
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
        if !(273.15..=2273.15).contains(&t) {
            return Err(IAPWSError::OutOfBounds(t, p));
        }
        if !(0.0..=100e6).contains(&p) {
            return Err(IAPWSError::OutOfBounds(t, p));
        }

        if t <= 623.15 {
            if p > psat97(t) {
                return Ok(Region::Region1);
            } else if p < psat97(t) {
                return Ok(Region::Region2);
            }
            return Ok(Region::Region4);
        }

        if t <= 863.15 {
            if p > p_boundary_2_3(t) {
                return Ok(Region::Region3);
            } else if p < p_boundary_2_3(t) {
                return Ok(Region::Region2);
            }
            return Ok(Region::Region4);
        }

        if t < 1073.15 {
            return Ok(Region::Region2);
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
    /// use rust_steam::iapws97::{hmass_tp};
    /// let h = hmass_tp(300.0, 101325.0).unwrap();
    /// ```
    pub fn hmass_tp(t: f64, p: f64) -> Result<f64, IAPWSError> {
        let region = region(t, p)?;
        match region {
            Region::Region1 => Ok(h_tp_1(t, p)),
            Region::Region2 => Ok(h_tp_2(t, p)),
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
    /// use rust_steam::iapws97::{speed_sound_tp};
    /// let w = speed_sound_tp(300.0, 101325.0).unwrap();
    /// ```
    pub fn speed_sound_tp(t: f64, p: f64) -> Result<f64, IAPWSError> {
        let region = region(t, p)?;
        match region {
            Region::Region1 => Ok(w_tp_1(t, p)),
            Region::Region2 => Ok(w_tp_2(t, p)),
            _ => Err(IAPWSError::NotImplemented()),
        }
    }

    #[cfg(test)]
    mod tests {

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
            let test_set = vec![
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
            let test_set = vec![
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
            let test_set = vec![
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
            let test_set = vec![
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
            let test_set = vec![
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
            let test_set = vec![
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
            let test_set = vec![
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
            let ps = psat97(300.0) / 10000.0;
            assert!(ps.approx_eq(0.353658941, (1e-9, 2)));

            let ps = psat97(500.0) / 1e7;
            assert!(ps.approx_eq(0.263889776, (1e-9, 2)));

            let ps = psat97(600.0) / 1e8;
            assert!(ps.approx_eq(0.123443146, (1e-9, 2)));
        }

        #[test]
        fn saturation_temperature() {
            let ts = tsat97(0.1e6) / 1000.0;
            assert!(ts.approx_eq(0.372755919, (1e-9, 2)));

            let ts = tsat97(1e6) / 1000.0;
            assert!(ts.approx_eq(0.453035632, (1e-9, 2)));

            let ts = tsat97(10e6) / 1000.0;
            assert!(ts.approx_eq(0.584149488, (1e-9, 2)));
        }

        #[test]
        fn region_2_3_auxiliary_boundary() {
            let p = p_boundary_2_3(623.15) / 1e8;
            assert!(p.approx_eq(0.165291643, (1e-9, 2)));
        }
    }
}
