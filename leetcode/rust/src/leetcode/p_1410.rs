// 1410. HTML Entity Parser
// ------------------------

use std::collections::HashMap;

impl Solution {
    pub fn entity_parser(text: String) -> String {
        let mut parsed: Vec<u8> = vec![];
        let mut buffer: Vec<u8> = vec![];
        let mut translations: HashMap<String, u8> = HashMap::with_capacity(6);
        translations.insert("&quot;".to_owned(), b'"');
        translations.insert("&apos;".to_owned(), b'\'');
        translations.insert("&amp;".to_owned(), b'&');
        translations.insert("&gt;".to_owned(), b'>');
        translations.insert("&lt;".to_owned(), b'<');
        translations.insert("&frasl;".to_owned(), b'/');

        let mut bytes = text.as_bytes().iter();

        while let Some(c) = bytes.next() {
            if *c == b'&' {
                if buffer.len() > 0 {
                    parsed.extend_from_slice(&buffer);
                }
                buffer.clear();
                buffer.push(*c);
            } else {
                if buffer.len() > 0 {
                    buffer.push(*c);
                    let key = String::from_utf8(buffer.clone()).unwrap();
                    if translations.contains_key(&key) {
                        let append = translations.get(&key).unwrap();
                        parsed.push(*append);
                        buffer.clear();
                    }
                } else {
                    parsed.push(*c);
                }
            }
        }

        if buffer.len() > 0 {
            parsed.extend_from_slice(&buffer);
        }

        String::from_utf8(parsed).unwrap()
    }
}
