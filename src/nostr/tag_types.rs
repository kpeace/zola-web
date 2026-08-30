use nostr_sdk::{TagKind, SingleLetterTag, Alphabet};

// Root tags (uppercase) A
pub const ROOT_A: TagKind = TagKind::SingleLetter(SingleLetterTag {
    character: Alphabet::A,
    uppercase: true,
});

// E
pub const ROOT_E: TagKind = TagKind::SingleLetter(SingleLetterTag {
    character: Alphabet::E,
    uppercase: true,
});

// Reply tags (lowercase) a
pub const REPLY_A: TagKind = TagKind::SingleLetter(SingleLetterTag {
    character: Alphabet::A,
    uppercase: false,
});

// e
pub const REPLY_E: TagKind = TagKind::SingleLetter(SingleLetterTag {
    character: Alphabet::E,
    uppercase: false,
});

// Other useful tags c
pub const CLIENT: TagKind = TagKind::SingleLetter(SingleLetterTag {
    character: Alphabet::C,
    uppercase: false,
});

// p
pub const PUBKEY: TagKind = TagKind::SingleLetter(SingleLetterTag {
    character: Alphabet::P,
    uppercase: false,
});