//! # AEI TAG PARSER
//!
//! This library provides a way to deserialize RFID AEI tags used in railway industry to identify wagons. It can be used both a CLI util or a library
//!
//! # Usage
//!
//! ## CLI
//!
//! Deserialize one or multiple tags passed as parameters :
//! ```bash
//! # One tag :
//! $ aei-tag-parser 9EA488C030426A179000000000000331
//! # Output :
//! # 9EA488C030426A179000000000000331 : Initials : IOCC      Car number : 3088
//! # Multiple tags :
//! $ aei-tag-parser 2F3E06C007DB1E139000000000000331 9EA488C030426A179000000000000331 9EA488C5320CC01B9000000000000331
//! # Output :
//! # 2F3E06C007DB1E139000000000000331 : Initials : QNSL      Car number : 502
//! # 9EA488C030426A179000000000000331 : Initials : IOCC      Car number : 3088
//! # 9EA488C5320CC01B9000000000000331 : Initials : IOCC      Car number : 85123
//! ```
//!
//! Tag deserialization from a file :
//! ```bash
//! # tags.txt content :
//! $ cat tags.txt
//! # Output :
//! # 2F3E06C007DB1E139000000000000331
//! # 9EA488C030426A179000000000000331
//! # 9EA488C5320CC01B9000000000000331
//! $ aei-tag-parser -f test.txt
//! # Output :
//! # 2F3E06C007DB1E139000000000000331 : Initials : QNSL      Car number : 502
//! # 9EA488C030426A179000000000000331 : Initials : IOCC      Car number : 3088
//! # 9EA488C5320CC01B9000000000000331 : Initials : IOCC      Car number : 85123
//! ```
//!
//! Tag deserialization from a UNIX pipe :
//! ```bash
//! # tags.txt content :
//! $ cat tags.txt
//! # Output :
//! # 2F3E06C007DB1E139000000000000331
//! # 9EA488C030426A179000000000000331
//! # 9EA488C5320CC01B9000000000000331
//! $ cat tags.txt | aei-tag-parser
//! # Output :
//! # 2F3E06C007DB1E139000000000000331 : Initials : QNSL      Car number : 502
//! # 9EA488C030426A179000000000000331 : Initials : IOCC      Car number : 3088
//! # 9EA488C5320CC01B9000000000000331 : Initials : IOCC      Car number : 85123
//! ```
//!
//! ## Librairie
//!
//! This project can also be used as an external library. Documentation is available here : [https://docs.rs/aei_tag_parser/1.0.0/aei_tag_parser/index.html](https://docs.rs/aei_tag_parser/1.0.0/aei_tag_parser/index.html)
//!
//! ### Usage
//!
//! ```rust
//! let tag_str : String = String::from("9EA488C030426A179000000000000331");
//! let tag : AEITagData = AEITagData::new(&tag_str);
//!
//! println!("Tag {} content is : \r\n\tInitials: {}\r\n\tCar number: {}", &tag_str, tag.equipment_initial(), tag.car_number());
//! ```
//!
//! # Install
//!
//! To install the CLI util, you must have Rust installed (cf [RustUp](https://rustup.rs/)).
//!
//! Then you can simply do :
//! ```bash
//! $ cargo install aei_tag_parser
//! ```

