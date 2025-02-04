use anyhow::anyhow;
use anyhow::Result;
use constcat::concat;
use db::Db;
use relm4::adw::prelude::*;
use relm4::{adw, gtk, prelude::*};
use tracing::info;
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

fn load_css() {
	let css_provider = gtk::CssProvider::new();
	css_provider.load_from_path("ui/main.css");

	gtk::style_context_add_provider_for_display(
		&gtk::gdk::Display::default().expect("Could not connect to display"),
		&css_provider,
		gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
	);
}

fn main() -> anyhow::Result<()> {
	// why current thread? idk, I think we don't need multi threading so maybe smaller binary size?
	// TODO: perf test
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

	let path_from_project_dir = db_path_from_project_dir()?;

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
	info!("Created new db at {:?}", db_path);

	let adw_app = adw::Application::builder().application_id(APP_ID).build();
	adw_app.connect_startup(|_| load_css());
	let app = RelmApp::from_app(adw_app);

	app.run_async::<window::Ht>(db);

	Ok(())
}

fn db_path_from_project_dir() -> Result<String> {
	let project_dirs = directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
		.ok_or(anyhow!("Failed to find project directory"))?;
	let mut db_path = project_dirs.data_dir().to_path_buf();
	if !db_path.try_exists()? {
		std::fs::create_dir(&db_path)?;
	}
	db_path.push("data.sqlite");
	Ok(db_path
		.into_os_string()
		.into_string()
		.or_else(|os_string| {
			Err(anyhow!(
				"Path not a valid unicode string {}",
				os_string.to_string_lossy()
			))
		})?)
}
