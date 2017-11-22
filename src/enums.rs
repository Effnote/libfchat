use serde;

macro_rules! make_enum {
    (enum $name: ident ; $($variant: ident : $string: expr),+ ; $($variant2: ident),* ) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum $name {
            $( $variant, )*
            $( $variant2, )*
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: serde::Serializer
            {
                serializer.serialize_str(
                    match *self {
                        $($name::$variant => $string,)*
                        $($name::$variant2 => stringify!($variant2),)*
                    }
                )
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: serde::Deserializer<'de>
            {
                let string = String::deserialize(deserializer)?;
                Ok(match &*string {
                        $($string => $name::$variant,)*
                        $(stringify!($variant2) => $name::$variant2,)*
                        _ => return Err(serde::de::Error::unknown_variant(
                            &*string,
                            &[$($string,)* $(stringify!($variant2),)*]))
                })
            }
        }
    }
}

make_enum!(enum Gender;
MaleHerm: "Male-Herm",
Cuntboy: "Cunt-boy";
Male, Female, Transgender, Herm, Shemale, None);

make_enum!(enum Orientation;
BiMalePref: "Bi - male preference",
BiFemPref: "Bi - female preference",
BiCurious: "Bi-curious";
Straight, Gay, Bisexual, Asexual, Unsure, Pansexual);

make_enum!(enum Language;
// Due to macro_rules! limitations,
// at least one element must have a string specified
Other: "Other";
Dutch, English, French, Spanish, German, Russian, Chinese, Japanese,
Portugese, Korean, Arabic, Italian, Swedish);

make_enum!(enum FurryPref;
JustHuman: "No furry characters, just humans",
JustFurry: "No humans, just furry characters",
PreferHuman: "Furries ok, Humans Preferred",
PreferFurry: "Humans ok, Furries Preferred",
Both: "Furs and / or humans";);

make_enum!(enum Role;
AlwaysDom: "Always dominant",
UsualDom: "Usually dominant",
UsualSub: "Usually submissive",
AlwaysSub: "Always submissive";
Switch, None);

make_enum!(enum IgnoreAction;
Add: "add",
Delete: "delete",
Notify: "notify",
List: "list";);

make_enum!(enum ChannelMode;
Chat: "chat",
Ads: "ads",
Both: "both";);

make_enum!(enum ChannelStatus;
Public: "public",
Private: "private";);

make_enum!(enum CharacterStatus;
Online: "online",
Looking: "looking",
Busy: "busy",
DND: "dnd",
Idle: "idle",
Away: "away",
Crown: "crown";);

make_enum!(enum TypingStatus;
Clear: "clear",
Paused: "paused",
Typing: "typing";);

make_enum!(enum SfcAction;
Report: "report";);

make_enum!(enum IdnMethod;
Ticket: "ticket";);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IgnEnum {
    Add { character: String },
    Delete { character: String },
    Notify { character: String },
    List,
}

impl serde::Serialize for IgnEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        use self::IgnEnum::*;
        let values = match *self {
            Add { ref character } => vec![("action", "add"), ("character", character)],
            Delete { ref character } => vec![("action", "delete"), ("character", character)],
            Notify { ref character } => vec![("action", "notify"), ("character", character)],
            List => vec![("action", "list")],
        };
        let mut map = serializer.serialize_map(Some(values.len()))?;
        for (k, v) in values {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
