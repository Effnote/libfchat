use serde;

macro_rules! make_enum {
    (enum $name: ident ; $($variant: ident : $string: expr),+ ; $($variant2: ident),* ) => {
        #[derive(Debug)]
        pub enum $name {
            $( $variant, )*
            $( $variant2, )*
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
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

        impl serde::Deserialize for $name {
            fn deserialize<D>(deserializer: &mut D) -> Result<$name, D::Error>
                where D: serde::Deserializer
            {
                let string = try!(String::deserialize(deserializer));
                Ok(match &*string {
                        $($string => $name::$variant,)*
                        $(stringify!($variant2) => $name::$variant2,)*
                        _ => return Err(serde::de::Error::invalid_value(&*string))
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

#[derive(Debug)]
pub enum IgnEnum<'a> {
    Add { character: &'a str },
    Delete { character: &'a str },
    Notify { character: &'a str },
    List,
}

impl<'a> serde::Serialize for IgnEnum<'a> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        use self::IgnEnum::*;
        let mut map_state = try!(serializer.serialize_map(None));
        match *self {
            Add { character } => {
                try!(serializer.serialize_map_key(&mut map_state, "action"));
                try!(serializer.serialize_map_value(&mut map_state, "add"));
                try!(serializer.serialize_map_key(&mut map_state, "character"));
                try!(serializer.serialize_map_value(&mut map_state, character));
            }
            Delete { character } => {
                try!(serializer.serialize_map_key(&mut map_state, "action"));
                try!(serializer.serialize_map_value(&mut map_state, "delete"));
                try!(serializer.serialize_map_key(&mut map_state, "character"));
                try!(serializer.serialize_map_value(&mut map_state, character));
            }
            Notify { character } => {
                try!(serializer.serialize_map_key(&mut map_state, "action"));
                try!(serializer.serialize_map_value(&mut map_state, "notify"));
                try!(serializer.serialize_map_key(&mut map_state, "character"));
                try!(serializer.serialize_map_value(&mut map_state, character));
            }
            List => {
                try!(serializer.serialize_map_key(&mut map_state, "action"));
                try!(serializer.serialize_map_value(&mut map_state, "list"));
            }
        };
        serializer.serialize_map_end(map_state)
    }
}
