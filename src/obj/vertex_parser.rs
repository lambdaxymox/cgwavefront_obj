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
    

    #[test]
    fn test_parse_vertex1() {
        let mut parser_state = super::ParserState::new("v -1.929448 13.329624 -5.221914\n");
        let parser = super::VertexParser::new();
        let vertex = Vertex { x: -1.929448, y: 13.329624, z: -5.221914, w: 1.0 };
        assert_eq!(parser.parse(&mut parser_state), Ok(vertex));
    }

    #[test]
    fn test_parse_vertex2() {
        let mut parser_state = super::ParserState::new("v -1.929448 13.329624 -5.221914 1.329624\n");
        let parser = super::VertexParser::new();
        let vertex = Vertex { x: -1.929448, y: 13.329624, z: -5.221914, w: 1.329624 };
        assert_eq!(parser.parse(&mut parser_state), Ok(vertex));
    }

    #[test]
    fn test_parse_vertex3() {
        let mut parser_state = super::ParserState::new("v -1.929448 13.329624 \n");
        let parser = super::VertexParser::new();
        assert!(parser.parse(&mut parser_state).is_err());
    }
}

