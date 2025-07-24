/// These tests represent verify the codes ability to identify these are in region 3 and
/// ability to calculate all conditions within the IAPWS97 set. All pull requests must
/// pass all tests. If a test is failed, an explanation as to why the test was entered
/// implemented incorrectly must be provided.

/// Region 3 has more lax equality checks since IAPWS assumes Temperature (K) and
/// Density (kg/m^3) to be provided. Since this code assumes Temperature (K) and
/// Pressure (Pa) are provided, an intermediate cacluation is performed to calculate
/// Density (kg/m^3) for use in the formulas.

// Region 3
#[cfg(test)]
mod region3_tests {
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
                temperature: 650.0,
                pressure: 25.5837018e6,
                divisor: 0.01,
                expected_result: 0.2,
            },
            TestData {
                temperature: 650.0,
                pressure: 22.2930643e6,
                divisor: 0.01,
                expected_result: 0.5,
            },
            TestData {
                temperature: 750.0,
                pressure: 78.3095639e6,
                divisor: 0.01,
                expected_result: 0.2,
            },
        ];

        for test_data in test_set.iter() {
            let volume =
                vmass_tp(test_data.temperature, test_data.pressure).unwrap() / test_data.divisor;
            assert!(
                volume.approx_eq(test_data.expected_result, (1e-6, 2)),
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
                temperature: 650.0,
                pressure: 25.5837018e6,
                divisor: 10000.0,
                expected_result: 0.186343019,
            },
            TestData {
                temperature: 650.0,
                pressure: 22.2930643e6,
                divisor: 10000.0,
                expected_result: 0.237512401,
            },
            TestData {
                temperature: 750.0,
                pressure: 78.3095639e6,
                divisor: 10000.0,
                expected_result: 0.225868845,
            },
        ];

        for test_data in test_set.iter() {
            let enthalpy =
                hmass_tp(test_data.temperature, test_data.pressure).unwrap() / test_data.divisor;
            assert!(enthalpy.approx_eq(test_data.expected_result, (1e-6, 2)));
        }
    }

    #[test]
    fn internal_energy_temperature_pressure() {
        let test_set = [
            TestData {
                temperature: 650.0,
                pressure: 25.5837018e6,
                divisor: 10000.0,
                expected_result: 0.181226279,
            },
            TestData {
                temperature: 650.0,
                pressure: 22.2930643e6,
                divisor: 10000.0,
                expected_result: 0.226365868,
            },
            TestData {
                temperature: 750.0,
                pressure: 78.3095639e6,
                divisor: 10000.0,
                expected_result: 0.210206932,
            },
        ];
        for test_data in test_set.iter() {
            let internal_energy =
                umass_tp(test_data.temperature, test_data.pressure).unwrap() / test_data.divisor;
            assert!(internal_energy.approx_eq(test_data.expected_result, (1e-6, 2)));
        }
    }

    #[test]
    fn entropy_temperature_pressure() {
        let test_set = [
            TestData {
                temperature: 650.0,
                pressure: 25.5837018e6,
                divisor: 10.0,
                expected_result: 0.405427273,
            },
            TestData {
                temperature: 650.0,
                pressure: 22.2930643e6,
                divisor: 10.0,
                expected_result: 0.485438792,
            },
            TestData {
                temperature: 750.0,
                pressure: 78.3095639e6,
                divisor: 10.0,
                expected_result: 0.446971906,
            },
        ];
        for test_data in test_set.iter() {
            let entropy =
                smass_tp(test_data.temperature, test_data.pressure).unwrap() / test_data.divisor;
            assert!(
                entropy.approx_eq(test_data.expected_result, (1e-6, 2)),
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
                temperature: 650.0,
                pressure: 25.5837018e6,
                divisor: 100.0,
                expected_result: 0.138935717,
            },
            TestData {
                temperature: 650.0,
                pressure: 22.2930643e6,
                divisor: 100.0,
                expected_result: 0.446579342,
            },
            TestData {
                temperature: 750.0,
                pressure: 78.3095639e6,
                divisor: 10.0,
                expected_result: 0.634165359,
            },
        ];

        for test_data in test_set.iter() {
            let isobaricheat =
                cpmass_tp(test_data.temperature, test_data.pressure).unwrap() / test_data.divisor;
            assert!(
                isobaricheat.approx_eq(test_data.expected_result, (1e-5, 2)),
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
                temperature: 650.0,
                pressure: 25.5837018e6,
                divisor: 1000.0,
                expected_result: 0.502005554,
            },
            TestData {
                temperature: 650.0,
                pressure: 22.2930643e6,
                divisor: 1000.0,
                expected_result: 0.383444594,
            },
            TestData {
                temperature: 750.0,
                pressure: 78.3095639e6,
                divisor: 1000.0,
                expected_result: 0.760696041,
            },
        ];
        for test_data in test_set.iter() {
            let speed_sound = speed_sound_tp(test_data.temperature, test_data.pressure).unwrap()
                / test_data.divisor;
            assert!(
                speed_sound.approx_eq(test_data.expected_result, (1e-5, 2)),
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
                temperature: 650.0,
                pressure: 25.5837018e6,
                divisor: 10.0,
                expected_result: 0.319133,
            },
            TestData {
                temperature: 650.0,
                pressure: 22.2930643e6,
                divisor: 10.0,
                expected_result: 0.404118,
            },
            TestData {
                temperature: 750.0,
                pressure: 78.3095639e6,
                divisor: 10.0,
                expected_result: 0.271702,
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
