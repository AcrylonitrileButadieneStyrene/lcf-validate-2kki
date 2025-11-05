#[derive(Clone, Copy, Default, clap::ValueEnum, num_enum::FromPrimitive)]
#[repr(u32)]
pub enum CodePage {
    #[default]
    Ascii = 0,
    Eastern,
    Cyrillic,
    ShiftJIS,
    Big5,
    __LENGTH,
}

impl CodePage {
    pub const fn to_encoding(self) -> &'static encoding_rs::Encoding {
        match self {
            Self::Ascii => encoding_rs::WINDOWS_1252,
            Self::Eastern => encoding_rs::WINDOWS_1250,
            Self::Cyrillic => encoding_rs::WINDOWS_1251,
            Self::ShiftJIS => encoding_rs::SHIFT_JIS,
            Self::Big5 => encoding_rs::BIG5,
            Self::__LENGTH => unreachable!(),
        }
    }

    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Ascii => "ASCII",
            Self::Eastern => "European",
            Self::Cyrillic => "Cyrillic",
            Self::ShiftJIS => "Japanese",
            Self::Big5 => "Chinese",
            Self::__LENGTH => unreachable!(),
        }
    }
}
