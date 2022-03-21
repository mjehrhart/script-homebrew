///Token field and description for tokenizer (lexer)
#[allow(dead_code,clippy::upper_case_acronyms,clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum TokenKind {
    Comment,
    Comment_InLine,
    Comment_Block_Start,
    Comment_Block_Stop,
    CRLF,

    Digital_Float(String), 
    Digit(u32),
    EOF,
    KeyWord(String),
    Latin(char),
    Letter(String),
    Number(String),
    Punctuation(char),
    Temp(String),
    Variable(String),
    WhiteSpace,
    Word(String),
    Undefined,
}

///Licenses type for brew create field 'License'
#[allow(dead_code,clippy::upper_case_acronyms,clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Copy, Clone, Eq)]
enum License {
    AppacheLicense2,
    GNUGeneralPublicLicenseV3,
    MITLicense,
    BSD2ClauseSimplifiedLicense,
    BSD3ClauseNewOrRevisedLicense,
    BoostSoftwareLicense1,
    CreativeCommonsZeroV1Universal,
    EclipsePublicLicense2,
    GNUAfferoGenralPublicLicenseV3,
    GNUGeneralPublicLicenseV2,
    GNULesserGeneralPublicLicenseV2_1,
    MozillaPublicLicense2,
    TheUnlicense,
}
