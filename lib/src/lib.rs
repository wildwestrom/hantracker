pub mod load_kanjidic;

pub fn is_chinese_character(c: &char) -> bool {
	let c = *c as u32;
	c >= 0x3400 && c <= 0x4DBF || // CJK Unified Ideographs Extension A
	c >= 0x4E00 && c <= 0x9FFF || // CJK Unified Ideographs
    c >= 0x20000 && c <= 0x2A6D || // CJK Unified Ideographs Extension B
    c >= 0x2A700 && c <= 0x2B73 || // CJK Unified Ideographs Extension C
    c >= 0x2B740 && c <= 0x2B81 || // CJK Unified Ideographs Extension D
    c >= 0x2B820 && c <= 0x2CEA || // CJK Unified Ideographs Extension E
    c >= 0x2CEB0 && c <= 0x2EBE || // CJK Unified Ideographs Extension F
    c >= 0x2EBF0 && c <= 0x2EE5 || // CJK Unified Ideographs Extension I
    c >= 0x2F800 && c <= 0x2FA1 || // CJK Compatibility Ideographs Supplement
    c >= 0x30000 && c <= 0x3134 || // CJK Unified Ideographs Extension G
    c >= 0x31350 && c <= 0x323A // CJK Unified Ideographs Extension H
}

pub fn sort_kanji(chars: &mut Vec<char>) {
	chars.sort();
}

pub fn vec_string_to_vec_char(one_char_strings: Vec<String>) -> Vec<char> {
	one_char_strings
		.into_iter()
		.map(|s| {
			let should_be_one_char = s.chars().collect::<Vec<_>>();
			assert!(should_be_one_char.len() == 1);
			unsafe { *should_be_one_char.get_unchecked(0) }
		})
		.collect()
}
