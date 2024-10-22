#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Header {
    field: String,
    body: String,
}

#[allow(dead_code)]
fn parse_header(line: &str) -> Header {
    let field_delimiter_index = line.find(":").unwrap();

    Header {
        field: line[0..field_delimiter_index].to_string(),
        body: line[field_delimiter_index + 1..].to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_unfolded_header() {
        let expected_header = Header {
            field: "this".to_string(),
            body: "is a simple header".to_string(),
        };
        let actual_header = parse_header("this:is a simple header");
        assert_eq!(expected_header, actual_header);
    }
}
