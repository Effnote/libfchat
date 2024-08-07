use crate::enums::*;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub enum Message {
    ACB {
        character: String,
    },
    AOP {
        character: String,
    },
    AWC {
        character: String,
    },
    BRO {
        message: String,
    },
    CBL {
        channel: String,
    },
    CBU {
        character: String,
        channel: String,
    },
    CCR {
        channel: String,
    },
    CDS {
        channel: String,
        description: String,
    },
    CHA,
    CIU {
        channel: String,
        character: String,
    },
    CKU {
        channel: String,
        character: String,
    },
    COA {
        channel: String,
        character: String,
    },
    COL {
        channel: String,
    },
    COR {
        channel: String,
        character: String,
    },
    CRC {
        channel: String,
    },
    CSO {
        character: String,
        channel: String,
    },
    CTU {
        channel: String,
        character: String,
        length: u8,
    },
    CUB {
        character: String,
        channel: String,
    },
    DOP {
        character: String,
    },
    FKS {
        kinks: Vec<i32>,
        genders: Vec<Gender>,
        orientations: Vec<Orientation>,
        languages: Vec<Language>,
        furryprefs: Vec<FurryPref>,
        roles: Vec<Role>,
    },
    IDN {
        method: IdnMethod,
        account: String,
        ticket: String,
        character: String,
        cname: String,
        cversion: String,
    },
    IGN(IgnEnum),
    JCH {
        channel: String,
    },
    KIK {
        character: String,
    },
    KIN {
        character: String,
    },
    LCH {
        channel: String,
    },
    LRP {
        channel: String,
        message: String,
    },
    MSG {
        channel: String,
        message: String,
    },
    ORS,
    PIN,
    PRI {
        recipient: String,
        message: String,
    },
    PRO {
        character: String,
    },
    RLL {
        channel: String,
        dice: String,
    },
    RLD {
        save: String,
    },
    RMO {
        channel: String,
        mode: ChannelMode,
    },
    RST {
        channel: String,
        status: ChannelStatus,
    },
    RWD {
        character: String,
    },
    SFC {
        // action is always "report"
        action: SfcAction,
        report: String,
        character: String,
    },
    STA {
        status: CharacterStatus,
        statusmsg: String,
    },
    TMO {
        character: String,
        time: u32,
        reason: String,
    },
    TPN {
        character: String,
        status: TypingStatus,
    },
    UBN {
        character: String,
    },
    UPT,
}

impl Message {
    // Serialize the Message into the final format.
    // Theoretically, this method should never panic, unless
    // breaking changes in serde happen.
    pub fn to_string(&self) -> String {
        use serde_json::Value;

        let value = serde_json::to_value(self).unwrap();
        match value {
            Value::String(variant) => variant,
            Value::Object(object) => {
                if let Some((variant, value)) = object.into_iter().nth(0) {
                    return format!("{} {}", variant, value);
                } else {
                    panic!("Error: serialization: Empty top object.");
                }
            }
            _ => panic!(
                "Error: serialization: expected Value::Object, got: {:?}",
                value
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pin_serialization() {
        assert_eq!(super::Message::PIN.to_string(), "PIN");
    }

    #[test]
    fn ign_serialization() {
        use crate::enums::IgnEnum;
        super::Message::IGN(IgnEnum::Add {
            character: "foo_bar".to_string(),
        })
        .to_string();
    }
}
