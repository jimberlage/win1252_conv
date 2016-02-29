pub mod win1252_conv {
    // Taken from http://unicode.org/Public/MAPPINGS/VENDORS/MICSFT/WINDOWS/CP1252.TXT
    const WIN1252_CHARS: [Option<char>; 128] = [
        Some('€'),
        None,
        Some('‚'),
        Some('ƒ'),
        Some('„'),
        Some('…'),
        Some('†'),
        Some('‡'),
        Some('ˆ'),
        Some('‰'),
        Some('Š'),
        Some('‹'),
        Some('Œ'),
        None,
        Some('Ž'),
        None,
        None,
        Some('‘'),
        Some('’'),
        Some('“'),
        Some('”'),
        Some('•'),
        Some('–'),
        Some('—'),
        Some('˜'),
        Some('™'),
        Some('š'),
        Some('›'),
        Some('œ'),
        None,
        Some('ž'),
        Some('Ÿ'),
        Some(' '),
        Some('¡'),
        Some('¢'),
        Some('£'),
        Some('¤'),
        Some('¥'),
        Some('¦'),
        Some('§'),
        Some('¨'),
        Some('©'),
        Some('ª'),
        Some('«'),
        Some('¬'),
        Some('­'),
        Some('®'),
        Some('¯'),
        Some('°'),
        Some('±'),
        Some('²'),
        Some('³'),
        Some('´'),
        Some('µ'),
        Some('¶'),
        Some('·'),
        Some('¸'),
        Some('¹'),
        Some('º'),
        Some('»'),
        Some('¼'),
        Some('½'),
        Some('¾'),
        Some('¿'),
        Some('À'),
        Some('Á'),
        Some('Â'),
        Some('Ã'),
        Some('Ä'),
        Some('Å'),
        Some('Æ'),
        Some('Ç'),
        Some('È'),
        Some('É'),
        Some('Ê'),
        Some('Ë'),
        Some('Ì'),
        Some('Í'),
        Some('Î'),
        Some('Ï'),
        Some('Ð'),
        Some('Ñ'),
        Some('Ò'),
        Some('Ó'),
        Some('Ô'),
        Some('Õ'),
        Some('Ö'),
        Some('×'),
        Some('Ø'),
        Some('Ù'),
        Some('Ú'),
        Some('Û'),
        Some('Ü'),
        Some('Ý'),
        Some('Þ'),
        Some('ß'),
        Some('à'),
        Some('á'),
        Some('â'),
        Some('ã'),
        Some('ä'),
        Some('å'),
        Some('æ'),
        Some('ç'),
        Some('è'),
        Some('é'),
        Some('ê'),
        Some('ë'),
        Some('ì'),
        Some('í'),
        Some('î'),
        Some('ï'),
        Some('ð'),
        Some('ñ'),
        Some('ò'),
        Some('ó'),
        Some('ô'),
        Some('õ'),
        Some('ö'),
        Some('÷'),
        Some('ø'),
        Some('ù'),
        Some('ú'),
        Some('û'),
        Some('ü'),
        Some('ý'),
        Some('þ'),
        Some('ÿ'),
    ];

    fn convert_win1252_byte(byte: u8) -> char {
        if byte >= 0x80 {
            let i = (byte - 0x80) as usize;

            match WIN1252_CHARS[i] {
                Some(ch) => ch,
                None     => panic!("Encountered an invalid WIN-1252 byte that was not a part of a UTF-8 character."),
            }
        } else {
            byte as char
        }
    }

    fn num_utf8_chars(c: u8) -> Option<u8> {
        if (c >> 5) == 6 {         // c = 110xxxxx
            Some(2)
        } else if (c >> 4) == 14 { // c = 1110xxxx
            Some(3)
        } else if (c >> 3) == 30 { // c = 11110xxx
            Some(4)
        } else {
            None
        }
    }

    /* Converts a terrible hybrid of WIN-1252 and UTF-8 to compliant UTF-8. Preserves valid UTF-8
     * characters, but converts various out-of-spec WIN-1252 bytes to their UTF-8 equivalent. */
    pub fn convert(bytes: Vec<u8>) -> String {
        let mut result = String::new();
        let mut i = 0;
        let len = bytes.len();

        while i < len {
            let c = bytes[i];
            match num_utf8_chars(c) {
                Some(n) => {
                    let j = if i + (n as usize) > len { len } else { i + (n as usize) };
                    let mut byte_slice = vec![];
                    for &b in bytes[i..j].iter() {
                        byte_slice.push(b);
                    }

                    match String::from_utf8(byte_slice) {
                        Ok(string) => {
                            result.push_str(&string);
                            i = j;
                        },
                        Err(_)     => {
                            result.push(convert_win1252_byte(c));
                            i = i + 1;
                        },
                    };
                },
                None    => {
                    result.push(convert_win1252_byte(c));
                    i = i + 1;
                },
            };
        }

        result
    }
}

#[cfg(test)]
mod test {
    use win1252_conv::convert;

    #[test]
    fn convert_test() {
        // It handles ASCII correctly
        assert!(convert(vec![0x61, 0x62, 0x63, 0x64]) == "abcd");

        // It converts WIN-1252 to UTF-8
        assert!(convert(vec![0x61, 0x86, 0x63, 0x64]) == "a†cd");

        // It preserves UTF-8 values
        assert!(convert(vec![0x61, 0xE2, 0x80, 0xA0, 0x63, 0x64]) == "a†cd");

        // It handles both WIN-1252 and UTF-8 values in the same string
        assert!(convert(vec![0x99, 0xE2, 0x84, 0xA2, 0x99]) == "™™™");
    }
}
