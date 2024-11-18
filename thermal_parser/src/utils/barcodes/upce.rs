use barcoders::sym::ean13::{ENCODINGS, LEFT_GUARD, MIDDLE_GUARD};

pub const UPCE_RIGHT_GUARD: [u8; 1] = [1];

// UPC-E doesn't have a check digit encoded explicity, rather the check digit
// is encoded in the parity of the other six characters. The check digit that
// is encoded is the check digit from the original UPC-A barcode.
const UPCE_PARITY_TABLE: [[[usize; 6]; 10]; 2] = [
    // Number system 0
    [
        [1, 1, 1, 0, 0, 0], // When check digit = 0
        [1, 1, 0, 1, 0, 0], // When check digit = 1
        [1, 1, 0, 0, 1, 0], // When check digit = 2
        [1, 1, 0, 0, 0, 1], // ...
        [1, 0, 1, 1, 0, 0],
        [1, 0, 0, 1, 1, 0],
        [1, 0, 0, 0, 1, 1],
        [1, 0, 1, 0, 1, 0],
        [1, 0, 1, 0, 0, 1],
        [1, 0, 0, 1, 0, 1],
    ],
    // Number system 1
    [
        [0, 0, 0, 1, 1, 1],
        [0, 0, 1, 0, 1, 1],
        [0, 0, 1, 1, 0, 1],
        [0, 0, 1, 1, 1, 0],
        [0, 1, 0, 0, 1, 1],
        [0, 1, 1, 0, 0, 1],
        [0, 1, 1, 1, 0, 0],
        [0, 1, 0, 1, 0, 1],
        [0, 1, 0, 1, 1, 0],
        [0, 1, 1, 0, 1, 0],
    ],
];

pub struct UPCE(Vec<u8>);

impl UPCE {
    /// Creates a new barcode
    /// Data can be 6,7,8,11,12 digits long
    /// 6 digits = needs 0 and check digit added
    /// 7 digits = needs check digit added
    /// 8 digits = nothing needs to change
    /// 11 digits = needs check digit added and convert to upce
    /// 12 digits = convert to upce
    pub fn new(data: String) -> Result<UPCE, String> {
        let mut digits = vec![];

        for char in data.as_bytes() {
            if !char.is_ascii_digit() {
                return Err(String::from("Invalid UPCA: Only digits 1 - 9 are valid."));
            }

            digits.push(char.saturating_sub(48))
        }

        match digits.len() {
            6 => {
                //Missing number system and checksum
                let mut combined = vec![];
                combined.push(0u8);
                combined.extend(digits);
                return UPCE::from_upc_e(&combined);
            }
            7 => {
                //Missing checksum
                return UPCE::from_upc_e(&digits);
            }
            8 => {
                //Checksum is provided
                return UPCE::from_upc_e(&digits);
            }
            11 => {
                //Missing checksum
                digits.push(calculate_checkdigit(&digits));
                return UPCE::from_upc_a(&digits);
            }
            12 => {
                //Checksum is provided
                return UPCE::from_upc_a(&digits);
            }
            _ => {
                return Err(String::from(
                    "Invalid UPCA: Data length is not correct. Valid lengths are 6,7,8,11 and 12",
                ));
            }
        }
    }

    // Takes 7 or 8 bytes
    // When 7 bytes, the checksum is calculated after decompression
    // When 8 bytes, we assume the checksum is correct and still make
    // the round trip to upc_a and back to upc_e
    fn from_upc_e(upc_e: &[u8]) -> Result<UPCE, String> {
        if upc_e.len() < 7 || upc_e.len() > 8 {
            return Err(String::from(
                "Invalid UPCE: The UPCE length is not correct, should be 7 or 8 digits.",
            ));
        }

        let (bytes, precalulated_checksum, checksum) = if upc_e.len() == 7 {
            (&upc_e[..], false, 0)
        } else {
            (&upc_e[0..=6], true, upc_e[7])
        };

        let mut upc_a = match bytes[6] {
            // UPC-E code ends in 0, 1, or 2: The UPC-A code is determined by taking the first two digits of the UPC-E code,
            // taking the last digit of the UPC-E code, adding four 0 digits, and then adding characters 3 through 5 from the UPC-E code.
            0 | 1 | 2 => vec![
                // num     first      second    last     4 zeroes    third      fourth   fifth (last is dropped)
                bytes[0], bytes[1], bytes[2], bytes[6], 0, 0, 0, 0, bytes[3], bytes[4], bytes[5],
            ],

            // UPC-E code ends in 3: The UPC-A code is determined by taking the first three digits of the UPC-E code,
            // adding five 0 digits, then adding characters 4 and 5 from the UPC-E code.
            3 => vec![
                // num      first    second     third     five zeroes    fourth    fifth (last is dropped)
                bytes[0], bytes[1], bytes[2], bytes[3], 0, 0, 0, 0, 0, bytes[4], bytes[5],
            ],

            // UPC-E code ends in 4: The UPC-A code is determined by taking the first four digits of the UPC-E code,
            // adding five 0 digits, then adding the fifth character from the UPC-E code.
            4 => vec![
                // num      first    second    third     fourth   five zeroes     fifth (last is dropped)
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], 0, 0, 0, 0, 0, bytes[5],
            ],

            // UPC-E code ends in 5, 6, 7, 8, or 9: The UPC-A code is determined by taking the first five digits of the UPC-E code,
            // adding four 0 digits, then adding the last character from the UPC-E code.
            5..=9 => vec![
                // num     first     second    third      fourth   fifth    four zeroes  last
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], 0, 0, 0, 0, bytes[6],
            ],

