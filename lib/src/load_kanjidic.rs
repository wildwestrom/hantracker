#![allow(clippy::needless_late_init, reason = "`XmlRead` macro error")]
#![allow(unused, reason = "This is temporary")]

use std::{
	fs::{self, File, OpenOptions},
	io::{self, BufReader, Read, Write},
};

const FORCE_INVALIDATE_CACHE: bool = false;

use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};

const KANJI_XML_PATH: &str = "resources/kanjidic2.xml.gz";

use hard_xml::XmlRead;
use thiserror::Error;
use tracing::debug;

use crate::{sort_kanji, vec_string_to_vec_char};

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "kanjidic2")]
pub struct Dict2 {
	#[xml(child = "header")]
	header: Dict2Header,
	#[xml(child = "character")]
	character: Vec<Character>,
}

impl Dict2 {
	#[must_use]
	pub fn get_all_jlpt(&self) -> Vec<char> {
		let mut chars: Vec<String> = self
			.character
			.iter()
			.filter(|c| c.misc.jlpt.is_some())
			.map(|c| c.literal.clone())
			.collect();
		chars.sort();
		vec_string_to_vec_char(chars)
	}

	#[must_use]
	pub fn get_all_joyo(&self) -> Vec<char> {
		let mut chars: Vec<String> = self
			.character
			.iter()
			.filter(|c| c.misc.grade.is_some_and(|g| matches!(g, 1..6 | 8)))
			.map(|c| c.literal.clone())
			.collect();
		chars.sort();
		vec_string_to_vec_char(chars)
	}

	#[must_use]
	pub fn get_all_kyoiku(&self) -> Vec<char> {
		let mut chars: Vec<String> = self
			.character
			.iter()
			.filter(|c| c.misc.grade.is_some_and(|g| matches!(g, 1..6)))
			.map(|c| c.literal.clone())
			.collect();
		chars.sort();
		vec_string_to_vec_char(chars)
	}
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "header")]
struct Dict2Header {
	#[xml(flatten_text = "file_version")]
	file_version: u64,
	#[xml(flatten_text = "database_version")]
	database_version: String,
	#[xml(flatten_text = "date_of_creation")]
	date_of_creation: String,
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "character")]
struct Character {
	#[xml(flatten_text = "literal")]
	literal: String,
	#[xml(child = "codepoint")]
	codepoint: Codepoint,
	#[xml(child = "radical")]
	radical: Radical,
	#[xml(child = "misc")]
	misc: Misc,
	#[xml(child = "dic_number")]
	dic_number: Option<DicNumber>,
	#[xml(child = "query_code")]
	query_code: QueryCode,
	#[xml(child = "reading_meaning")]
	reading_meaning: Option<ReadingMeaning>,
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "codepoint")]
struct Codepoint {
	#[xml(child = "cp_value")]
	cp_value: Vec<CpValue>,
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "cp_value")]
struct CpValue {
	#[xml(attr = "cp_type")]
	cp_type: String,
	#[xml(text)]
	value: String,
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "radical")]
struct Radical {
	#[xml(child = "rad_value")]
	rad_value: Vec<RadValue>,
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "rad_value")]
struct RadValue {
	#[xml(attr = "rad_type")]
	rad_type: String,
	#[xml(text)]
	rad_value: u64,
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "misc")]
struct Misc {
	#[xml(flatten_text = "grade")]
	grade: Option<u64>,
	#[xml(flatten_text = "stroke_count")]
	stroke_count: Vec<u64>,
	#[xml(child = "variant")]
	variant: Option<MiscVariant>,
	#[xml(flatten_text = "freq")]
	freq: Option<u64>,
	#[xml(flatten_text = "jlpt")]
	jlpt: Option<u64>,
	#[xml(flatten_text = "rad_name")]
	rad_name: Option<String>,
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "variant")]
struct MiscVariant {
	#[xml(attr = "var_type")]
	var_type: String,
	#[xml(text)]
	var_value: String,
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "dic_number")]
struct DicNumber {
	#[xml(child = "dic_ref")]
	dic_ref: Vec<DicRef>,
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "dic_ref")]
struct DicRef {
	#[xml(attr = "dr_type")]
	dr_type: String,
	#[xml(text)]
	dr_value: String,
	#[xml(attr = "m_vol")]
	m_vol: Option<u64>,
	#[xml(attr = "m_page")]
	m_page: Option<u64>,
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "query_code")]
struct QueryCode {
	#[xml(child = "q_code")]
	q_code: Vec<QCode>,
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "q_code")]
struct QCode {
	#[xml(attr = "qc_type")]
	qc_type: String,
	#[xml(text)]
	qc_value: String,
	#[xml(attr = "skip_misclass")]
	skip_misclass: Option<String>,
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "reading_meaning")]
struct ReadingMeaning {
	#[xml(child = "rmgroup")]
	rmgroup: Vec<RMGroup>,
	#[xml(flatten_text = "nanori")]
	nanori: Vec<String>,
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "rmgroup")]
struct RMGroup {
	#[xml(child = "reading")]
	reading: Vec<Reading>,
	#[xml(child = "meaning")]
	meaning: Vec<Meaning>,
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "meaning")]
struct Meaning {
	#[xml(attr = "m_lang")]
	m_lang: Option<String>,
	#[xml(text)]
	meaning: String,
}

