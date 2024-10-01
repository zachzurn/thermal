mod codepage_0_437;
mod codepage_11_pc851;
mod codepage_12_pc853;
mod codepage_13_pc857;
mod codepage_1_katakana;
mod codepage_2_pc850;
mod codepage_3_pc860;
mod codepage_4_pc863;
mod codepage_5_pc865;
mod codepage_base;
mod international;

#[derive(Clone)]
pub struct Codepage {
    table: [&'static str; 256],
    pub name: &'static str,
    pub language: &'static str,
}

impl Codepage {
    //TODO there may be a more efficient way to do this
    pub fn decode_utf8(&self, bytes: &[u8]) -> String {
        let mut decoded: Vec<&str> = Vec::new();

        for byte in bytes {
            let string_part = self.table[*byte as usize];
            decoded.push(string_part)
        }

        decoded.join("")
    }
}

pub fn get_codepage(codepage_index: u8, language_index: u8) -> Codepage {
    let mut codepage = [""; 256];
    let mut index: usize = 0;
    let (_base_name, base_table) = codepage_base::TABLE;

    //Apply the base codepage 0 - 128
    for code in base_table {
        codepage[index] = code;
        index += 1;
    }

    //Apply the codepage
    let (codepage_name, codepage_table) = get_codepage_table(codepage_index);

    for code in codepage_table {
        codepage[index] = code;
        index += 1;
    }

    //Make language specific replacements
    let (language_name, language_replacements) = get_language_replacements(language_index);
    for (index, str) in language_replacements {
        codepage[*index as usize] = str
    }

    Codepage {
        table: codepage,
        name: codepage_name,
        language: language_name,
    }
}

fn get_codepage_table(codepage_index: u8) -> (&'static str, &'static [&'static str; 128]) {
    match codepage_index {
        1 => codepage_1_katakana::TABLE,
        2 => codepage_2_pc850::TABLE,
        3 => codepage_3_pc860::TABLE,
        4 => codepage_4_pc863::TABLE,
        5 => codepage_5_pc865::TABLE,
        11 => codepage_11_pc851::TABLE,
        12 => codepage_12_pc853::TABLE,
        13 => codepage_13_pc857::TABLE,
        _ => codepage_0_437::TABLE,
    }
}

fn get_language_replacements(language_index: u8) -> (&'static str, &'static [(u8, &'static str)]) {
    match language_index {
        1 => international::FRANCE,
        2 => international::GERMANY,
        3 => international::UK,
        4 => international::DENMARK_I,
        5 => international::SWEDEN,
        6 => international::ITALY,
        7 => international::SPAIN_I,
        8 => international::JAPAN,
        9 => international::NORWAY,
        10 => international::DENMARK_II,
        11 => international::SPAIN_II,
        12 => international::LATIN_AMERICA,
        13 => international::KOREA,
        14 => international::SLOVENIA,
        15 => international::CHINA,
        16 => international::VIETNAM,
        17 => international::ARABIA,
        66 => international::DEVANAGARI,
        67 => international::BENGALI,
        68 => international::TAMIL,
        69 => international::TELUGU,
        70 => international::ASSAMESE,
        71 => international::ORIYA,
        72 => international::KANNADA,
        73 => international::MALAYALAM,
        74 => international::GUJARATI,
        75 => international::PUNJABI,
        82 => international::MARATHI,
        _ => international::USA,
    }
}
