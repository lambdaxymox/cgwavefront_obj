use std::str;


#[inline]
fn is_whitespace(ch: u8) -> bool {
    ch == b' ' || ch == b'\\' || ch == b'\t'
}

#[inline]
fn is_newline(ch: u8) -> bool {
    ch == b'\n' || ch == b'\r'
}

#[inline]
fn is_whitespace_or_newline(ch: u8) -> bool {
    is_whitespace(ch) || is_newline(ch)
}
/// A OBJ file lexer tokenizes an input character stream.
#[derive(Clone)]
pub struct Lexer<'a> {
    /// The current line position in the token stream.
    current_line_number: usize,
    /// The cursor position in the character stream.
    stream_position: usize,
    /// The input stream.
    stream: &'a [u8],
}

impl<'a> Lexer<'a> {
    /// Create a new lexer.
    pub fn new(stream: &'a str) -> Lexer<'a> {
        Lexer {
            current_line_number: 1,
            stream_position: 0,
            stream: stream.as_bytes(),
        }
    }

    /// The function `peek` looks at the character at the current position
    /// in the byte stream without advancing the stream.
    #[inline]
    fn peek(&mut self) -> Option<&u8> {
        self.stream.get(self.stream_position)
    }

    /// the function `advance` advances the lexer by one
    /// character in the byte stream.
    fn advance(&mut self) {
        match self.peek() {
            Some(&ch) if is_newline(ch) => {
                self.current_line_number += 1;
            },
            _ => {}
        }
        self.stream_position += 1;
    }

    /// Given a boolean predicate that operates on bytes, advance through the 
    /// stream while the predicate is still satisfied.
    /// This function returns the number of characters skipped.
    fn skip_while<P: Fn(u8) -> bool>(&mut self, predicate: P) -> usize {
        let mut skipped = 0;
        loop {
            match self.peek() {
                Some(&ch) if predicate(ch) => {
                    self.advance();
                    skipped += 1;
                }
                Some(_) | None => {
                    break;
                }
            }
        }

        skipped
    }

    /// Given a predicate that operates on bytes, advance through stream while the predicates
    /// is not satisfied. That is, advance one character at a time unless the predicate is 
    /// satisfied, and then stop. This function returns the number of characters skipped.
    fn skip_unless<P: Fn(u8) -> bool>(&mut self, not_predicate: P) -> usize {
        self.skip_while(|ch| !not_predicate(ch))
    }

    /// The function `skip_comment` consumes a comment line
    /// without returning it.
    fn skip_comment(&mut self) -> usize {
        match self.peek() {
            Some(b'#') => self.skip_unless(is_newline),
            _ => 0,
        }
    }

    /// The function `skip_whitespace` consumes a string of whitespace
    /// characters without returning them.
    fn skip_whitespace(&mut self) -> usize {
        self.skip_while(is_whitespace)
    }

    /// This function fetches the next token from the input stream.
    fn next_token(&mut self) -> Option<&'a [u8]> {
        self.skip_whitespace();
        self.skip_comment();

        let start_position = self.stream_position;

        match self.peek() {
            Some(&ch) if is_newline(ch) => {
                self.advance();
                self.stream.get(start_position..self.stream_position)
            }
            Some(_) => {
                let skipped = self.skip_unless(|ch| is_whitespace_or_newline(ch) || ch == b'#');
                if skipped > 0 {
                    self.stream.get(start_position..self.stream_position)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

pub struct ObjectLexer<'a> {
    inner: Lexer<'a>,
    cache: Option<Option<&'a str>>,
}

impl<'a> ObjectLexer<'a> {
    pub fn new(lexer: Lexer<'a>) -> ObjectLexer<'a> {
        ObjectLexer {
            inner: lexer,
            cache: None,
        }
    }

    pub fn next_token(&mut self) -> Option<&'a str> {
        match self.cache.take() {
            Some(token) => token,
            None => {
                self.inner.next_token().map(
                    |t| { unsafe { str::from_utf8_unchecked(t) } 
                })
            }
        }
    }

    pub fn peek(&mut self) -> Option<&'a str> {
        match self.cache {
            Some(token) => token,
            None => {
                let next_token = self.inner.next_token().map(
                    |t| { unsafe { str::from_utf8_unchecked(t) } 
                });
                self.cache.replace(next_token);
                next_token
            }
        }
    }
}

impl<'a> Iterator for ObjectLexer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, ObjectLexer};
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
                        v -2.300000  1.950000  0.000000                \
                        v -2.200000  0.790000  0.000000                \
                        v -2.340000 -1.510000  0.000000                \
                        v -1.530000 -1.490000  0.000000                \
                        v -0.720000 -1.470000  0.000000                \
                        v -0.780000  0.230000  0.000000                \
                        v  0.070000  0.250000  0.000000                \
                        v  0.920000  0.270000  0.000000                \
                        v  0.800000 -1.610000  0.000000                \
                        v  1.620000 -1.590000  0.000000                \
                        v  2.440000 -1.570000  0.000000                \
                        v  2.690000  0.670000  0.000000                \
                        v  2.900000  1.980000  0.000000                \
                        # 13 vertices                                  \
                                                                       \
                        # Multi                                        \
                        # line                                         \
                        # comment                                      \
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
                        "\n",
                        "v", "-2.300000",  "1.950000", "0.000000", "\n",
                        "v", "-2.200000",  "0.790000", "0.000000", "\n",
                        "v", "-2.340000", "-1.510000", "0.000000", "\n",
                        "v", "-1.530000", "-1.490000", "0.000000", "\n",
                        "v", "-0.720000", "-1.470000", "0.000000", "\n",
                        "v", "-0.780000",  "0.230000", "0.000000", "\n",
                        "v",  "0.070000",  "0.250000", "0.000000", "\n",
                        "v",  "0.920000",  "0.270000", "0.000000", "\n",
                        "v",  "0.800000", "-1.610000", "0.000000", "\n",
                        "v",  "1.620000", "-1.590000", "0.000000", "\n",
                        "v",  "2.440000", "-1.570000", "0.000000", "\n",
                        "v",  "2.690000",  "0.670000", "0.000000", "\n",
                        "v",  "2.900000",  "1.980000", "0.000000", "\n", "\n", "\n", "\n", "\n", "\n",
                        "cstype", "bezier", "\n",
                        "ctech", "cparm", "1.000000", "\n",
                        "deg", "3", "\n",
                        "curv", "0.000000", "4.000000", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "\n",
                        "11", "12", "13", "\n",
                        "parm", "u", "0.000000", "1.000000", "2.000000", "3.000000", "\n",
                        "4.000000", "\n",
                        "end", "\n", 
                        "\n",
                    ].iter().map(|&str| String::from(str)).collect(),
                },
                TestCase {
                    data: String::from(r"                           \
                        # diamond.obj                               \
                                                                    \
                        g Object001                                 \
                                                                    \
                        v  0.000000E+00  0.000000E+00  78.0000      \
                        v  45.0000       45.0000       0.000000E+00 \
                        v  45.0000      -45.0000       0.000000E+00 \
                        v -45.0000      -45.0000       0.000000E+00 \
                        v -45.0000       45.0000       0.000000E+00 \
                        v  0.000000E+00  0.000000E+00 -78.0000      \
                                                                    \
                        f     1 2 3                                 \
                        f     1 3 4                                 \
                        f     1 4 5                                 \
                        f     1 5 2                                 \
                        f     6 5 4                                 \
                        f     6 4 3                                 \
                        f     6 3 2                                 \
                        f     6 2 1                                 \
                        f     6 1 5                                 \
                                                                    \
                                                                    \
                                                                    \
                        "),
                    expected: vec![
                        "\n", "\n", "\n",
                        "g", "Object001", "\n", "\n",
                        "v", "0.000000E+00", "0.000000E+00", "78.0000", "\n",
                        "v", "45.0000", "45.0000", "0.000000E+00", "\n",
                        "v", "45.0000", "-45.0000", "0.000000E+00", "\n",
                        "v", "-45.0000", "-45.0000", "0.000000E+00", "\n",
                        "v", "-45.0000", "45.0000",  "0.000000E+00", "\n",
                        "v", "0.000000E+00", "0.000000E+00", "-78.0000", "\n", "\n",
                        "f", "1", "2", "3", "\n",
                        "f", "1", "3", "4", "\n",
                        "f", "1", "4", "5", "\n",
                        "f", "1", "5", "2", "\n",
                        "f", "6", "5", "4", "\n",
                        "f", "6", "4", "3", "\n",
                        "f", "6", "3", "2", "\n",
                        "f", "6", "2", "1", "\n",
                        "f", "6", "1", "5", "\n", "\n", "\n", "\n",
                    ].iter().map(|&str| String::from(str)).collect(),
                },
                TestCase {
                    data: String::from(r"                                   \
                        # trimming curve                                    \
                        vp -0.675  1.850 3.000                              \
                        vp  0.915  1.930                                    \
                        vp  2.485  0.470 2.000                              \
                        vp  2.485 -1.030                                    \
                        vp  1.605 -1.890 10.700                             \
                        vp -0.745 -0.654 0.500                              \
                        cstype rat bezier                                   \
                        deg 3                                               \
                        curv2 -6 -5 -4 -3 -2 -1 -6                          \
                        parm u 0.00 1.00 2.00                               \
                        end                                                 \
                        # special curve                                     \
                        vp -0.185 0.322                                     \   
                        vp  0.214 0.818                                     \
                        vp  1.652 0.207                                     \
                        vp  1.652 -0.455                                    \
                        curv2 -4 -3 -2 -1                                   \
                        parm u 2.00 10.00                                   \
                        end                                                 \
                        # surface                                           \
                        v -1.350 -1.030 0.000                               \
                        v  0.130 -1.030 0.432 7.600                         \
                        v  1.480 -1.030 0.000 2.300                         \
                        v -1.460  0.060 0.201                               \
                        v  0.120  0.060 0.915 0.500                         \
                        v  1.380  0.060 0.454 1.500                         \
                        v -1.480  1.030 0.000 2.300                         \
                        v  0.120  1.030 0.394 6.100                         \
                        v  1.170  1.030 0.000 3.300                         \
                        cstype rat bspline                                  \
                        deg 2 2                                             \
                        surf -1.0 2.5 -2.0 2.0 -9 -8 -7 -6 -5 -4 -3 -2 -1   \
                        parm u -1.00 -1.00 -1.00 2.50 2.50 2.50             \
                        parm v -2.00 -2.00 -2.00 2.00 2.00 2.00             \
                        trim 0.0 2.0 1                                      \
                        scrv 4.2 9.7 2                                      \
                        end                                                 \
                    "),
                    expected: vec![
                        "\n", "\n",
                        "vp", "-0.675", "1.850", "3.000", "\n",
                        "vp", "0.915", "1.930", "\n",
                        "vp", "2.485", "0.470", "2.000", "\n",
                        "vp", "2.485", "-1.030", "\n",
                        "vp", "1.605", "-1.890", "10.700", "\n",
                        "vp", "-0.745", "-0.654", "0.500", "\n",
                        "cstype", "rat", "bezier", "\n",
                        "deg", "3", "\n",
                        "curv2", "-6", "-5", "-4", "-3", "-2", "-1", "-6", "\n",
                        "parm", "u", "0.00", "1.00", "2.00", "\n",
                        "end", "\n", "\n",
                        "vp", "-0.185", "0.322", "\n",
                        "vp", "0.214", "0.818", "\n",
                        "vp", "1.652", "0.207", "\n",
                        "vp", "1.652", "-0.455", "\n",
                        "curv2", "-4", "-3", "-2", "-1", "\n",
                        "parm", "u", "2.00", "10.00", "\n",
                        "end", "\n", "\n",
                        "v", "-1.350", "-1.030", "0.000", "\n",
                        "v", "0.130", "-1.030", "0.432", "7.600", "\n",
                        "v", "1.480", "-1.030", "0.000", "2.300", "\n",
                        "v", "-1.460", "0.060", "0.201", "\n",
                        "v", "0.120", "0.060", "0.915", "0.500", "\n",
                        "v", "1.380", "0.060", "0.454", "1.500", "\n",
                        "v", "-1.480", "1.030", "0.000", "2.300", "\n",
                        "v", "0.120", "1.030", "0.394", "6.100", "\n",
                        "v", "1.170", "1.030", "0.000", "3.300",  "\n",
                        "cstype", "rat", "bspline",  "\n",
                        "deg", "2", "2",  "\n",
                        "surf", "-1.0", "2.5", "-2.0", "2.0", "-9", "-8", "-7", 
                                "-6", "-5", "-4", "-3", "-2", "-1", "\n",
                        "parm", "u", "-1.00", "-1.00", "-1.00", "2.50", "2.50", "2.50", "\n",
                        "parm", "v", "-2.00", "-2.00", "-2.00", "2.00", "2.00", "2.00", "\n",
                        "trim", "0.0", "2.0", "1", "\n",
                        "scrv", "4.2", "9.7", "2", "\n",
                        "end", "\n",
                    ].iter().map(|&str| String::from(str)).collect(),
                },
            ]
        }
    }

    /// for each instance of sample data, the resulting token stream
    /// should match the expected output from the lexer.
    #[test]
    fn test_lexer() {
        for test_case in test_cases().iter() {
            let lexer = ObjectLexer::new(Lexer::new(&test_case.data));
            let result = lexer.map(|token| token.into()).collect::<Vec<String>>();
            assert_eq!(result, test_case.expected);
        }
    }

    #[test]
    fn test_lexer_tokenwise() {
        for test_case in test_cases().iter() {
            let lexer = ObjectLexer::new(Lexer::new(&test_case.data));
        
            for (result, expected) in lexer.zip(test_case.expected.iter()) {
                assert_eq!(
                    result, expected,
                    "result = {:?}; expected = {:?}", result, expected
                );
            }
        }
    }
}

