# AEI TAG PARSER

This library provides a way to deserialize RFID AEI tags used in railway industry to identify wagons. It can be used both a CLI util or a library

# Usage 

## CLI

Deserialize one or multiple tags passed as parameters :
```bash 
# One tag :
$ aei-tag-parser 9EA488C030426A179000000000000331
# Output : 
# 9EA488C030426A179000000000000331 : Initials : IOCC      Car number : 3088
# Multiple tags :
$ aei-tag-parser 2F3E06C007DB1E139000000000000331 9EA488C030426A179000000000000331 9EA488C5320CC01B9000000000000331
# Output : 
# 2F3E06C007DB1E139000000000000331 : Initials : QNSL      Car number : 502
# 9EA488C030426A179000000000000331 : Initials : IOCC      Car number : 3088
# 9EA488C5320CC01B9000000000000331 : Initials : IOCC      Car number : 85123
```

Tag deserialization from a file :
```bash
# tags.txt content :
$ cat tags.txt
# Output :
# 2F3E06C007DB1E139000000000000331
# 9EA488C030426A179000000000000331
# 9EA488C5320CC01B9000000000000331
$ aei-tag-parser -f test.txt
# Output :
# 2F3E06C007DB1E139000000000000331 : Initials : QNSL      Car number : 502
# 9EA488C030426A179000000000000331 : Initials : IOCC      Car number : 3088
# 9EA488C5320CC01B9000000000000331 : Initials : IOCC      Car number : 85123
```

Tag deserialization from a UNIX pipe :
```bash
# tags.txt content :
$ cat tags.txt
# Output :
# 2F3E06C007DB1E139000000000000331
# 9EA488C030426A179000000000000331
# 9EA488C5320CC01B9000000000000331
$ cat tags.txt | aei-tag-parser
# Output :
# 2F3E06C007DB1E139000000000000331 : Initials : QNSL      Car number : 502
# 9EA488C030426A179000000000000331 : Initials : IOCC      Car number : 3088
# 9EA488C5320CC01B9000000000000331 : Initials : IOCC      Car number : 85123
```

## Librairie

This project can also be used as an external library. Documentation is available here : [https://docs.rs/aei_tag_parser/1.0.0/aei_tag_parser/index.html](https://docs.rs/aei_tag_parser/1.0.0/aei_tag_parser/index.html)

### Usage

```rust
let tag_str : String = String::from("9EA488C030426A179000000000000331");
let tag : AEITagData = AEITagData::new(&tag_str);

println!("Tag {} content is : \r\n\tInitials: {}\r\n\tCar number: {}", &tag_str, tag.equipment_initial(), tag.car_number());
```

# Install

To install the CLI util, you must have Rust installed (cf [RustUp](https://rustup.rs/)).

Then you can simply do : 
```bash 
$ cargo install aei_tag_parser
```