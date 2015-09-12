use enums::*;

pub enum Message<'a> {
    ACB {
        character: &'a str,
    },
    AOP {
        character: &'a str,
    },
    AWC {
        character: &'a str,
    },
    BRO {
        message: &'a str,
    },
    CBL {
        channel: &'a str,
    },
    CBU {
        character: &'a str,
        channel: &'a str,
    },
    CCR {
        channel: &'a str,
    },
    CDS {
        channel: &'a str,
        description: &'a str,
    },
    CHA,
    CIU {
        channel: &'a str,
        character: &'a str,
    },
    CKU {
        channel: &'a str,
        character: &'a str,
    },
    COA {
        channel: &'a str,
        character: &'a str,
    },
    COL {
        channel: &'a str,
    },
    COR {
        channel: &'a str,
        character: &'a str,
    },
    CRC {
        channel: &'a str,
    },
    CSO {
        character: &'a str,
        channel: &'a str,
    },
    CTU {
        channel: &'a str,
        character: &'a str,
        length: u8
    },
    CUB {
        character: &'a str,
        channel: &'a str,
    },
    DOP {
        character: &'a str,
    },
    // Test this one
    FKS {
        kinks: &'a [i32],
        genders: &'a [Gender],
        orientations: &'a [Orientation],
        languages: &'a [Language],
        furryprefs: &'a [FurryPref],
        roles: &'a [Role],
    },
    IDN {
        method: &'a str,
        account: &'a str,
        ticket: &'a str,
        character: &'a str,
        cname: &'a str,
        version: &'a str,
    },
    // TODO: Handle IGN
    JCH {
        channel: &'a str,
    },
    KIK {
        character: &'a str,
    },
    KIN {
        character: &'a str,
    },
    LCH {
        channel: &'a str,
    },
    MSG {
        channel: &'a str,
        message: &'a str,
    },
    ORS,
    PIN,
    PRI {
        recipient: &'a str,
        message: &'a str,
    },
    PRO {
        character: &'a str,
    },
    RLL {
        channel: &'a str,
        dice: &'a str,
    },
    RLD {
        save: &'a str,
    },
    RMO {
        channel: &'a str,
        mode: ChannelMode,
    },
    RST {
        channel: &'a str,
        status: ChannelStatus,
    },
    RWD {
        character: &'a str,
    },
    SFC {
        // action is always "report"
        action: &'a str,
        report: &'a str,
        character: &'a str,
    },
    STA {
        status: CharacterStatus,
        statusmsg: &'a str,
    },
    TMO {
        character: &'a str,
        time: u32,
        reason: &'a str,
    },
    TPN {
        character: &'a str,
        status: TypingStatus
    },
    UBN {
        character: &'a str,
    },
    UPT,
}

impl<'a> Message<'a> {
    pub fn to_string(&self) -> String {
        unimplemented!();
    }
}