            _ => {
                return Err("Invalid UPCE: The last digit should be a 3,4,5,6,7,8 or 9".to_string())
            }
        };

        if precalulated_checksum {
            upc_a.push(checksum);
        } else {
            upc_a.push(calculate_checkdigit(&upc_a));
        }

        UPCE::from_upc_a(&upc_a)
    }

    /// Takes an 12 digit upca value and compresses into an upce value
    /// This function can take any number of bytes and will always return
    /// a valid upce or an explanatory error result
    fn from_upc_a(upc_a: &[u8]) -> Result<UPCE, String> {
        if upc_a.len() != 12 {
            return Err(String::from(
                "Invalid UPCA: The UPCA length is not correct, should be 12 digits.",
            ));
        }

        //Number system
        let nsys = upc_a[0];

        if nsys != 1 && nsys != 0 {
            return Err(String::from(
                "Invalid UPCA: The number system (first digit) should be 0 or 1.",
            ));
        }

        //Manufacturer code
        let manu = &upc_a[1..=5];

        //Product code
        let prod = &upc_a[6..=10];

        //Check digit
        let chck = upc_a[11];

        // Condition 1: Manufacturer code ends in 000, 100, or 200

        // If the manufacturer code ends in 000, 100, or 200, the UPC-E code consists
        // of the first two characters of the manufacturer code, the last three
        // characters of the product code, followed by the third character of the
        // manufacturer code. The product code must be 00000 to 00999.
        if manu[2..] == [0, 0, 0] || manu[2..] == [1, 0, 0] || manu[2..] == [2, 0, 0] {
            if prod[0] != 0 && prod[1] != 0 {
                return Err(String::from(
                    "Invalid UPCA: Product code can not be greater than 00999.",
                ));
            }

            return Ok(UPCE(vec![
                nsys, manu[0], manu[1], prod[2], prod[3], prod[4], manu[2], chck,
            ]));
        }

        // If the manufacturer code ends in 00 but does not qualify for #1 above,
        // the UPC-E code consists of the first three characters of the manufacturer code,
        // the last two characters of the product code, followed by the digit "3".
        // The product code must be 00000 to 00099.
        if manu[3..] == [0, 0] {
            if prod[0] != 0 && prod[1] != 0 {
                return Err(String::from(
                    "Invalid UPCA: Product code can not be greater than 00999.",
                ));
            }

            return Ok(UPCE(vec![
                nsys, manu[0], manu[1], manu[2], prod[3], prod[4], 3, chck,
            ]));
        }

        // If the manufacturer code ends in 0 but does not quality for #1 or #2 above,
        // the UPC-E code consists of the first four characters of the manufacturer code,
        // the last character of the product code, followed by the digit "4".
        // The product code must be 00000 to 00009.
        if manu[4..] == [0] {
            if prod[0] != 0 && prod[1] != 0 && prod[2] != 0 && prod[3] != 0 {
                return Err(String::from(
                    "Invalid UPCA: Product code can not be greater than 00009.",
                ));
            }

            return Ok(UPCE(vec![
                nsys, manu[0], manu[1], manu[2], manu[3], prod[4], 4, chck,
            ]));
        }

        // If the manufacturer code does not end in zero, the UPC-E code consists of the
        // entire manufacturer code and the last digit of the product code. Note that the
        // last digit of the product code must be in the range of 5 through 9.
        // The product code must be 00005 to 00009.
        if manu[4] != 0 {
            if prod[0] != 0
                || prod[1] != 0
                || prod[2] != 0
                || prod[3] != 0
                || prod[4] < 5
                || prod[4] > 9
            {
                return Err(String::from(
                    "Invalid UPCA: Product code can not be greater than 00005 and less than 00009.",
                ));
            }

            return Ok(UPCE(vec![
                nsys, manu[0], manu[1], manu[2], manu[3], manu[4], prod[4], chck,
            ]));
        }

        Err(String::from(
            "Invalid UPCA: Unable to compress into UPCE, manufacturer code does not end in zero.",
        ))
    }

    fn get_parity_encoding(&self) -> [usize; 6] {
        // Uses the UPC-A check digit in the parity table
        UPCE_PARITY_TABLE[self.0[0] as usize][self.0[7] as usize]
    }

    // COPIED
    fn char_encoding(&self, side: usize, d: u8) -> [u8; 7] {
        ENCODINGS[side][d as usize]
    }

    // COPIED
    /// Joins and flattens the given slice of &[u8] slices into a Vec<u8>.
    /// TODO: Work out how to use join_iters with slices and then remove this function.
    fn join_slices(&self, slices: &[&[u8]]) -> Vec<u8> {
        slices.iter().flat_map(|b| b.iter()).cloned().collect()
    }

    // COPIED
    /// Joins and flattens the given iterator of iterables into a Vec<u8>.
    fn join_iters<'a, T: Iterator>(&self, iters: T) -> Vec<u8>
    where
        T::Item: IntoIterator<Item = &'a u8>,
    {
        iters.flat_map(|b| b.into_iter()).cloned().collect()
    }

    // COPIED
    fn upce_payload(&self) -> Vec<u8> {
        let slices: Vec<[u8; 7]> = self.0[1..=6]
            .iter()
            .zip(self.get_parity_encoding().iter())
            .map(|(d, s)| self.char_encoding(*s, *d))
            .collect();

        self.join_iters(slices.iter())
    }

    // COPIED AND EDITED
    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> Vec<u8> {
        self.join_slices(
            &[
                &LEFT_GUARD[..],
                &self.upce_payload()[..],
                &MIDDLE_GUARD[..],
                &UPCE_RIGHT_GUARD[..],
            ][..],
        )
    }
}

