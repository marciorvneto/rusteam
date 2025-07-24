// These tests represent verify the codes ability to identify these are in region 5 and
// ability to calculate all conditions within the IAPWS97 set. All pull requests must
// pass all tests. If a test is failed, an explanation as to why the test was entered
// implemented incorrectly must be provided.

// Region 5
#[cfg(test)]
mod region5_tests {
    // use super::*;
    extern crate float_cmp;
    use float_cmp::ApproxEq;
    use rust_steam::iapws97::{
        cpmass_tp, cvmass_tp, hmass_tp, smass_tp, speed_sound_tp, umass_tp, vmass_tp,
    };

    struct TestData {
        temperature: f64,
        pressure: f64,
        divisor: f64,
        expected_result: f64,
    }

    #[test]
    fn volume_temperature_pressure() {
        let test_set = [
            TestData {
                temperature: 1500.0,
                pressure: 0.5e6,
                divisor: 10.0,
                expected_result: 0.138455090,
            },
            TestData {
                temperature: 1500.0,
                pressure: 30e6,
                divisor: 0.1,
                expected_result: 0.230761299,
            },
            TestData {
                temperature: 2000.0,
                pressure: 30e6,
                divisor: 0.1,
                expected_result: 0.311385219,
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
                temperature: 1500.0,
                pressure: 0.5e6,
                divisor: 10000.0,
                expected_result: 0.521976855,
            },
            TestData {
                temperature: 1500.0,
                pressure: 30e6,
                divisor: 10000.0,
                expected_result: 0.516723514,
            },
            TestData {
                temperature: 2000.0,
                pressure: 30e6,
                divisor: 10000.0,
                expected_result: 0.657122604,
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
                temperature: 1500.0,
                pressure: 0.5e6,
                divisor: 10000.0,
                expected_result: 0.452749310,
            },
            TestData {
                temperature: 1500.0,
                pressure: 30e6,
                divisor: 10000.0,
                expected_result: 0.447495124,
            },
            TestData {
                temperature: 2000.0,
                pressure: 30e6,
                divisor: 10000.0,
                expected_result: 0.563707038,
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
                temperature: 1500.0,
                pressure: 0.5e6,
                divisor: 10.0,
                expected_result: 0.965408875,
            },
            TestData {
                temperature: 1500.0,
                pressure: 30e6,
                divisor: 10.0,
                expected_result: 0.772970133,
            },
            TestData {
                temperature: 2000.0,
                pressure: 30e6,
                divisor: 10.0,
                expected_result: 0.853640523,
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
                temperature: 1500.0,
                pressure: 0.5e6,
                divisor: 10.0,
                expected_result: 0.261609445,
            },
            TestData {
                temperature: 1500.0,
                pressure: 30e6,
                divisor: 10.0,
                expected_result: 0.272724317,
            },
            TestData {
                temperature: 2000.0,
                pressure: 30e6,
                divisor: 10.0,
                expected_result: 0.288569882,
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
                temperature: 1500.0,
                pressure: 0.5e6,
                divisor: 1000.0,
                expected_result: 0.917068690,
            },
            TestData {
                temperature: 1500.0,
                pressure: 30e6,
                divisor: 1000.0,
                expected_result: 0.928548002,
            },
            TestData {
                temperature: 2000.0,
                pressure: 30e6,
                divisor: 10000.0,
                expected_result: 0.106736948,
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
                temperature: 1500.0,
                pressure: 0.5e6,
                divisor: 10.0,
                expected_result: 0.215338,
            },
            TestData {
                temperature: 1500.0,
                pressure: 30e6,
                divisor: 10.0,
                expected_result: 0.219275,
            },
            TestData {
                temperature: 2000.0,
                pressure: 30e6,
                divisor: 10.0,
                expected_result: 0.239589,
            },
        ];

        for test_data in test_set.iter() {
            let isochoricheat =
                cvmass_tp(test_data.temperature, test_data.pressure).unwrap() / test_data.divisor;
            assert!(
                isochoricheat.approx_eq(test_data.expected_result, (1e-6, 2)),
                "Expected: {} Result: {}",
                test_data.expected_result,
                isochoricheat
            );
        }
    }
}
