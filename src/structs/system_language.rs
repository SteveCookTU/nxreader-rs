pub enum SystemLanguage {
    Ja,
    Enus,
    Fr,
    De,
    It,
    Es,
    Zhcn,
    Ko,
    Nl,
    Pt,
    Zhtw,
    Engb,
    Frca,
    Es419,
    Zhhans,
    Zhhant,
}

impl From<SystemLanguage> for u8 {
    fn from(lang: SystemLanguage) -> Self {
        match lang {
            SystemLanguage::Ja => 0,
            SystemLanguage::Enus => 1,
            SystemLanguage::Fr => 2,
            SystemLanguage::De => 3,
            SystemLanguage::It => 4,
            SystemLanguage::Es => 5,
            SystemLanguage::Zhcn => 6,
            SystemLanguage::Ko => 7,
            SystemLanguage::Nl => 8,
            SystemLanguage::Pt => 9,
            SystemLanguage::Zhtw => 11,
            SystemLanguage::Engb => 12,
            SystemLanguage::Frca => 13,
            SystemLanguage::Es419 => 14,
            SystemLanguage::Zhhans => 15,
            SystemLanguage::Zhhant => 16,
        }
    }
}

impl From<u8> for SystemLanguage {
    fn from(lang: u8) -> Self {
        match lang {
            0 => Self::Ja,
            2 => Self::Fr,
            3 => Self::De,
            4 => Self::It,
            5 => Self::Es,
            6 => Self::Zhcn,
            7 => Self::Ko,
            8 => Self::Nl,
            9 => Self::Pt,
            11 => Self::Zhtw,
            12 => Self::Engb,
            13 => Self::Frca,
            14 => Self::Es419,
            15 => Self::Zhhans,
            16 => Self::Zhhant,
            _ => Self::Enus,
        }
    }
}
