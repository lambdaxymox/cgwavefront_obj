use lexer::Lexer;
use std::slice;


struct TestCase {
    data: String,
    expected: Vec<String>,
}

struct Test {
    test_cases: Vec<TestCase>,
}

impl Test {
    fn iter(&self) -> TestIter {
        TestIter {
            inner: self.test_cases.iter(),
        }
    }
}

struct TestIter<'a> {
    inner: slice::Iter<'a, TestCase>,
}

impl<'a> Iterator for TestIter<'a> {
    type Item = &'a TestCase;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}


fn test_cases() -> Test {
    Test {
        test_cases: vec![
            TestCase {
                data: String::from(r"                              \
                    v -2.300000 1.950000 0.000000                  \
                    v -2.200000 0.790000 0.000000                  \
                    v -2.340000 -1.510000 0.000000                 \
                    v -1.530000 -1.490000 0.000000                 \
                    v -0.720000 -1.470000 0.000000                 \
                    v -0.780000 0.230000 0.000000                  \
                    v 0.070000 0.250000 0.000000                   \
                    v 0.920000 0.270000 0.000000                   \
                    v 0.800000 -1.610000 0.000000                  \
                    v 1.620000 -1.590000 0.000000                  \
                    v 2.440000 -1.570000 0.000000                  \
                    v 2.690000 0.670000 0.000000                   \
                    v 2.900000 1.980000 0.000000                   \
                    # 13 vertices                                  \
                                                                   \
                                                                   \
                    cstype bezier                                  \
                    ctech cparm 1.000000                           \
                    deg 3                                          \
                    curv 0.000000 4.000000 1 2 3 4 5 6 7 8 9 10 \\ \
                    11 12 13                                       \
                    parm u 0.000000 1.000000 2.000000 3.000000  \\ \
                    4.000000                                       \
                    end                                            \
                    # 1 element                                    \
                    "
                ),
                expected: vec![
                    "v", "-2.300000", "1.950000", "0.000000",
                    "v", "-2.200000", "0.790000", "0.000000",
                    "v", "-2.340000", "-1.510000", "0.000000",
                    "v", "-1.530000", "-1.490000", "0.000000",
                    "v", "-0.720000", "-1.470000", "0.000000",
                    "v", "-0.780000", "0.230000", "0.000000",
                    "v", "0.070000", "0.250000", "0.000000",
                    "v", "0.920000", "0.270000", "0.000000",
                    "v", "0.800000", "-1.610000", "0.000000",
                    "v", "1.620000", "-1.590000", "0.000000",
                    "v", "2.440000", "-1.570000", "0.000000",
                    "v", "2.690000", "0.670000", "0.000000",
                    "v", "2.900000", "1.980000", "0.000000",
                    "cstype", "bezier",
                    "ctech", "cparm", "1.000000",
                    "deg", "3",
                    "curv", "0.000000", "4.000000", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10",
                    "11", "12", "13",
                    "parm", "u", "0.000000", "1.000000", "2.000000", "3.000000",
                    "4.000000",
                    "end"
                ].iter().map(|&str| String::from(str)).collect(),
            },
        ]
    }
}

#[test]
fn test_lexer1() {
    for test_case in test_cases().iter() {
        let lexer = Lexer::new(&test_case.data);
        let result = lexer.collect::<Vec<String>>();
        assert_eq!(result, test_case.expected);
    }
}