//Should be provided 11 digits for a proper checksum
fn calculate_checkdigit(data: &[u8]) -> u8 {
    let mut odds = 0;
    let mut evens = 0;

    for (i, d) in data.iter().enumerate() {
        match (i + 1) % 2 {
            1 => odds += *d,
            _ => evens += *d,
        }
    }

    odds *= 3;

    match 10 - ((odds + evens) % 10) {
        10 => 0,
        n => n,
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::barcodes::upce::{calculate_checkdigit, UPCE};

    #[test]
    fn checkdigit() {
        let checksum_1 = calculate_checkdigit(&[0, 4, 2, 1, 0, 0, 0, 0, 5, 2, 6]);
        assert_eq!(checksum_1, 4);

        let checksum_2 = calculate_checkdigit(&[8, 1, 0, 0, 1, 2, 1, 1, 0, 0, 9]);
        assert_eq!(checksum_2, 9);

        let checksum_2 = calculate_checkdigit(&[8, 1, 2, 0, 1, 2, 1, 1, 6, 0, 9]);
        assert_eq!(checksum_2, 5);
    }

    #[test]
    fn from_upc_a_11() {
        let upc_e = UPCE::new("04210000526".to_string()).unwrap();
        assert_eq!(upc_e.0, [0, 4, 2, 5, 2, 6, 1, 4]);

        let upc_e_2 = UPCE::new("01200000789".to_string()).unwrap();
        assert_eq!(upc_e_2.0, [0, 1, 2, 7, 8, 9, 0, 7]);
    }

    #[test]
    fn from_upc_a_12() {
        let upc_e = UPCE::new("042100005264".to_string()).unwrap();
        assert_eq!(upc_e.0, [0, 4, 2, 5, 2, 6, 1, 4]);

        let upc_e_2 = UPCE::new("012000007897".to_string()).unwrap();
        assert_eq!(upc_e_2.0, [0, 1, 2, 7, 8, 9, 0, 7]);
    }

    #[test]
    fn from_upc_e_6() {
        let upc_e = UPCE::new("425261".to_string()).unwrap();
        assert_eq!(upc_e.0, [0, 4, 2, 5, 2, 6, 1, 4]);
    }

    #[test]
    fn from_upc_e_7() {
        let upc_e = UPCE::new("0425261".to_string()).unwrap();
        assert_eq!(upc_e.0, [0, 4, 2, 5, 2, 6, 1, 4]);
    }

    #[test]
    fn from_upc_e_8() {
        let upc_e = UPCE::new("04252614".to_string()).unwrap();
        assert_eq!(upc_e.0, [0, 4, 2, 5, 2, 6, 1, 4]);
    }
}
