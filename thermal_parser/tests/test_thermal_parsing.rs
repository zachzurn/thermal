use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use thermal_parser::parse_esc_pos;
use thermal_parser::thermal_file::{cmds_to_thermal, parse_str, parse_tokens, try_const};

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


#[test]
fn it_decompiles_binary() {
    convert_binary_to_thermal("discount")
}

// Utility function for converting bin files
// into the more readable format. Ideally all
// tests are in the .thermal format
fn convert_binary_to_thermal(name: &str) {
    let binary_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("sample_files")
        .join("in")
        .join(format!("{}.bin", name));

    let thermal_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("sample_files")
        .join("in")
        .join(format!("{}.converted.thermal", name));

    let original_bytes = std::fs::read(binary_filepath.to_str().unwrap()).unwrap();
    let parsed_commands = parse_esc_pos(&original_bytes);
    let thermal_file = cmds_to_thermal(&parsed_commands);
    let new_bytes = parse_str(&thermal_file);

    for i in 0..original_bytes.len() {
        if new_bytes[i] != original_bytes[i] {
            let look_back = 15;
            let look_forward = 10;

            if i > look_back && i + look_forward <= original_bytes.len() {
                let mut debug_original = String::new();
                let mut debug_new = String::new();
                for j in i - look_back..i + look_forward + 1 {
                    debug_original.push_str(&*try_const(&original_bytes[j]));
                    debug_original.push(' ');

                    debug_new.push_str(&*try_const(&new_bytes[j]));
                    debug_new.push(' ');
                }

                println!("MISMATCH AT BYTE {}. See below for backtrace.", i);
                println!("ORIG: {}", &debug_original);
                println!("NEW:  {}", &debug_new);

                println!("{}", thermal_file);

                panic!("Bytes do not match at byte {}", i);
            }
        }
    }

    let mut file = File::create(thermal_filepath).unwrap();
    file.write_all(thermal_file.as_bytes())
        .expect("Can't write output to thermal file");
}

