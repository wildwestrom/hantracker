use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use constcat::concat;
use db::Db;
use relm4::adw::prelude::*;
use relm4::{adw, gtk, prelude::*};
use tracing::{error, info};
use tracing::warn;

mod db;
mod input;
mod result;
mod testing;
mod window;

const QUALIFIER: &str = "xyz";
const ORGANIZATION: &str = "westrom";
const APPLICATION: &str = "hantracker";
const APP_ID: &str = concat!(QUALIFIER, ".", ORGANIZATION, ".", APPLICATION);

fn load_css() -> Result<()> {
	let css_path = "ui/main.css";
	if !std::path::Path::new(css_path).exists() {
		return Err(anyhow!("CSS file not found at {}", css_path));
	}

	let css_provider = gtk::CssProvider::new();
	css_provider.load_from_path(css_path);

	let display = gtk::gdk::Display::default()
		.with_context(|| "Failed to get default display. Is a display server running?")?;

	gtk::style_context_add_provider_for_display(
		&display,
		&css_provider,
		gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
	);

	Ok(())
}

fn main() -> anyhow::Result<()> {
	let rt = relm4::tokio::runtime::Builder::new_current_thread()
		.enable_time()
		.build()?;

	let subscriber = tracing_subscriber::FmtSubscriber::builder()
		.with_max_level(if cfg!(debug_assertions) {
			tracing::Level::TRACE
		} else {
			tracing::Level::INFO
		})
		.without_time()
		.finish();

	tracing::subscriber::set_global_default(subscriber)?;

	let project_dirs = directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
		.ok_or_else(|| anyhow!("Failed to find project directory"))?;
	let data_dir = project_dirs.data_dir();
	let data_dir = Arc::new(data_dir.to_owned());

	let path_from_project_dir = db_path_from_data_dir(&data_dir)?;

	let db_path = match std::env::var("DATABASE_URL") {
		Ok(path) => path,
		Err(err) => match err {
			std::env::VarError::NotPresent => path_from_project_dir,
			std::env::VarError::NotUnicode(_os_string) => {
				warn!(
					"Database path is not a unicode string!: {}",
					_os_string.to_string_lossy()
				);
				path_from_project_dir
			}
		},
	};

	let db_path = db_path
		.trim_start_matches("sqlite://")
		.trim_end_matches("?mode=rwc") // This looks so dirty to me, but I don't know a better way
		.to_string();

	// open a new database
	let db = rt.block_on(async { Db::new(&db_path).await })?;
	info!("Connected to database at {:?}", db_path);

	let adw_app = adw::Application::builder().application_id(APP_ID).build();
	adw_app.connect_startup(|_| {
		if let Err(e) = load_css() {
			error!("Failed to load CSS: {}", e);
		}
	});
	let app = RelmApp::from_app(adw_app);

	app.run_async::<window::Ht>((db, data_dir));

	Ok(())
}

fn db_path_from_data_dir(data_dir: &Path) -> Result<String> {
	let mut db_path = data_dir.to_path_buf();
	if !db_path.try_exists()? {
		std::fs::create_dir_all(&db_path)?;
	}
	db_path.push("data.sqlite");
	db_path.into_os_string().into_string().map_err(|os_string| {
		anyhow!(
			"Path not a valid unicode string {}",
			os_string.to_string_lossy()
		)
	})
}
