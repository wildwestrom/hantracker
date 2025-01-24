pub mod derive_str;
pub mod load_kanjidic;

#[must_use]
pub const fn is_chinese_character(c: &char) -> bool {
	#[allow(clippy::as_conversions, reason = "I doubt this will fail")]
	let c = *c as u32;
	c >= 0x3400 && c <= 0x4DBF || // CJK Unified Ideographs Extension A
	c >= 0x4E00 && c <= 0x9FFF || // CJK Unified Ideographs
    c >= 0x20000 && c <= 0x2A6DF || // CJK Unified Ideographs Extension B
    c >= 0x2A700 && c <= 0x2B73F || // CJK Unified Ideographs Extension C
    c >= 0x2B740 && c <= 0x2B81F || // CJK Unified Ideographs Extension D
    c >= 0x2B820 && c <= 0x2CEAF || // CJK Unified Ideographs Extension E
    c >= 0x2CEB0 && c <= 0x2EBEF || // CJK Unified Ideographs Extension F
    c >= 0x2EBF0 && c <= 0x2EE5F || // CJK Unified Ideographs Extension I
    c >= 0x2F800 && c <= 0x2FA1F || // CJK Compatibility Ideographs Supplement
    c >= 0x30000 && c <= 0x3134F || // CJK Unified Ideographs Extension G
    c >= 0x31350 && c <= 0x323AF // CJK Unified Ideographs Extension H
}

pub fn sort_kanji(chars: &mut [char]) {
	chars.sort_unstable();
}

/// # Panics
///
/// TODO: Somehow make the type system assert that these strings are in fact one character strings.
#[must_use]
pub fn vec_string_to_vec_char(one_char_strings: Vec<String>) -> Vec<char> {
	one_char_strings
		.into_iter()
		.map(|s| {
			let should_be_one_char = s.chars().collect::<Vec<_>>();
			assert!(
				should_be_one_char.len() == 1,
				"The caller is responsible for making sure they only pass 1 character strings"
			);
			*should_be_one_char.first().unwrap()
		})
		.collect()
}
