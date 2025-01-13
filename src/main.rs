use anyhow::Result;

fn main() -> Result<()> {
	let dict = lib::load_kanjidic::bootstrap_dict()?;
	Ok(())
}
