use anyhow::Result;
use lib::is_chinese_character;
use sqlx::{migrate, query, sqlite::SqliteConnectOptions, QueryBuilder, SqlitePool};
use tracing::debug;

use rand::seq::SliceRandom;

use crate::testing::Test;

// const SCHEMA_SQL: &str = include_str!("schema.sql");
// const SCHEMA_VERSION: i32 = 1;

// This is the default profile id to use until we support having multiple profiles.
const DEFAULT_PROFILE_ID: u8 = 0;

#[derive(Clone, Debug)]
pub struct Db {
	pool: SqlitePool,
}

impl Db {
	pub async fn new(url: &str) -> Result<Self> {
		debug!("attempt connection with database at {url}");

		// Set connection options BEFORE creating the pool
		let options = SqliteConnectOptions::new()
			.create_if_missing(true)
			.foreign_keys(true)
			.journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
			.filename(url);

		debug!("Attempting to open database file at: {:?}", url);

		// Use the options when creating the pool
		let pool = SqlitePool::connect_with(options).await?;

		// Add this section to run migrations
		migrate!().run(&pool).await?;

		debug!("Current directory: {:?}", std::env::current_dir()?);

		if cfg!(debug_assertions) {
			// Check what tables exist
			let mut conn = pool.acquire().await?;
			let tables = query!("SELECT name FROM sqlite_master WHERE type='table'")
				.fetch_all(&mut *conn)
				.await?;

			debug!("Existing tables: {:?}", tables);

			if !tables
				.into_iter()
				.any(|t| t.name.expect("table doesn't have a name") == "user_profile")
			{
				return Err(anyhow::anyhow!("user_profile table does not exist - did migrations run on the correct database file?"));
			}
		}

		Ok(Self { pool })
	}

	pub async fn test_exists(&self) -> Result<bool> {
		let mut conn = self.pool.acquire().await?;
		let exists = query!(
			"SELECT test_exists FROM user_profile WHERE id = ? LIMIT 1",
			DEFAULT_PROFILE_ID,
		)
		.fetch_one(&mut *conn)
		.await?
		.test_exists;
		Ok(exists)
	}

	pub async fn get_text(&self) -> Result<String> {
		let mut conn = self.pool.acquire().await?;
		let text = query!(
			"SELECT text FROM user_profile_raw_test_text WHERE profile = ? LIMIT 1",
			DEFAULT_PROFILE_ID
		)
		.fetch_one(&mut *conn)
		.await?
		.text;
		Ok(text)
	}

	pub async fn set_text(&self, s: &str) -> Result<()> {
		let mut conn = self.pool.acquire().await?;
		query!(
			"UPDATE user_profile_raw_test_text SET text = ?1 WHERE profile = ?2",
			s,
			DEFAULT_PROFILE_ID,
		)
		.execute(&mut *conn)
		.await?;
		Ok(())
	}

	pub async fn create_test_from_raw_text(&self) -> Result<()> {
		let mut conn = self.pool.acquire().await?;

		let mut chars = self.get_all_chinese_characters().await?;

		chars.dedup(); // I'm not sure if this actually makes a difference, but I don't wanna see this bug.

		let mut rng = rand::rng();
		chars.shuffle(&mut rng);

		query!(
			"UPDATE user_profile SET test_exists = true WHERE id = ?",
			DEFAULT_PROFILE_ID
		)
		.execute(&mut *conn)
		.await?;

		query!(
			"DELETE FROM user_profile_characters WHERE profile = ?",
			DEFAULT_PROFILE_ID
		)
		.execute(&mut *conn)
		.await?;

		let mut query_builder: QueryBuilder<sqlx::Sqlite> =
			QueryBuilder::new("INSERT INTO user_profile_characters (profile, char, known)");
		query_builder.push_values(chars, |mut row, c| {
			row.push_bind(DEFAULT_PROFILE_ID)
				.push_bind(u32::from(c))
				.push_bind(false);
		});
		query_builder.build().execute(&mut *conn).await?;

		Ok(())
	}

	pub async fn get_all_chinese_characters(&self) -> Result<Vec<char>> {
		let text = self.get_text().await?;
		debug!("All the text in the database {text}");
		Ok(text.chars().filter(is_chinese_character).collect())
	}

	pub async fn chinese_character_exists(&self) -> Result<bool> {
		let chars = self.get_all_chinese_characters().await?;
		debug!("{}", chars.len());
		Ok(!chars.is_empty())
	}

	pub async fn get_previous_test(&self) -> Result<Option<Vec<Test>>> {
		if self.test_exists().await? {
			let chars = self.get_default_profile_chars().await?;
			Ok(Some(chars))
		} else {
			Ok(None)
		}
	}

	pub async fn get_default_profile_chars(&self) -> Result<Vec<Test>> {
		let mut conn = self.pool.acquire().await?;
		let records = query!(
			"SELECT char, known FROM user_profile_characters WHERE profile = ?1",
			DEFAULT_PROFILE_ID,
		)
		.fetch_all(&mut *conn)
		.await?;
		let chars = records
			.iter()
			.map(|record| Test {
				#[allow(
					clippy::cast_possible_truncation,
					clippy::as_conversions,
					clippy::cast_sign_loss,
					reason = "I expect every value to be a valid UTF-8 character"
				)]
				char: char::from_u32(record.char as u32).expect("should work"),
				#[allow(clippy::match_bool, reason = "This is fine")]
				recalled: match record.known {
					true => crate::testing::Recalled::Known,
					false => crate::testing::Recalled::Unknown,
				},
			})
			.collect();
		Ok(chars)
	}

	pub async fn get_test_progress(&self) -> Result<i64> {
		let mut conn = self.pool.acquire().await?;
		let record = query!(
			"SELECT test_progress FROM user_profile WHERE id = ?1 LIMIT 1",
			DEFAULT_PROFILE_ID,
		)
		.fetch_one(&mut *conn)
		.await?;
		Ok(record.test_progress)
	}

	pub async fn set_test_progress(&self, progress: usize) -> Result<()> {
		let mut conn = self.pool.acquire().await?;
		let progress = i64::try_from(progress)?;
		// I don't know why the value "doesn't live long enough"
		// when I just inline it into the query macro... whatever.
		query!(
			"UPDATE user_profile SET test_progress = ? WHERE id = ?",
			progress,
			DEFAULT_PROFILE_ID
		)
		.execute(&mut *conn)
		.await?;
		Ok(())
	}

	pub async fn reset(&self) -> Result<()> {
		let mut conn = self.pool.acquire().await?;
		query!(
			"UPDATE user_profile SET test_exists = true WHERE id = ?",
			DEFAULT_PROFILE_ID
		)
		.execute(&mut *conn)
		.await?;

		query!(
			"DELETE FROM user_profile_characters WHERE profile = ?",
			DEFAULT_PROFILE_ID
		)
		.execute(&mut *conn)
		.await?;

		Ok(())
	}
}
