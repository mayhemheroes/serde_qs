#![no_main]
use libfuzzer_sys::fuzz_target;
use serde::{Deserialize, Serialize};

const DEBUG: bool = false;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum PlainEnum {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Enum {
    A(u8),
    B(()),
    C(Vec<PlainEnum>),
    D(i128),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum FloatEnum {
    A(Enum),
    E(Option<f32>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Struct {
    _a: (),
    _b: u8,
    _c: Vec<Enum>,
    _d: (u128, i8, (), PlainEnum, String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct FloatStruct {
    _a: Struct,
    _b: f64,
}

macro_rules! round_trip {
    ($ty:ty, $data:ident, $equality:expr) => {{
        if DEBUG {
            println!("roundtripping {}", stringify!($ty));
        }

        let _x: Result<$ty, _> = serde_qs::from_str($data);
        
        #[cfg(full_roundtrip)]
        if let Ok(inner) = _x {
            if DEBUG {
                dbg!(&inner);
            }

            let ser = serde_qs::to_string(&inner).expect("a deserialized type should serialize");
            if DEBUG {
                dbg!(&ser);
            }

            let des: $ty = serde_qs::from_str(&ser).expect("a serialized type should deserialize");
            if DEBUG {
                dbg!(&des);
            }

            if $equality {
                assert_eq!(inner, des, "roundtripped object changed");
            }
        }
    }};
}

macro_rules! from_str {
    ($ty:ty, $data:ident, $equality:expr) => {{
        round_trip!($ty, $data, $equality);
        round_trip!(Vec<$ty>, $data, $equality);
        round_trip!(Option<$ty>, $data, $equality);
    }};
}

fuzz_target!(|data: &str| {
    if DEBUG {
        println!("fuzz input: {:?}", data);
    }

    from_str!(bool, data, true);
    from_str!(i8, data, true);
    from_str!(i16, data, true);
    from_str!(i32, data, true);
    from_str!(i64, data, true);
    from_str!(i128, data, true);
    from_str!(u8, data, true);
    from_str!(u16, data, true);
    from_str!(u32, data, true);
    from_str!(u64, data, true);
    from_str!(u128, data, true);
    from_str!(f32, data, false);
    from_str!(f64, data, false);
    from_str!(String, data, true);
    from_str!((), data, true);
    from_str!(PlainEnum, data, true);
    from_str!(Enum, data, true);
    from_str!(FloatEnum, data, false);
    from_str!(Struct, data, true);
    from_str!(FloatStruct, data, false);
});
