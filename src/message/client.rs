use enums::*;

#[derive(Serialize, Debug)]
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
    IGN(IgnEnum<'a>),
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
        action: SfcAction,
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
    // Serialize the Message into the final format.
    // Theoretically, this method should never panic, unless
    // breaking changes in serde happen.
    pub fn to_string(&self) -> String {
        use serde_json;

        let value = serde_json::to_value(self);
        let object = value.as_object()
            .expect("Error: serialization: expected Value::Object, got something else");
        if let Some((variant, value)) = object.into_iter().nth(0) {
            if value.is_object() {
                return format!("{} {:?}\n", variant, value);
            } else {
                return format!("{}\n", variant);
            }
        } else {
            panic!("Error: serialization: Empty top object.");
        }
    }
}

#[test]
fn fks_serialize() {
    use enums::Gender::*;
    use enums::Orientation::*;
    use enums::Language::*;
    use enums::FurryPref::*;
    use enums::Role;

    let string = Message::FKS {
        kinks: &[51, 425],
        genders: &[MaleHerm, Cuntboy, Female],
        orientations: &[Straight, Bisexual, BiFemPref],
        languages: &[Dutch, French, Other],
        furryprefs: &[JustHuman, JustFurry],
        roles: &[Role::AlwaysDom, Role::AlwaysSub, Role::Switch],
    }.to_string();

    assert_eq!(
        string,
        "FKS {\"furryprefs\":[\"No furry characters, just humans\"\
        ,\"No humans, just furry characters\"]\
        ,\"genders\":[\"Male-Herm\",\"Cunt-boy\",\"Female\"],\
        \"kinks\":[51,425],\
        \"languages\":[\"Dutch\",\"French\",\"Other\"],\
        \"orientations\":[\"Straight\",\"Bisexual\",\"Bi - female preference\"],\
        \"roles\":[\"Always dominant\",\"Always submissive\",\"Switch\"]}\n");
}
