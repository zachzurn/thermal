type Language = (&'static str, &'static [(u8, &'static str)]);

pub static USA: Language = ("Us English", &[]);

pub static FRANCE: Language = (
    "French",
    &[
        (0x40, "à"),
        (0x5B, "ç"),
        (0x5C, "§"),
        (0x5D, "^"),
        (0x5E, "é"),
        (0x60, "ù"),
        (0x7B, "è"),
        (0x7C, "¨"),
        (0x7D, "û"),
        (0x7E, "ü"),
    ],
);

pub static GERMANY: Language = (
    "German",
    &[
        (0x40, "§"),
        (0x5B, "Ä"),
        (0x5C, "Ö"),
        (0x5D, "Ü"),
        (0x60, "ä"),
        (0x7B, "ö"),
        (0x7C, "ü"),
        (0x7D, "ß"),
    ],
);

pub static UK: Language = ("British English", &[(0x23, "£")]);

pub static DENMARK_I: Language = (
    "Danish Variant I",
    &[
        (0x40, "Ä"),
        (0x5B, "Æ"),
        (0x5C, "Ø"),
        (0x5D, "Å"),
        (0x5E, "^"),
        (0x60, "ä"),
        (0x7B, "æ"),
        (0x7C, "ø"),
        (0x7D, "å"),
        (0x7E, "~"),
    ],
);

pub static SWEDEN: Language = (
    "Swedish",
    &[
        (0x40, "É"),
        (0x5B, "Ä"),
        (0x5C, "Ö"),
        (0x5D, "Å"),
        (0x5E, "Ü"),
        (0x60, "é"),
        (0x7B, "ä"),
        (0x7C, "ö"),
        (0x7D, "å"),
        (0x7E, "ü"),
    ],
);

pub static ITALY: Language = (
    "Italian",
    &[
        (0x40, "§"),
        (0x5C, "ù"),
        (0x5E, "é"),
        (0x60, "à"),
        (0x7B, "ò"),
        (0x7C, "ç"),
        (0x7D, "è"),
        (0x7E, "ì"),
    ],
);

pub static SPAIN_I: Language = (
    "Spanish Variant I",
    &[
        (0x40, "Ñ"),
        (0x5B, "¡"),
        (0x5C, "¿"),
        (0x5D, "ç"),
        (0x7B, "ñ"),
        (0x7C, "¬"),
        (0x7D, "½"),
        (0x7E, "¼"),
    ],
);

pub static JAPAN: Language = ("Japanese", &[(0x5C, "¥"), (0x7C, "¦")]);

pub static NORWAY: Language = (
    "Norwegian",
    &[
        (0x40, "Ä"),
        (0x5B, "Æ"),
        (0x5C, "Ø"),
        (0x5D, "Å"),
        (0x5E, "Ü"),
        (0x60, "ä"),
        (0x7B, "æ"),
        (0x7C, "ø"),
        (0x7D, "å"),
        (0x7E, "ü"),
    ],
);

pub static DENMARK_II: Language = (
    "Danish Variant II",
    &[
        (0x40, "É"),
        (0x5B, "Æ"),
        (0x5C, "Ø"),
        (0x5D, "Å"),
        (0x5E, "Ü"),
        (0x60, "é"),
        (0x7B, "æ"),
        (0x7C, "ø"),
        (0x7D, "å"),
        (0x7E, "ü"),
    ],
);

pub static SPAIN_II: Language = (
    "Spanish Variant II",
    &[
        (0x40, "á"),
        (0x5B, "í"),
        (0x5C, "Ñ"),
        (0x5D, "Ó"),
        (0x5E, "Ú"),
        (0x60, "ñ"),
        (0x7B, "á"),
        (0x7C, "í"),
        (0x7D, "ó"),
        (0x7E, "ú"),
    ],
);

pub static LATIN_AMERICA: Language = (
    "Latin",
    &[
        (0x40, "á"),
        (0x5B, "í"),
        (0x5C, "Ñ"),
        (0x5D, "Ó"),
        (0x5E, "Ú"),
        (0x60, "ñ"),
        (0x7B, "é"),
        (0x7C, "í"),
        (0x7D, "ó"),
        (0x7E, "ú"),
    ],
);

pub static KOREA: Language = ("Korean", &[(0x5C, "₩"), (0x7C, "₩")]);

pub static SLOVENIA: Language = (
    "Slovenian",
    &[
        (0x5B, "Š"),
        (0x5C, "Đ"),
        (0x5D, "Č"),
        (0x7B, "ž"),
        (0x7C, "đ"),
        (0x7D, "č"),
    ],
);

pub static CHINA: Language = ("Chinese", &[(0x5C, "¥")]);

pub static VIETNAM: Language = ("Vietnamese", &[(0x23, "đ")]);

pub static ARABIA: Language = ("Arabic", &[(0x2A, "٭")]);

pub static DEVANAGARI: Language = (
    "Devengari",
    &[
        (0x30, "०"),
        (0x31, "१"),
        (0x32, "२"),
        (0x33, "३"),
        (0x34, "४"),
        (0x35, "५"),
        (0x36, "६"),
        (0x37, "७"),
        (0x38, "८"),
        (0x39, "९"),
    ],
);

pub static BENGALI: Language = (
    "Bengali",
    &[
        (0x30, "০"),
        (0x31, "১"),
        (0x32, "২"),
        (0x33, "৩"),
        (0x34, "৪"),
        (0x35, "৫"),
        (0x36, "৬"),
        (0x37, "৭"),
        (0x38, "৮"),
        (0x39, "৯"),
    ],
);

pub static TAMIL: Language = (
    "Tamil",
    &[
        (0x30, "௦"),
        (0x31, "௧"),
        (0x32, "௨"),
        (0x33, "௩"),
        (0x34, "௪"),
        (0x35, "௫"),
        (0x36, "௬"),
        (0x37, "௭"),
        (0x38, "௮"),
        (0x39, "௯"),
    ],
);

pub static TELUGU: Language = (
    "Telugu",
    &[
        (0x30, "౦"),
        (0x31, "౧"),
        (0x32, "౨"),
        (0x33, "౩"),
        (0x34, "౪"),
        (0x35, "౫"),
        (0x36, "౬"),
        (0x37, "౭"),
        (0x38, "౮"),
        (0x39, "౯"),
    ],
);

pub static ASSAMESE: Language = (
    "Assamese",
    &[
        (0x30, "০"),
        (0x31, "১"),
        (0x32, "২"),
        (0x33, "৩"),
        (0x34, "৪"),
        (0x35, "৫"),
        (0x36, "৬"),
        (0x37, "৭"),
        (0x38, "৮"),
        (0x39, "৯"),
    ],
);

pub static ORIYA: Language = (
    "Oriya",
    &[
        (0x30, "୦"),
        (0x31, "୧"),
        (0x32, "୨"),
        (0x33, "୩"),
        (0x34, "୪"),
        (0x35, "୫"),
        (0x36, "୬"),
        (0x37, "୭"),
        (0x38, "୮"),
        (0x39, "୯"),
    ],
);

pub static KANNADA: Language = (
    "Kannada",
    &[
        (0x30, "೦"),
        (0x31, "೧"),
        (0x32, "೨"),
        (0x33, "೩"),
        (0x34, "೪"),
        (0x35, "೫"),
        (0x36, "೬"),
        (0x37, "೭"),
        (0x38, "೮"),
        (0x39, "೯"),
    ],
);

pub static MALAYALAM: Language = (
    "Malayalam",
    &[
        (0x30, "൦"),
        (0x31, "൧"),
        (0x32, "൨"),
        (0x33, "൩"),
        (0x34, "൪"),
        (0x35, "൫"),
        (0x36, "൬"),
        (0x37, "൭"),
        (0x38, "൮"),
        (0x39, "൯"),
    ],
);

pub static GUJARATI: Language = (
    "Gujarati",
    &[
        (0x30, "૦"),
        (0x31, "૧"),
        (0x32, "૨"),
        (0x33, "૩"),
        (0x34, "૪"),
        (0x35, "૫"),
        (0x36, "૬"),
        (0x37, "૭"),
        (0x38, "૮"),
        (0x39, "૯"),
    ],
);

pub static PUNJABI: Language = (
    "Punjabi",
    &[
        (0x30, "੦"),
        (0x31, "੧"),
        (0x32, "੨"),
        (0x33, "੩"),
        (0x34, "੪"),
        (0x35, "੫"),
        (0x36, "੬"),
        (0x37, "੭"),
        (0x38, "੮"),
        (0x39, "੯"),
    ],
);

pub static MARATHI: Language = (
    "Marathi",
    &[
        (0x30, "०"),
        (0x31, "१"),
        (0x32, "२"),
        (0x33, "३"),
        (0x34, "४"),
        (0x35, "५"),
        (0x36, "६"),
        (0x37, "७"),
        (0x38, "८"),
        (0x39, "९"),
    ],
);
