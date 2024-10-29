use thermal_parser::thermal_file::{parse_str, parse_tokens};

#[test]
fn it_parses_tokens() {
    let tokens = parse_tokens(" ESC \"D\"   bob   35 0  0xFF 0x0 \"\\\"");
    println!("{:?}", tokens);
    assert_eq!(tokens.len(), 8);
    assert_eq!(tokens[0], "ESC");
    assert_eq!(tokens[1], "\"D");
    assert_eq!(tokens[2], "bob");
    assert_eq!(tokens[3], "35");
    assert_eq!(tokens[4], "0");
    assert_eq!(tokens[5], "0xFF");
    assert_eq!(tokens[6], "0x0");
    assert_eq!(tokens[7], "\"\\\""); //should be 2 1 quote and 1 escaped quote
}

#[test]
fn it_parses_lines() {
    let bytes =
        parse_str("'// This is a comment\r\n ESC \"D\"   bob   35 0  0xFF 0x0 \"\\\"\" \"\\\\\"");

    println!("{:?}", bytes);

    assert_eq!(bytes.len(), 11);
    assert_eq!(bytes[0], 27); // ESC
    assert_eq!(bytes[1], 68); // D
    assert_eq!(bytes[2], 98); // b
    assert_eq!(bytes[3], 111); // o
    assert_eq!(bytes[4], 98); // b
    assert_eq!(bytes[5], 35); // 35
    assert_eq!(bytes[6], 0); // 0
    assert_eq!(bytes[7], 255); // 0xFF = 255
    assert_eq!(bytes[8], 0); // 0x0 = 0
    assert_eq!(bytes[9], 34); // "
    assert_eq!(bytes[10], 92); // \
}
