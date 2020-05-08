use std::iter;

 
#[inline]
fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\\' || ch == '\t'
}

#[inline]
fn is_newline(ch: char) -> bool {
    ch == '\n' || ch == '\r'
}

#[inline]
fn is_whitespace_or_newline(ch: char) -> bool {
    is_whitespace(ch) || is_newline(ch)
}
/// The return type from the lexer.
#[derive(Clone, Debug)]
pub struct Token {
    pub line_number: usize,
    pub content: String,
}

impl Token {
    fn new(line_number: usize, content: String) -> Token {
        Token {
            line_number: line_number,
            content: content,
        }
    }
}

/// A OBJ file lexer tokenizes an input character stream.
#[derive(Clone)]
pub struct Lexer<Stream> where Stream: Iterator<Item=char> {
    /// The current line position in the token stream.
    current_line_number: usize,
    /// The input stream.
    stream: iter::Peekable<Stream>,
}

impl<Stream> Lexer<Stream> where Stream: Iterator<Item=char> {
    /// Create a new lexer.
    pub fn new(stream: Stream) -> Lexer<Stream> {
        Lexer {
            current_line_number: 1,
            stream: stream.peekable(),
        }
    }

    /// The function `peek_u8` looks at the character at the current position
    /// in the byte stream without advancing the stream.
    #[inline]
    fn peek_char(&mut self) -> Option<char> {
        self.stream.peek().map(|&x| x)
    }

    /// the function `advance` advances the lexer by one
    /// character in the byte stream.
    fn advance(&mut self) {
        match self.stream.next() {
            Some(ch) if is_newline(ch) => {
                self.current_line_number += 1;
            },
            _ => {}
        }
    }

    /// The function `skip_comment` consumes a comment line
    /// without returning it.
    fn skip_comment(&mut self) -> usize {
        let mut skipped = 0;
        loop {
            match self.peek_char() { 
                Some(ch) if !is_newline(ch) => {
                    self.advance();
                    skipped += 1;
                }
                _ => break,
            }
        }

        skipped
    }

    /// The function `skip_whitespace` consumes a string of whitespace
    /// characters without returning them.
    fn skip_whitespace(&mut self) -> usize {
        let mut skipped = 0;
        loop {
            match self.peek_char() {
                Some(ch) if is_whitespace(ch) => {
                    self.advance();
                    skipped += 1;
                }
                _ => {
                    break;
                }
            }
        }

        skipped
    }

    /// The method `next_token` fetches the next token from the input stream.
    fn next_token(&mut self) -> Option<Token> {
        // The lexer has processed each token it has seen so far. 
        // We must fetch and then buffer another token from the stream.

        // Count the number of bytes consumed for a token.
        let mut consumed: usize = 0;
        let mut token: String = String::new();
        loop {
            match self.peek_char() {
                Some(ch) if ch == '#' => {
                    self.skip_comment();
                }
                Some(ch) if is_whitespace_or_newline(ch) => {
                    // If the cursor is pointing at a whitespace or newline character,
                    // there are two possible situations:
                    // (1) We are at the end of the token,
                    // (2) We have not encountered a token yet.
                    if consumed != 0 {
                        // We are at the end of a token.
                        break;
                    } else if is_newline(ch) {
                        // We are at the end of a line.
                        self.advance();
                        token.push('\n');
                        consumed += 1;
                        break;
                    } else {
                        // We have consumed only whitespace. No token has been found yet.
                        self.skip_whitespace();
                    }
                }
                Some(ch) => {
                    self.advance();
                    token.push(ch);
                    consumed += 1;
                }
                None => {
                    break;
                }
            }
        }

        if consumed != 0 {
            // We consumed a token.
            debug_assert!(token.len() != 0);
            Some(Token::new(self.current_line_number, token))
        } else {
            debug_assert!(token.len() == 0);
            None
        }
    }
}

impl<Stream> Iterator for Lexer<Stream> where Stream: Iterator<Item=char> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}


#[cfg(test)]
mod tests {
    use super::Lexer;
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
            let lexer = Lexer::new(test_case.data.chars());
            let result = lexer.map(|token| token.content).collect::<Vec<String>>();
            assert_eq!(result, test_case.expected);
        }
    }

    #[test]
    fn test_lexer_tokenwise() {
        for test_case in test_cases().iter() {
            let lexer = Lexer::new(test_case.data.chars());
        
            for (result, expected) in lexer.zip(test_case.expected.iter()) {
                assert_eq!(
                    &result.content, expected,
                    "result = {:?}; expected = {:?}", result, expected
                );
            }
        }
    }
}

