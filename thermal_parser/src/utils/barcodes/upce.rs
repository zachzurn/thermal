use barcoders::sym::ean13::{ENCODINGS, LEFT_GUARD, MIDDLE_GUARD};

/// UPC-E paraty encoding table (2 cols and 10 rows)
/// Col 0 = Number system 0 encoding; Col 1 = Number system 1 encoding
/// 0 (ODD), 1 (EVEN)
const UPCE_PARATY_TABLE: [[[usize; 6]; 10]; 2] = [
    [[1, 1, 1, 0, 0, 0], [1, 1, 0, 1, 0, 0], [1, 1, 0, 0, 1, 0],
        [1, 1, 0, 0, 0, 1], [1, 0, 1, 1, 0, 0], [1, 0, 0, 1, 1, 0],
        [1, 0, 0, 0, 1, 1], [1, 0, 1, 0, 1, 0], [1, 0, 1, 0, 0, 1],
        [1, 0, 0, 1, 0, 1]],
    [[0, 0, 0, 1, 1, 1], [0, 0, 1, 0, 1, 1], [0, 0, 1, 1, 0, 1],
        [0, 0, 1, 1, 1, 0], [0, 1, 0, 0, 1, 1], [0, 1, 1, 0, 0, 1],
        [0, 1, 1, 1, 0, 0], [0, 1, 0, 1, 0, 1], [0, 1, 0, 1, 1, 0],
        [0, 1, 1, 0, 1, 0]]
];

const RIGHT_GUARD: [u8; 1] = [1];

/// The UPCE barcode type.
#[derive(Debug)]
pub struct UPCE(Vec<u8>);
impl UPCE {
    /// Creates a new barcode
    pub fn new(data: String) -> Result<UPCE, String> {
        if data.chars().all(|c| c.is_digit(10)) && data.len() == 12 {
            let digits: Vec<u8> = data
                .chars()
                .map(|c| c.to_digit(10).expect("Unknown character") as u8)
                .collect();
            Ok(UPCE(digits))
        } else {
            Err(String::from("Invalid UPCA: The code must contain exactly 12 digits."))
        }

   }

    fn upca_to_upce<'a>(&'a self, upce: &'a mut Vec<u8>) -> &[u8]  {
        let upca_formated = &self.0[1..11];

        // Extract the manufacturer code and product code
        let (manufacturer_code, product_code) = upca_formated.split_at(5);

        let product_code_str = self.vec_to_str(product_code);
        let formatted_product_code = self.remove_leading_zeros(product_code_str);

        match formatted_product_code.parse::<i32>() {
            Ok(number) => {
                // Condition 1: Manufacturer code ends in 000, 100, or 200
                if manufacturer_code[2..] == [0, 0, 0] ||
                    manufacturer_code[2..] == [1, 0, 0] ||
                    manufacturer_code[2..] == [2, 0, 0] {
                    if number >= 0 && number <= 999 {
                        upce.extend_from_slice(&manufacturer_code[..2]);
                        upce.extend_from_slice(&product_code[2..]);
                        upce.extend_from_slice(&manufacturer_code[2..3])
                    }
                }
                // Condition 2: Manufacturer code ends in 00 but does not qualify for #1
                else if manufacturer_code[3..] == [0, 0] {
                    if number >= 0 && number <= 99 {
                        upce.extend_from_slice(&manufacturer_code[..3]);
                        upce.extend_from_slice(&product_code[3..]);
                        upce.push(3);
                    }
                }
                // Condition 3: Manufacturer code ends in 0 but does not qualify for #1 or #2
                else if manufacturer_code[4..] == [0] {
                    if number >= 0 && number <= 9 {
                        upce.extend_from_slice(&manufacturer_code[..4]);
                        upce.extend_from_slice(&product_code[4..]);
                        upce.push(4);
                    }
                }
                // Condition 4: Manufacturer code does not end in zero
                else {
                    if number >= 5 && number <= 9 {
                        let last_digit = (product_code[4..])[0];
                        // Ensure the last digit of the product code is in the range of 5 through 9
                        if (5..=9).contains(&last_digit) {
                            upce.extend_from_slice(manufacturer_code);
                            upce.push(last_digit);
                        }

                    }
                }
            }
            _ => {
                println!("Failed to parse the string as a number");
            }
        };
        &mut *upce
    }

    fn remove_leading_zeros(&self, input: String) -> String {
        let result: String = input.chars().skip_while(|&c| c == '0').collect();

        if result.is_empty() {
            "0".to_string()
        } else {
            result
        }
    }

    fn vec_to_str(&self, input: &[u8]) -> String {
        let mut inp_str = String::from("");
        for i in input {
            inp_str.push_str(&i.to_string());
        }
        inp_str
    }

    fn get_check_digit(&self) -> u8 {
        self.0[11]
    }

    fn get_paraty_encoding(&self) -> [usize; 6] {
        // Uses the UPC-A check digit in the parity table
        UPCE_PARATY_TABLE[self.0[0] as usize][self.get_check_digit() as usize]
    }

    // COPIED
    fn char_encoding(&self, side: usize, d: u8) -> [u8; 7] {
        ENCODINGS[side][d as usize]
    }

    // COPIED
    /// Joins and flattens the given slice of &[u8] slices into a Vec<u8>.
    /// TODO: Work out how to use join_iters with slices and then remove this function.
    fn join_slices(&self, slices: &[&[u8]]) -> Vec<u8> {
        slices.iter()
            .flat_map(|b| b.iter())
            .cloned()
            .collect()
    }

    // COPIED
    /// Joins and flattens the given iterator of iterables into a Vec<u8>.
    fn join_iters<'a, T: Iterator>(&self, iters: T) -> Vec<u8>
        where T::Item: IntoIterator<Item=&'a u8> {
        iters.flat_map(|b| b.into_iter())
            .cloned()
            .collect()
    }

    // COPIED
    fn upce_payload(&self) -> Vec<u8> {
        let mut upce: Vec<u8> = Vec::new();
        self.upca_to_upce(&mut upce);
        let slices: Vec<[u8; 7]> = self.upca_to_upce(&mut upce)
            .iter()
            .zip(self.get_paraty_encoding().iter())
            .map(|(d, s)| self.char_encoding(*s, *d))
            .collect();

        self.join_iters(slices.iter())
    }

    // COPIED AND EDITED
    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> Vec<u8> {
        self.join_slices(&[&LEFT_GUARD[..],
            &self.upce_payload()[..],
            &MIDDLE_GUARD[..],
            &RIGHT_GUARD[..]][..])
    }
}
