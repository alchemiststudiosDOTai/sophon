//! SQLite persistence for CLI runs. Kept in `cli` so domain stays free of `rusqlite`.

use std::path::Path;

use chrono::Utc;
use rusqlite::{Connection, params};

use crate::domain::{SearchBatchResponse, SearchResponse, SearchResult};

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS search_runs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  query TEXT NOT NULL,
  provider TEXT NOT NULL,
  run_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS search_results (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  run_id INTEGER NOT NULL,
  result_type TEXT NOT NULL,
  title TEXT NOT NULL,
  url TEXT NOT NULL,
  snippet TEXT,
  source TEXT,
  published_at TEXT,
  thumbnail_url TEXT,
  duration TEXT,
  display_url TEXT,
  FOREIGN KEY (run_id) REFERENCES search_runs(id)
);

CREATE TABLE IF NOT EXISTS scraped_sites (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  run_id INTEGER NOT NULL,
  seed_url TEXT NOT NULL,
  duration_ms INTEGER,
  page_limit INTEGER,
  error TEXT,
  FOREIGN KEY (run_id) REFERENCES search_runs(id)
);

CREATE TABLE IF NOT EXISTS scraped_pages (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  site_id INTEGER NOT NULL,
  url TEXT NOT NULL,
  status_code INTEGER,
  content TEXT,
  FOREIGN KEY (site_id) REFERENCES scraped_sites(id)
);
"#;

/// Append-only writer for search (and optional scrape) tables.
pub struct SearchDbWriter {
    conn: Connection,
}

impl SearchDbWriter {
    pub fn open(path: impl AsRef<Path>) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self { conn })
    }

    pub fn insert_search_run(&self, query: &str, provider: &str) -> rusqlite::Result<i64> {
        let run_at = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO search_runs (query, provider, run_at) VALUES (?1, ?2, ?3)",
            params![query, provider, run_at],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn insert_results(&self, run_id: i64, response: &SearchResponse) -> rusqlite::Result<()> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO search_results (
                run_id, result_type, title, url, snippet, source,
                published_at, thumbnail_url, duration, display_url
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        )?;

        for result in &response.results {
            let (
                result_type,
                title,
                url,
                snippet,
                source,
                published_at,
                thumbnail_url,
                duration,
                display_url,
            ) = match result {
                SearchResult::Web(r) => (
                    "web",
                    r.title.as_str(),
                    r.url.as_str(),
                    r.snippet.as_deref(),
                    None,
                    None,
                    None,
                    None,
                    r.display_url.as_deref(),
                ),
                SearchResult::News(r) => (
                    "news",
                    r.title.as_str(),
                    r.url.as_str(),
                    r.snippet.as_deref(),
                    r.source.as_deref(),
                    r.published_at.as_deref(),
                    None,
                    None,
                    None,
                ),
                SearchResult::Image(r) => (
                    "image",
                    r.title.as_str(),
                    r.url.as_str(),
                    None,
                    r.source.as_deref(),
                    None,
                    r.thumbnail_url.as_deref(),
                    None,
                    None,
                ),
                SearchResult::Video(r) => (
                    "video",
                    r.title.as_str(),
                    r.url.as_str(),
                    None,
                    None,
                    r.published_at.as_deref(),
                    r.thumbnail_url.as_deref(),
                    r.duration.as_deref(),
                    None,
                ),
            };

            stmt.execute(params![
                run_id,
                result_type,
                title,
                url,
                snippet,
                source,
                published_at,
                thumbnail_url,
                duration,
                display_url,
            ])?;
        }
        Ok(())
    }

    /// One `scraped_sites` row plus pages; `run_id` ties telemetry to a search run.
    pub fn insert_scrape(
        &self,
        run_id: i64,
        seed_url: &str,
        duration_ms: u64,
        page_limit: usize,
        scrape_error: Option<&str>,
        pages: &[(String, Option<u16>, String)],
    ) -> rusqlite::Result<()> {
        let tx = self.conn.unchecked_transaction()?;
        tx.execute(
            "INSERT INTO scraped_sites (run_id, seed_url, duration_ms, page_limit, error)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                run_id,
                seed_url,
                duration_ms as i64,
                page_limit as i64,
                scrape_error,
            ],
        )?;
        let site_id = tx.last_insert_rowid();
        for (url, status, content) in pages {
            tx.execute(
                "INSERT INTO scraped_pages (site_id, url, status_code, content) VALUES (?1, ?2, ?3, ?4)",
                params![
                    site_id,
                    url,
                    status.map(|s| s as i64),
                    content.as_str(),
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn persist_response(&self, response: &SearchResponse) -> rusqlite::Result<i64> {
        let run_id = self.insert_search_run(&response.query, &response.provider)?;
        self.insert_results(run_id, response)?;
        Ok(run_id)
    }

    pub fn persist_batch_responses(
        &self,
        batch: &SearchBatchResponse,
    ) -> rusqlite::Result<Vec<i64>> {
        let mut ids = Vec::with_capacity(batch.responses.len());
        for response in &batch.responses {
            ids.push(self.persist_response(response)?);
        }
        Ok(ids)
    }

    #[cfg(test)]
    pub(crate) fn result_count_for_run(&self, run_id: i64) -> rusqlite::Result<i64> {
        self.conn.query_row(
            "SELECT COUNT(*) FROM search_results WHERE run_id = ?1",
            [run_id],
            |r| r.get(0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{SearchResult, WebResult};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn persist_response_creates_run_and_results() {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("sophon_db_test_{nanos}.db"));
        let _ = std::fs::remove_file(&path);

        let db = SearchDbWriter::open(path.as_path()).unwrap();
        let response = SearchResponse {
            query: "q".to_string(),
            provider: "test".to_string(),
            results: vec![SearchResult::Web(WebResult {
                title: "t".to_string(),
                url: "https://example.com".to_string(),
                snippet: Some("s".to_string()),
                display_url: None,
            })],
            total_estimated: None,
            next_page: None,
        };
        let run_id = db.persist_response(&response).unwrap();
        assert!(run_id > 0);

        let count = db.result_count_for_run(run_id).unwrap();
        assert_eq!(count, 1);

        let _ = std::fs::remove_file(path);
    }
}
