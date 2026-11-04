//! External-agent ingest: foreign sessions → site [`RunRecord`]s.
//!
//! Slice 1: Cursor cloud / Project workers (`cursor.cloud`). Other vendors
//! land as file-watch adapters later — same [`SiteRunsStore`], different parsers.

pub mod cursor;

pub use cursor::{CursorAgentStub, CursorEvent, ingest_cursor_agents, ingest_cursor_json};
