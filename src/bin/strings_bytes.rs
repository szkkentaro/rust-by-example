use std::str;

fn main() {
    // This is not actually a &str
    let bytestring: &[u8; 20] = b"this is a bytestring";

    println!("A bytestring: {:?}", bytestring);

    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...but no unicode escapes
    // let escaped = b"\u{211D} is not allowd";

    println!("Some escaped bytes: {:?}", escaped);

    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("{:?}", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                     like with normal raw strings"#;

    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82"; // ようこそ in SHIFT-JIS

    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    }
}