/* Data field descriptions for the Railcar tag
// +==========================+===============+====================+===============+===============+===============================================+
// |          Entry           | Bits required | Tag Data Sequences | Minimum Value | Maximum Value |                     Unit                      |
// +==========================+===============+====================+===============+===============+===============================================+
// | Equipment Group Code     |             5 | 0-4                | 1             | 31            | Type Code                                     |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | Tag Type                 |             2 | 5-6                | 1             | 4             | Type Code                                     |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | Equipment Initial (Mark) |            19 | 7-25               | A             | ZZZZ          | Alpha                                         |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | Car Number               |            20 | 26-45              | 0             | 999999        | Numeric                                       |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | Side Indicator Code      |             1 | 46                 | 0             | 1             | Side Code                                     |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | Length                   |            12 | 94-96              | 0             | 4095          | Decimeters                                    |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | 47-55                    |             0 | 1343               | Feet          |               |                                               |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | Number of Axles          |             5 | 56-59, 64          | 1             | 32            | Axles                                         |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | First Checksum           |             2 | 60-61              |               |               |                                               |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | Reserved Frame Marker    |             2 | 62.63              |               |               |                                               |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | Bearing Type Code        |             3 | 65-67              | 0             | 7             | Type Code                                     |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | Platform Identifier Code |             4 | 68-71              | 0             | 15            | Platform Code                                 |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | Spare                    |            22 | 81-93              |               |               | Available for Owner's Use                     |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | Reserver                 |             9 | 97-105             |               |               | Reserved for Future Use by AAR                |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | Security                 |            12 | 106-117            |               |               | Reserved for Security or Limited Owner's  Use |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | Data Format Code         |             6 | 118-123            | 0             | 63            | Format Code                                   |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | Second Checl Sum         |             2 | 124-125            |               |               |                                               |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
// | Frame Marker             |             2 | 126-127            |               |               |                                               |
// +--------------------------+---------------+--------------------+---------------+---------------+-----------------------------------------------+
*/
extern crate hex;
use hex::FromHexError;
use serde::{Deserialize, Serialize};
use std::{
    error,
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub enum NewTagError {
    HexParsing(FromHexError),
}

impl From<FromHexError> for NewTagError {
    fn from(err: FromHexError) -> Self {
        NewTagError::HexParsing(err)
    }
}

impl Display for NewTagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            NewTagError::HexParsing(..) => {
                write!(
                    f,
                    "the provided string couldn't be parsed as an hexadecimal number"
                )
            }
        }
    }
}