#[derive(XmlRead, Debug)]
#[xml(strict(unknown_attribute, unknown_element))]
#[xml(tag = "reading")]
struct Reading {
	#[xml(attr = "r_type")]
	r_type: String,
	#[xml(text)]
	r_value: String,
}

#[derive(Debug, Error, Default)]
pub enum Error {
	#[error("Io operation `{operation}` failed")]
	Io {
		source: io::Error,
		operation: String,
	},
	#[error("Failed to handle xml")]
	Xml(#[from] hard_xml::XmlError),
	#[error("Failed to calculate time")]
	Time(#[from] std::time::SystemTimeError),
	#[error("Failed to calculate time")]
	Network(#[from] reqwest::Error),

	#[error("unknown error")]
	#[default]
	Unknown,
}

impl Error {
	fn from_io(op: &str) -> (impl FnOnce(io::Error) -> Self + use<'_>) {
		|e: io::Error| Self::Io {
			source: e,
			operation: op.to_owned(),
		}
	}
}

#[allow(clippy::missing_errors_doc, reason = "Don't care for now")]
pub fn bootstrap_dict() -> Result<Dict2, Error> {
	load_kanji_xml_data()?;
	let file = File::open(KANJI_XML_PATH).map_err(Error::from_io("open"))?;
	let mut decoded = GzDecoder::new(file);

	let string = {
		let mut s = String::new();
		decoded
			.read_to_string(&mut s)
			.map_err(Error::from_io("read"))?;
		s
	};

	let dicc = Dict2::from_str(&string)?;
	Ok(dicc)
}

fn load_kanji_xml_data() -> Result<(), Error> {
	fs::create_dir_all("resources").map_err(Error::from_io("create_dir_all"))?;
	if fs::exists(KANJI_XML_PATH).map_err(Error::from_io("check if exists"))? {
		let mut file_on_disk = OpenOptions::new()
			.write(true)
			.read(true)
			.open(KANJI_XML_PATH)
			.map_err(Error::from_io("open"))?;
		let file_is_old = {
			let modified_date = file_on_disk
				.metadata()
				.map_err(Error::from_io("read metadata"))?
				.modified()
				.map_err(Error::from_io("read modified time"))?;
			let now = std::time::SystemTime::now();
			let diff = now.duration_since(modified_date)?;
			diff >= std::time::Duration::from_secs(60 * 60 * 24 * 60 /*60 days*/)
		};
		if file_is_old || FORCE_INVALIDATE_CACHE {
			let downloaded_bytes = download_kanjidic()?;
			let downloaded_shasum = Sha256::digest(&downloaded_bytes);
			let disk_shasum = {
				let mut hasher = Sha256::new();
				let mut filereader = BufReader::new(&file_on_disk);
				io::copy(&mut filereader, &mut hasher).map_err(Error::from_io("copy"))?;
				hasher.finalize()
			};
			if downloaded_shasum != disk_shasum {
				file_on_disk
					.write_all(&downloaded_bytes)
					.map_err(Error::from_io("write if shasums aren't equal"))?;
			}
		}
	} else {
		let mut file_on_disk =
			File::create(KANJI_XML_PATH).map_err(Error::from_io("create file"))?;
		let downloaded_bytes = download_kanjidic()?;
		file_on_disk
			.write_all(&downloaded_bytes)
			.map_err(Error::from_io("write if file doesn't exist"))?;
	}
	Ok(())
}

fn download_kanjidic() -> Result<Vec<u8>, Error> {
	let url = "http://www.edrdg.org/kanjidic/kanjidic2.xml.gz";
	debug!("Requesting...");
	let mut request = reqwest::blocking::get(url)?;
	debug!("Request finished!");
	let mut buffer = Vec::<u8>::new();
	io::copy(&mut request, &mut buffer).map_err(Error::from_io("copy"))?;
	Ok(buffer)
}
