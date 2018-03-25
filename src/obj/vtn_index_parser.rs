use obj::parser::{ParserState, ParseError};
use obj::object::VTNIndex;


struct VTNIndexParser {}

impl VTNIndexParser {
    fn new() -> VTNIndexParser { VTNIndexParser {} }

    #[inline(always)]
    fn parse_vn(&self, state: &mut ParserState, st: &str) -> Result<VTNIndex, ParseError> {
        if let Some(v_index_in_str) = st.find("//") {
            let v_index = match st[0..v_index_in_str].parse::<usize>() {
                Ok(val) => val,
                Err(_) => return state.error(format!("Expected `vertex` index but got `{}`", st))
            };
            let vn_index = match st[v_index_in_str+2..].parse::<usize>() {
                Ok(val) => val,
                Err(_) => return state.error(format!("Expected `normal` index but got `{}`", st))
            };

            return Ok(VTNIndex::new(v_index, None, Some(vn_index)));
        } else {
            return state.error(format!("Expected `vertex//normal` index but got `{}`", st))
        }
    }

    #[inline(always)]
    fn parse_vt(&self, state: &mut ParserState, st: &str) -> Result<VTNIndex, ParseError> {
        if let Some(v_index_in_str) = st.find("/") {
            let v_index = match st[0..v_index_in_str].parse::<usize>() {
                Ok(val) => val,
                Err(_) => return state.error(format!("Expected `vertex` index but got `{}`", st))
            };
            let vt_index = match st[v_index_in_str+1..].parse::<usize>() {
                Ok(val) => val,
                Err(_) => return state.error(format!("Expected `texture` index but got `{}`", st))
            };

            return Ok(VTNIndex::new(v_index, Some(vt_index), None));
        } else {
            return state.error(format!("Expected `vertex/texture` index but got `{}`", st))
        }
    }

    #[inline(always)]
    fn parse_vtn(&self, state: &mut ParserState, st: &str) -> Result<VTNIndex, ParseError> {
        let v_index_in_str = match st.find("/") {
            Some(val) => val,
            None => return state.error(format!("Expected `vertex` index but got `{}`", st))
        };
        let v_index = match st[0..v_index_in_str].parse::<usize>() {
            Ok(val) => val,
            Err(_) => return state.error(format!("Expected `vertex` index but got `{}`", st))
        };
        let vt_index_in_str = match st[(v_index_in_str + 1)..].find("/") {
            Some(val) => v_index_in_str + 1 + val,
            None => return state.error(format!("Expected `texture` index but got `{}`", st))
        };
        let vt_index = match st[(v_index_in_str + 1)..vt_index_in_str].parse::<usize>() {
            Ok(val) => val,
            Err(_) => return state.error(format!("Expected `texture` index but got `{}`", st))
        };
        let vn_index = match st[(vt_index_in_str + 1)..].parse::<usize>() {
            Ok(val) => val,
            Err(_) => return state.error(format!("Expected `normal` index but got `{}`", st))
        };
   
        Ok(VTNIndex::new(v_index, Some(vt_index), Some(vn_index)))
    }

    #[inline(always)]
    fn parse_v(&self, state: &mut ParserState, st: &str) -> Result<VTNIndex, ParseError> {
        match st.parse::<usize>() {
            Ok(val) => Ok(VTNIndex::new(val, None, None)),
            Err(_) => return state.error(format!("Expected `vertex` index but got `{}`", st))
        }
    }

    fn parse(&self, state: &mut ParserState) -> Result<VTNIndex, ParseError> {
        let st = try!(state.parse_string());
        match self.parse_vn(state, &st) {
            Ok(val) => return Ok(val),
            Err(_) => {},
        }
        match self.parse_vtn(state, &st) {
            Ok(val) => return Ok(val),
            Err(_) => {},
        }
        match self.parse_vt(state, &st) {
            Ok(val) => return Ok(val),
            Err(_) => {},
        }
        match self.parse_v(state, &st) {
            Ok(val) => return Ok(val),
            Err(_) => {},
        }

        state.error(format!("Expected `vertex/texture/normal` index but got `{}`", st))
    }
}


#[cfg(test)]
mod tests {
    use obj::object::VTNIndex;
    use obj::parser::ParserState;

    #[test]
    fn test_parse_vtn_index1() {
        let mut state = ParserState::new("1291");
        let parser = super::VTNIndexParser::new();
        let expected = VTNIndex::new(1291, None, None);
        let result = parser.parse(&mut state);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_vtn_index2() {
        let mut state = ParserState::new("1291/1315");
        let parser = super::VTNIndexParser::new();
        let expected = VTNIndex::new(1291, Some(1315), None);
        let result = parser.parse(&mut state);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_vtn_index3() {
        let mut state = ParserState::new("1291/1315/1314");
        let parser = super::VTNIndexParser::new();
        let expected = VTNIndex::new(1291, Some(1315), Some(1314));
        let result = parser.parse(&mut state);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_vtn_index4() {
        let mut state = ParserState::new("1291//1315");
        let parser = super::VTNIndexParser::new();
        let expected = VTNIndex::new(1291, None, Some(1315));
        let result = parser.parse(&mut state);
        assert_eq!(result, Ok(expected));
    }
}