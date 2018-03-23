use obj::parser::{ParserState, ParseError};
use obj::object::TextureVertex;


pub struct TextureVertexParser {}

impl TextureVertexParser {
    fn new() -> TextureVertexParser { TextureVertexParser {} }

    fn parse(&self, state: &mut ParserState) -> Result<TextureVertex, ParseError> {
        try!(state.parse_statement("vt"));

        let u = try!(state.parse_f32());
        let mv = state.try_once(|st| st.parse::<f32>().ok());
        let v = match mv {
            Some(val) => val,
            None => return Ok(TextureVertex { u: u, v: 0., w: 0. })
        };

        let mw = state.try_once(|st| st.parse::<f32>().ok());
        let w = match mw {
            Some(val) => val,
            None => return Ok(TextureVertex { u: u, v: v, w: 0. })
        };

        Ok(TextureVertex { u: u, v: v, w: w })
    }
}


#[cfg(test)]
mod tests {
    use obj::object::TextureVertex;
    use obj::parser::{ParserState, ParseError};


    #[test]
    fn test_parse_texture_vertex1() {
        let mut state = ParserState::new("vt -1.929448");
        let parser = super::TextureVertexParser::new();
        let vt = TextureVertex { u: -1.929448, v: 0.0, w: 0.0 };
        assert_eq!(parser.parse(&mut state), Ok(vt));
    }

    #[test]
    fn test_parse_texture_vertex2() {
        let mut state = ParserState::new("vt -1.929448 13.329624 -5.221914");
        let parser = super::TextureVertexParser::new();
        let vt = TextureVertex { u: -1.929448, v: 13.329624, w: -5.221914 };
        assert_eq!(parser.parse(&mut state), Ok(vt));
    }

    #[test]
    fn test_parse_texture_vertex3() {
        let mut state = ParserState::new(
            "vt -1.929448 13.329624 -5.221914
             vt -27.6068  31.1438    27.2099"
        );
        let parser = super::TextureVertexParser::new();
        assert_eq!(
            parser.parse(&mut state), 
            Ok(TextureVertex { u: -1.929448, v: 13.329624, w: -5.221914 })
        );
        assert_eq!(state.next(), Some(String::from("\n")));
        assert_eq!(
            parser.parse(&mut state),
            Ok(TextureVertex { u: -27.6068, v: 31.1438, w: 27.2099 })
        );
    }
}

