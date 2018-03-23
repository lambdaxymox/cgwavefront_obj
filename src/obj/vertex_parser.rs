use obj::parser::{ParserState, ParseError};
use obj::object::Vertex;


pub struct VertexParser {}

impl VertexParser {
    pub fn new() -> VertexParser { VertexParser {} }

    pub fn parse(&self, state: &mut ParserState) -> Result<Vertex, ParseError> {
        try!(state.parse_statement("v"));
 
        let x = try!(state.parse_f32());
        let y = try!(state.parse_f32());
        let z = try!(state.parse_f32());
        let mw = state.try_once(|st| st.parse::<f32>().ok());
        let w = match mw {
            Some(val) => val,
            None => return Ok(Vertex { x: x, y: y, z: z, w: 1. })
        };

        Ok(Vertex { x: x, y: y, z: z, w: w })
    }
}


#[cfg(test)]
mod tests {
    use obj::object::Vertex;
    use obj::parser::ParserState;


    #[test]
    fn test_parse_vertex1() {
        let mut state = ParserState::new("v -1.929448 13.329624 -5.221914\n");
        let parser = super::VertexParser::new();
        let vertex = Vertex { x: -1.929448, y: 13.329624, z: -5.221914, w: 1.0 };
        assert_eq!(parser.parse(&mut state), Ok(vertex));
    }

    #[test]
    fn test_parse_vertex2() {
        let mut state = ParserState::new("v -1.929448 13.329624 -5.221914 1.329624\n");
        let parser = super::VertexParser::new();
        let vertex = Vertex { x: -1.929448, y: 13.329624, z: -5.221914, w: 1.329624 };
        assert_eq!(parser.parse(&mut state), Ok(vertex));
    }

    #[test]
    fn test_parse_vertex3() {
        let mut state = ParserState::new("v -1.929448 13.329624 \n");
        let parser = super::VertexParser::new();
        assert!(parser.parse(&mut state).is_err());
    }

    #[test]
    fn test_parse_vertex4() {
        let mut state = ParserState::new("v -1.929448 13.329624 -5.221914 1.329624\n v");
        let parser = super::VertexParser::new();
        assert!(parser.parse(&mut state).is_ok());
    }

    #[test]
    fn test_parse_vertex5() {
        let mut state = ParserState::new(
             "v -6.207583 1.699077 8.466142
              v -14.299248 1.700244 8.468981 1.329624"
        );
        let parser = super::VertexParser::new();
        assert_eq!(
            parser.parse(&mut state), 
            Ok(Vertex { x: -6.207583, y: 1.699077, z: 8.466142, w: 1.0 })
        );
        assert_eq!(state.next(), Some(String::from("\n")));
        assert_eq!(
            parser.parse(&mut state), 
            Ok(Vertex { x: -14.299248, y: 1.700244, z: 8.468981, w: 1.329624 })
        );
    }
}

