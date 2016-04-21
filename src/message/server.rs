use enums::*;
use serde_json as json;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
pub struct PublicChannel {
    pub name: String,
    pub mode: ChannelMode,
    pub characters: i32,
}

#[derive(Deserialize, Debug)]
pub struct ORSDetails {
    pub name: String,
    pub characters: i32,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct UserObject {
    pub identity: String,
}

#[derive(Deserialize, Debug)]
pub enum Message {
    ADL {
        ops: Vec<String>,
    },
    AOP {
        character: String,
    },
    BRO {
        message: String,
    },
    CDS {
        channel: String,
        description: String,
    },
    CHA {
        channels: Vec<String>,
    },
    CIU {
        sender: String,
        title: String,
        name: String,
    },
    CBU {
        operator: String,
        channel: String,
        character: String,
    },
    CKU {
        operator: String,
        channel: String,
        character: String,
    },
    COA {
        character: String,
        channel: String,
    },
    COL {
        channel: String,
        oplist: Vec<String>,
    },
    CON {
        count: i32,
    },
    COR {
        character: String,
        channel: String,
    },
    CSO {
        character: String,
        channel: String,
    },
    CTU {
        operator: String,
        channel: String,
        length: i32,
        character: String,
    },
    DOP {
        character: String,
    },
    ERR {
        number: i32,
        message: String,
    },
    FKS {
        characters: Vec<String>,
        kinks: Vec<i32>,
    },
    FLN {
        character: String,
    },
    HLO {
        message: String,
    },
    ICH {
        users: Vec<UserObject>,
        channel: String,
        mode: ChannelMode,
    },
    IDN {
        character: String,
    },
    JCH {
        channel: String,
        character: UserObject,
        title: String,
    },
    KID(json::Value),
    LCH {
        channel: String,
        character: String,
    },
    LIS {
        characters: Vec<Vec<String>>,
    },
    NLN {
        identity: String,
        gender: Gender,
        status: CharacterStatus,
    },
    IGN(json::Value),
    FRL {
        characters: Vec<String>,
    },
    ORS {
        channels: Vec<ORSDetails>,
    },
    PIN,
    PRD(json::Value),
    PRI {
        character: String,
        message: String,
    },
    MSG {
        character: String,
        message: String,
        channel: String,
    },
    LRP {
        character: String,
        message: String,
        channel: String,
    },
    RLL(json::Value),
    RMO {
        mode: ChannelMode,
        channel: String,
    },
    RTB {
        #[serde(rename = "type")]
        _type: String,
        character: String,
    },
    SFC(json::Value),
    STA {
        status: CharacterStatus,
        character: String,
        statusmsg: String,
    },
    SYS {
        message: String,
        channel: Option<String>,
    },
    TPN {
        character: String,
        status: TypingStatus,
    },
    UPT {
        time: i64,
        starttime: i64,
        startstring: String,
        accepted: i64,
        channels: i64,
        users: i64,
        maxusers: i64,
    },
    VAR {
        variable: String,
        value: json::Value,
    },
}

#[derive(Debug)]
pub enum Error {
    JsonError(json::Error),
    InvalidMessage
}

impl ::std::convert::From<json::Error> for Error {
    fn from(error: json::Error) -> Error {
        Error::JsonError(error)
    }
}

impl Message {
    // TODO: Find a way to deserialize without allocating a BTreeMap
    fn deserialize(variant: &[u8], text: &[u8]) -> Result<Self, Error> {
        let mut map = BTreeMap::new();

        let variant = try!(
            String::from_utf8(Vec::from(variant))
            .map_err(|_| Error::InvalidMessage));

        if text != &[] {
            let data = try!(json::from_slice(text));
            map.insert(variant, data);
        } else {
            map.insert(variant, json::Value::Null);
        }

        Ok(try!(json::from_value(json::Value::Object(map))))
    }

    pub fn from_slice(message: &[u8]) -> Result<Self, Error> {
        if message.len() < 3 {
            Err(Error::InvalidMessage)
        } else {
            let text = if message.len() >= 4 { &message[4..] } else { &[] };
            Message::deserialize(&message[..3], text)
        }
    }
}
