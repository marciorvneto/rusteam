// These tests represent verify the codes ability to identify these are in region 2 and
// ability to calculate all conditions within the IAPWS97 set. All pull requests must
// pass all tests. If a test is failed, an explanation as to why the test was entered
// implemented incorrectly must be provided.

// Region 2
#[cfg(test)]
mod region2_tests {
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

    // Extra test based on calculations from IAPWS Online Calculator
    #[test]
    fn isochoric_heat_pressure_temperature() {
        let test_set = [
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
                pressure: 0.001e6,
                value: 3000.0,
                divisor: 1e3,
                expected_result: 0.534433241,
            },
            BackTestData {
                pressure: 3e6,
                value: 3000.0,
                divisor: 1e3,
                expected_result: 0.575373370,
            },
            BackTestData {
                pressure: 3e6,
                value: 4000.0,
                divisor: 1e4,
                expected_result: 0.101077577,
            },
            BackTestData {
                pressure: 5e6,
                value: 3500.0,
                divisor: 1e3,
                expected_result: 0.801299102,
            },
            BackTestData {
                pressure: 5e6,
                value: 4000.0,
                divisor: 1e4,
                expected_result: 0.101531583,
            },
            BackTestData {
                pressure: 25e6,
                value: 3500.0,
                divisor: 1e3,
                expected_result: 0.875279054,
            },
            BackTestData {
                pressure: 40e6,
                value: 2700.0,
                divisor: 1e3,
                expected_result: 0.743056411,
            },
            BackTestData {
                pressure: 60e6,
                value: 2700.0,
                divisor: 1e3,
                expected_result: 0.791137067,
            },
            BackTestData {
                pressure: 60e6,
                value: 3200.0,
                divisor: 1e3,
                expected_result: 0.882756860,
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
                pressure: 0.1e6,
                value: 7.5,
                divisor: 1e3,
                expected_result: 0.399517097,
            },
            BackTestData {
                pressure: 0.1e6,
                value: 8.0,
                divisor: 1e3,
                expected_result: 0.514127081,
            },
            BackTestData {
                pressure: 2.5e6,
                value: 8.0,
                divisor: 1e4,
                expected_result: 0.103984917,
            },
            BackTestData {
                pressure: 8e6,
                value: 6.0,
                divisor: 1e3,
                expected_result: 0.600484040,
            },
            BackTestData {
                pressure: 8e6,
                value: 7.5,
                divisor: 1e4,
                expected_result: 0.106495556,
            },
            BackTestData {
                pressure: 90e6,
                value: 6.0,
                divisor: 1e4,
                expected_result: 0.103801126,
            },
            BackTestData {
                pressure: 20e6,
                value: 5.75,
                divisor: 1e3,
                expected_result: 0.697992849,
            },
            BackTestData {
                pressure: 80e6,
                value: 5.25,
                divisor: 1e3,
                expected_result: 0.854011484,
            },
            BackTestData {
                pressure: 80e6,
                value: 5.75,
                divisor: 1e3,
                expected_result: 0.949017998,
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
