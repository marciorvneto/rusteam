// These tests represent verify the codes ability to identify these are in region 1 and
// ability to calculate all conditions within the IAPWS97 set. All pull requests must
// pass all tests. If a test is failed, an explanation as to why the test was entered
// implemented incorrectly must be provided.

// Region 1
#[cfg(test)]
mod region1_tests {
    extern crate float_cmp;
    use float_cmp::ApproxEq;
    use rust_steam::iapws97::{
        cpmass_tp, cvmass_tp, hmass_tp, smass_tp, speed_sound_tp, temperature_ph, temperature_ps,
        umass_tp, vmass_tp,
    };

    struct TestData {
        temperature: f64,
        pressure: f64,
        divisor: f64,
        expected_result: f64,
    }
    struct BackTestData {
        pressure: f64,
        value: f64,
        divisor: f64,
        expected_result: f64,
    }

    #[test]
    fn volume_temperature_pressure() {
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
        ];

        for test_data in test_set.iter() {
            let volume =
                vmass_tp(test_data.temperature, test_data.pressure).unwrap() / test_data.divisor;
            assert!(
                volume.approx_eq(test_data.expected_result, (1e-9, 2)),
                "Expected: {} Result: {}",
                test_data.expected_result,
                volume
            );
        }
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
        ];

        for test_data in test_set.iter() {
            let enthalpy =
                hmass_tp(test_data.temperature, test_data.pressure).unwrap() / test_data.divisor;
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
        ];
        for test_data in test_set.iter() {
            let internal_energy =
                umass_tp(test_data.temperature, test_data.pressure).unwrap() / test_data.divisor;
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
        ];
        for test_data in test_set.iter() {
            let entropy =
                smass_tp(test_data.temperature, test_data.pressure).unwrap() / test_data.divisor;
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
        ];

        for test_data in test_set.iter() {
            let isobaricheat =
                cpmass_tp(test_data.temperature, test_data.pressure).unwrap() / test_data.divisor;
            assert!(
                isobaricheat.approx_eq(test_data.expected_result, (1e-9, 2)),
                "Expected: {} Result: {}",
                test_data.expected_result,
                isobaricheat
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
        ];
        for test_data in test_set.iter() {
            let speed_sound = speed_sound_tp(test_data.temperature, test_data.pressure).unwrap()
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
        ];

        for test_data in test_set.iter() {
            let isochoricheat =
                cvmass_tp(test_data.temperature, test_data.pressure).unwrap() / test_data.divisor;
            assert!(
                isochoricheat.approx_eq(test_data.expected_result, (1e-9, 2)),
                "Expected: {} Result: {}",
                test_data.expected_result,
                isochoricheat
            );
        }
    }

    #[test]
    fn backwards_t_ph() {
        let test_set = [
            BackTestData {
                pressure: 3e6,
                value: 500.0,
                divisor: 1e3,
                expected_result: 0.391798509,
            },
            BackTestData {
                pressure: 80e6,
                value: 500.0,
                divisor: 1e3,
                expected_result: 0.378108626,
            },
            BackTestData {
                pressure: 80e6,
                value: 1500.0,
                divisor: 1e3,
                expected_result: 0.611041229,
            },
        ];

        for test_data in test_set.iter() {
            let temperature =
                temperature_ph(test_data.pressure, test_data.value).unwrap() / test_data.divisor;
            assert!(
                temperature.approx_eq(test_data.expected_result, (1e-9, 2)),
                "Expected: {} Result: {}",
                test_data.expected_result,
                temperature
            );
        }
    }

    #[test]
    fn backwards_t_ps() {
        let test_set = [
            BackTestData {
                pressure: 3e6,
                value: 0.5,
                divisor: 1e3,
                expected_result: 0.307842258,
            },
            BackTestData {
                pressure: 80e6,
                value: 0.5,
                divisor: 1e3,
                expected_result: 0.309979785,
            },
            BackTestData {
                pressure: 80e6,
                value: 3.0,
                divisor: 1e3,
                expected_result: 0.565899909,
            },
        ];

        for test_data in test_set.iter() {
            let temperature =
                temperature_ps(test_data.pressure, test_data.value).unwrap() / test_data.divisor;
            assert!(
                temperature.approx_eq(test_data.expected_result, (1e-9, 2)),
                "Expected: {} Result: {}",
                test_data.expected_result,
                temperature
            );
        }
    }
}