impl error::Error for NewTagError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            NewTagError::HexParsing(ref e) => Some(e),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Side {
    LEFT,
    RIGHT,
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Side::LEFT => write!(f,"Left"),
            Side::RIGHT => write!(f,"Right"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AEITagData {
    raw: [u8; 16],
    equipment_group_code: u8,
    tag_type_code: u8,
    equipment_initial_code: u32,
    equipment_initial: String,
    car_number: u32,
    side_indicator: Side,
    length_dm: u16,
    number_axles: u8,
}

impl AEITagData {
    pub fn new(tag: &str) -> Result<AEITagData, NewTagError> {
        let mut raw = [0u8; 16];
        hex::decode_to_slice(tag, &mut raw)?;

        let equipment_group_code = AEITagData::parse_eqp_group(&raw);
        let tag_type_code = AEITagData::parse_tag_type(&raw);
        let equipment_initial_code = AEITagData::parse_eqp_initial(&raw);
        let car_number = AEITagData::parse_car_number(&raw);
        let side_indicator = AEITagData::parse_side(&raw);
        let length_dm = AEITagData::parse_length(&raw);
        let number_axles = AEITagData::parse_axles(&raw);

        Ok(AEITagData {
            raw,
            equipment_group_code,
            tag_type_code,
            equipment_initial_code,
            equipment_initial: AEITagData::deserialize_equipement_initial(equipment_initial_code),
            car_number,
            side_indicator,
            length_dm,
            number_axles,
        })
    }

    /// Parse the equipement group code from the raw tag data.
    /// The value must be contained in \[0;31]
    fn parse_eqp_group(raw: &[u8; 16]) -> u8 {
        (raw[0] & 0xF8) >> 3
    }

    /// Equipment group code value
    pub fn equipment_group_code(&self) -> u8 {
        self.equipment_group_code
    }

    fn parse_equipment_group_code(code: u8) -> String {
        match code {
            0 => String::from("Other"),
            1 => String::from("Railcar cover"),
            4 => String::from("Train number tag (locomotive variable data)"),
            5 => String::from("Locomotive"),
            6 => String::from("End-of-train device"),
            8 => String::from("Generator set"),
            10 => String::from("Intermodal container"),
            12 => String::from("Marker tags"),
            14 => String::from("Reserved (formerly nonrevenue rail)"),
            17 => String::from("Tractor (power)"),
            18 => String::from("Straight truck"),
            19 => String::from("Railcar"),
            20 => String::from("Dolly"),
            21 => String::from("Trailer"),
            24 => String::from("Rail-compatible multimodal equipment"),
            27 => String::from("Chassis"),
            28 => String::from("Passive alarm tag"),
            31 => String::from("Experimental use/other"),
            2 | 3 | 7 | 9 | 11 | 13 | 15 | 16 | 22 | 23 | 25 | 26 | 29 | 30 => {
                String::from("Reserved")
            },
            _ => panic!("You should never reach this point. The value of equipment group code is meant to be contained in [0; 31]")
        }
    }

    /// Equipment group value
    pub fn equipment_group(&self) -> String {
        AEITagData::parse_equipment_group_code(self.equipment_group_code)
    }

    /// Parse the tag type code
    fn parse_tag_type(raw: &[u8; 16]) -> u8 {
        (raw[0] & 0b0000_0110) >> 1
    }

    // Tag type code value
    pub fn tag_type(&self) -> u8 {
        self.tag_type_code
    }

    /// Parse the equipement initial code from the raw tag data
    fn parse_eqp_initial(raw: &[u8; 16]) -> u32 {
        let mut value = [0u8; 4];
        value[0] = raw[0] & 0x01;
        value[1] = raw[1];
        value[2] = raw[2];
        value[3] = raw[3] & 0xC0;

        u32::from_be_bytes(value) >> 6
    }

    /// Equipement initial code
    pub fn equipment_initial_code(&self) -> u32 {
        self.equipment_initial_code
    }

    fn deserialize_equipement_initial(equipment_initial_code: u32) -> String {
        // The equipment initial code is 19 bits long. Therefore, we can safely assume that the max value is going to be 2^19 - 1 = 524287
        // Given that information, we would handle the risk of overflow when converting the result from u32 to u8
        let n1 = equipment_initial_code / 27u32.pow(3);
        let n2 = (equipment_initial_code - n1 * 27u32.pow(3)) / 27u32.pow(2);
        let n3 = (equipment_initial_code - n1 * 27u32.pow(3) - n2 * 27u32.pow(2)) / 27;
        let n4 = equipment_initial_code - n1 * 27u32.pow(3) - n2 * 27u32.pow(2) - n3 * 27;

        // For C1, A = 0, ..., Z = 25
        let c1 = char::try_from(n1 + 65).unwrap();
        // For C2, C3, C4, Blank / Space = 0, A = 1, ..., Z = 26
        let c2 = char::try_from(if n2 == 0 { 32 } else { n2 + 64 }).unwrap();
        let c3 = char::try_from(if n3 == 0 { 32 } else { n3 + 64 }).unwrap();
        let c4 = char::try_from(if n4 == 0 { 32 } else { n4 + 64 }).unwrap();

        vec![c1, c2, c3, c4].iter().collect()
    }

    /// Equipment initial value
    pub fn equipment_initial(&self) -> String {
        AEITagData::deserialize_equipement_initial(self.equipment_initial_code)
    }

    /// Parse the car number from the raw tag data
    fn parse_car_number(raw: &[u8; 16]) -> u32 {
        let mut value = [0u8; 4];
        value[1] = raw[3];
        value[2] = raw[4];
        value[3] = raw[5];

        let value = u32::from_be_bytes(value);
        (value >> 2) & 0x00_0F_FF_FF
    }

    /// Car number value
    pub fn car_number(&self) -> u32 {
        self.car_number
    }

    /// Parse the side indicator from the raw tag data
    fn parse_side(raw: &[u8; 16]) -> Side {
        if raw[5] & 0b0000_0010 == 0 {
            Side::LEFT
        } else {
            Side::RIGHT
        }
    }

    /// Side indicator value
    pub fn side_indicator(&self) -> Side {
        self.side_indicator
    }

    /// Parse the railcar length from the raw tag data
    fn parse_length(raw: &[u8; 16]) -> u16 {
        let mut value = [0u8; 2];
        value[0] = ((raw[11] & 0x03) << 2) | ((raw[12] & 0x80) >> 6) | (raw[5] & 0x01);
        value[1] = raw[6];

        u16::from_be_bytes(value)
    }

    /// Length of the railcar in decimeters
    pub fn length_dm(&self) -> u16 {
        self.length_dm
    }

    /// Length of the railcar in feets
    pub fn length_ft(&self) -> u16 {
        (f64::try_from(self.length_dm).unwrap() * 0.328084).round() as u16
    }

    /// Parse the number of axles from the raw tag data
    fn parse_axles(raw: &[u8; 16]) -> u8 {
        (((raw[7] >> 3) & 0x1E) | (raw[8] >> 7)) + 1
    }

    /// Number of axles
    pub fn number_axles(&self) -> u8 {
        self.number_axles
    }

    /// Raw value of the tag
    pub fn raw(&self) -> &[u8; 16] {
        &self.raw
    }

    /// Returns a short string which describes the datas
    pub fn to_short_string(&self) -> String {
        format!(
            "Raw : {}\tInitials : {}\tCar number : {}\tEquipment type : {}({})\tSide : {}",
            hex::encode_upper(&self.raw),
            self.equipment_initial(),
            self.car_number,
            self.equipment_group(),
            self.equipment_group_code,
            self.side_indicator
        )
    }

    // Returns a string in CSV format which describes the tag
    pub fn to_csv(&self) -> String {
        format!(
            "{};{};{};{};{};{}",
            hex::encode_upper(&self.raw),
            self.equipment_initial(),
            self.car_number,
            self.equipment_group(),
            self.equipment_group_code,
            self.side_indicator
        )
    }

    /// Check if 2 tags belong to the same wagon. It compare the equipment initials code,
    /// the equipment group code and the car number however, the side is ignored
    pub fn is_same_wagon(&self, tag: AEITagData) -> bool {
        self.car_number == tag.car_number
            && self.equipment_group_code == tag.equipment_group_code
            && self.equipment_initial_code == tag.equipment_initial_code
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    /// QNSL 502 RIGHT Locomotive(5) 94' 0" 4 axles
    static TAG1: &str = "2F3E06C007DB1E139000000000000331";
    /// IOCC 3088 RIGHT Railcar(19) 35' 0" 4 axles
    static TAG2: &str = "9EA488C030426A179000000000000331";
    /// IOCC 85123 LEFT Railcar(19) 63' 0" 4 axles
    static TAG3: &str = "9EA488C5320CC01B9000000000000331";

    #[test]
    fn valid_group_code() {
        assert_eq!(AEITagData::new(TAG1).unwrap().equipment_group_code(), 5);
        assert_eq!(AEITagData::new(TAG2).unwrap().equipment_group_code(), 19);
    }

    #[test]
    fn valid_group() {
        assert_eq!(
            AEITagData::new(TAG1).unwrap().equipment_group(),
            "Locomotive"
        );
        assert_eq!(AEITagData::new(TAG2).unwrap().equipment_group(), "Railcar");
    }

    #[test]
    fn valid_group_code_parsing() {
        assert_eq!(AEITagData::parse_equipment_group_code(0), "Other");
        assert_eq!(AEITagData::parse_equipment_group_code(1), "Railcar cover");
        assert_eq!(AEITagData::parse_equipment_group_code(2), "Reserved");
        assert_eq!(AEITagData::parse_equipment_group_code(3), "Reserved");
        assert_eq!(
            AEITagData::parse_equipment_group_code(4),
            "Train number tag (locomotive variable data)"
        );
        assert_eq!(AEITagData::parse_equipment_group_code(5), "Locomotive");
        assert_eq!(
            AEITagData::parse_equipment_group_code(6),
            "End-of-train device"
        );
        assert_eq!(AEITagData::parse_equipment_group_code(7), "Reserved");
        assert_eq!(AEITagData::parse_equipment_group_code(8), "Generator set");
        assert_eq!(AEITagData::parse_equipment_group_code(9), "Reserved");
        assert_eq!(
            AEITagData::parse_equipment_group_code(10),
            "Intermodal container"
        );
        assert_eq!(AEITagData::parse_equipment_group_code(11), "Reserved");
        assert_eq!(AEITagData::parse_equipment_group_code(12), "Marker tags");
        assert_eq!(AEITagData::parse_equipment_group_code(13), "Reserved");
        assert_eq!(
            AEITagData::parse_equipment_group_code(14),
            "Reserved (formerly nonrevenue rail)"
        );
        assert_eq!(AEITagData::parse_equipment_group_code(15), "Reserved");
        assert_eq!(AEITagData::parse_equipment_group_code(16), "Reserved");
        assert_eq!(
            AEITagData::parse_equipment_group_code(17),
            "Tractor (power)"
        );
        assert_eq!(AEITagData::parse_equipment_group_code(18), "Straight truck");
        assert_eq!(AEITagData::parse_equipment_group_code(19), "Railcar");
        assert_eq!(AEITagData::parse_equipment_group_code(20), "Dolly");
        assert_eq!(AEITagData::parse_equipment_group_code(21), "Trailer");
        assert_eq!(AEITagData::parse_equipment_group_code(22), "Reserved");
        assert_eq!(AEITagData::parse_equipment_group_code(23), "Reserved");
        assert_eq!(
            AEITagData::parse_equipment_group_code(24),
            "Rail-compatible multimodal equipment"
        );
        assert_eq!(AEITagData::parse_equipment_group_code(25), "Reserved");
        assert_eq!(AEITagData::parse_equipment_group_code(26), "Reserved");
        assert_eq!(AEITagData::parse_equipment_group_code(27), "Chassis");
        assert_eq!(
            AEITagData::parse_equipment_group_code(28),
            "Passive alarm tag"
        );
        assert_eq!(AEITagData::parse_equipment_group_code(29), "Reserved");
        assert_eq!(AEITagData::parse_equipment_group_code(30), "Reserved");
        assert_eq!(
            AEITagData::parse_equipment_group_code(31),
            "Experimental use/other"
        );
    }

    #[test]
    #[should_panic]
    fn invalid_group_code_parsing() {
        AEITagData::parse_equipment_group_code(32);
    }

    #[test]
    fn valid_tag_type() {
        assert_eq!(AEITagData::new(TAG1).unwrap().tag_type(), 3);
    }

    #[test]
    fn valid_equipement_initial() {
        assert_eq!(AEITagData::new(TAG1).unwrap().equipment_initial(), "QNSL");
        assert_eq!(AEITagData::new(TAG2).unwrap().equipment_initial(), "IOCC");
    }

    #[test]
    fn valid_equipment_initial_code() {
        assert_eq!(
            AEITagData::new(TAG1).unwrap().equipment_initial_code(),
            325659
        );
        assert_eq!(
            AEITagData::new(TAG2).unwrap().equipment_initial_code(),
            168483
        );
    }

    #[test]
    fn valid_car_number() {
        assert_eq!(AEITagData::new(TAG1).unwrap().car_number(), 502);
        assert_eq!(AEITagData::new(TAG2).unwrap().car_number(), 3088);
        assert_eq!(AEITagData::new(TAG3).unwrap().car_number(), 85123);
    }

    #[test]
    fn valid_side_indicator() {
        assert_eq!(AEITagData::new(TAG1).unwrap().side_indicator(), Side::RIGHT);
        assert_eq!(AEITagData::new(TAG3).unwrap().side_indicator(), Side::LEFT);
    }

    #[test]
    fn valid_length_dm() {
        assert_eq!(AEITagData::new(TAG1).unwrap().length_dm(), 286);
        assert_eq!(AEITagData::new(TAG2).unwrap().length_dm(), 106);
        assert_eq!(AEITagData::new(TAG3).unwrap().length_dm(), 192);
    }

    #[test]
    fn valid_length_ft() {
        assert_eq!(AEITagData::new(TAG1).unwrap().length_ft(), 94);
        assert_eq!(AEITagData::new(TAG2).unwrap().length_ft(), 35);
        assert_eq!(AEITagData::new(TAG3).unwrap().length_ft(), 63);
    }

    #[test]
    fn valid_number_axles() {
        assert_eq!(AEITagData::new(TAG1).unwrap().number_axles(), 4);
    }

    #[test]
    fn invalid_hex_odd_length() {
        let result = AEITagData::new("00F");

        let err = result.unwrap_err();
        assert_eq!(
            err.to_string(),
            "the provided string couldn't be parsed as an hexadecimal number"
        );
        assert_eq!(
            err.source().unwrap().to_string(),
            hex::FromHexError::OddLength.to_string()
        );
    }

    #[test]
    fn invalid_hex_invalid_characters() {
        let result = AEITagData::new("9EA488C5320CC01B900000000000033T");

        let err = result.unwrap_err();
        assert_eq!(
            err.to_string(),
            "the provided string couldn't be parsed as an hexadecimal number"
        );
        assert_eq!(
            err.source().unwrap().to_string(),
            hex::FromHexError::InvalidHexCharacter { c: 'T', index: 31 }.to_string()
        );
    }

    #[test]
    fn invalid_hex_invalid_length() {
        let result = AEITagData::new("00");

        let err = result.unwrap_err();
        assert_eq!(
            err.to_string(),
            "the provided string couldn't be parsed as an hexadecimal number"
        );
        assert_eq!(
            err.source().unwrap().to_string(),
            hex::FromHexError::InvalidStringLength.to_string()
        );
    }

    #[test]
    fn raw_value() {
        let tag = AEITagData::new(TAG1).unwrap();
        let mut raw_tag = [0u8; 16];
        hex::decode_to_slice("2F3E06C007DB1E139000000000000331", &mut raw_tag).unwrap();

        tag.raw()
            .iter()
            .enumerate()
            .for_each(|(index, value)| assert_eq!(value, &raw_tag[index]));
    }

    #[test]
    fn test_short_string() {
        let tag = AEITagData::new(TAG1).unwrap();

        assert_eq!(tag.to_short_string(), "Initials : QNSL\tCar number : 502");
    }
}
