// These tests represent verify the codes ability to identify these are in region 4 and
// ability to calculate all conditions within the IAPWS97 set. All pull requests must
// pass all tests. If a test is failed, an explanation as to why the test was entered
// implemented incorrectly must be provided.

// Region 4
#[cfg(test)]
mod region4_tests {
    // use super::*;
    extern crate float_cmp;
    use float_cmp::ApproxEq;
    use rust_steam::iapws97::{psat97, tsat97};

    struct TestData {
        value: f64,
        divisor: f64,
        expected_result: f64,
    }

    #[test]
    fn saturation_pressure() {
        let test_set = [
            TestData {
                value: 300.0,
                divisor: 1e4,
                expected_result: 0.353658941,
            },
            TestData {
                value: 500.0,
                divisor: 1e7,
                expected_result: 0.263889776,
            },
            TestData {
                value: 600.0,
                divisor: 1e8,
                expected_result: 0.123443146,
            },
        ];

        for test_data in test_set.iter() {
            let volume = psat97(&test_data.value) / test_data.divisor;
            assert!(
                volume.approx_eq(test_data.expected_result, (1e-9, 2)),
                "Expected: {} Result: {}",
                test_data.expected_result,
                volume
            );
        }
    }

    #[test]
    fn saturation_temperature() {
        let test_set = [
            TestData {
                value: 0.1e6,
                divisor: 1e3,
                expected_result: 0.372755919,
            },
            TestData {
                value: 1e6,
                divisor: 1e3,
                expected_result: 0.453035632,
            },
            TestData {
                value: 10e6,
                divisor: 1e3,
                expected_result: 0.584149488,
            },
        ];

        for test_data in test_set.iter() {
            let volume = tsat97(&test_data.value) / test_data.divisor;
            assert!(
                volume.approx_eq(test_data.expected_result, (1e-9, 2)),
                "Expected: {} Result: {}",
                test_data.expected_result,
                volume
            );
        }
    }
}
