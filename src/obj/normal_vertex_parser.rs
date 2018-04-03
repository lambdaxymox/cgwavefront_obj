use obj::parser::{ParserState, ParseError};
use obj::object::NormalVertex;


pub struct NormalVertexParser {}
    
impl NormalVertexParser {
    pub fn new() -> NormalVertexParser { NormalVertexParser {} }

    pub fn parse(&self, state: &mut ParserState) -> Result<NormalVertex, ParseError> {
        try!(state.expect("vn"));

        let i = try!(state.parse_f32());
        let j = try!(state.parse_f32());
        let k = try!(state.parse_f32());

        Ok(NormalVertex { i: i, j: j, k: k })
    }
}


#[cfg(test)]
mod tests {
    use obj::object::NormalVertex;
    use obj::parser::ParserState;


    #[test]
    fn test_parse_normal_vertex1() {
        let mut state = ParserState::new("vn  -0.966742  -0.255752  9.97231e-09");
        let parser = super::NormalVertexParser::new();
        let vn = NormalVertex { i: -0.966742, j: -0.255752, k: 9.97231e-09 };
        assert_eq!(parser.parse(&mut state), Ok(vn));
    }

    #[test]
    fn test_parse_normal_vertex2() {
        let mut state = ParserState::new(
            "vn -1.929448 13.329624 -5.221914
             vn -27.6068  31.1438    27.2099"
        );
        let parser = super::NormalVertexParser::new();
        assert_eq!(
            parser.parse(&mut state), 
            Ok(NormalVertex { i: -1.929448, j: 13.329624, k: -5.221914 })
        );
        assert_eq!(state.next(), Some(String::from("\n")));
        assert_eq!(
            parser.parse(&mut state),
            Ok(NormalVertex { i: -27.6068, j: 31.1438, k: 27.2099 })
        );        
    }
}

