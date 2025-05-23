//! Cleanup queries for servers altered by client behavior.
use super::{super::Server, Guard};

/// Queries used to clean up server connections after
/// client modifications.
#[derive(Default)]
#[allow(dead_code)]
pub struct Cleanup {
    queries: Vec<&'static str>,
    reset: bool,
    dirty: bool,
    deallocate: bool,
}

impl std::fmt::Display for Cleanup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.queries.join(","))
    }
}

impl Cleanup {
    /// New cleanup operation.
    pub fn new(guard: &Guard, server: &Server) -> Self {
        if guard.reset {
            Self::all()
        } else if server.dirty() {
            Self::parameters()
        } else if server.schema_changed() {
            Self::prepared_statements()
        } else {
            Self::none()
        }
    }

    /// Cleanup prepared statements.
    pub fn prepared_statements() -> Self {
        Self {
            queries: vec!["DEALLOCATE ALL"],
            deallocate: true,
            ..Default::default()
        }
    }

    /// Cleanup parameters.
    pub fn parameters() -> Self {
        Self {
            queries: vec!["RESET ALL", "DISCARD ALL"],
            dirty: true,
            ..Default::default()
        }
    }

    /// Cleanup everything.
    pub fn all() -> Self {
        Self {
            reset: true,
            dirty: true,
            deallocate: true,
            queries: vec!["RESET ALL", "DISCARD ALL", "DEALLOCATE ALL"],
        }
    }

    /// Nothing to clean up.
    pub fn none() -> Self {
        Self {
            queries: vec![],
            ..Default::default()
        }
    }

    /// Cleanup needed?
    pub fn needed(&self) -> bool {
        !self.queries.is_empty()
    }

    /// Get queries to execute on the server to perform cleanup.
    pub fn queries(&self) -> &[&str] {
        &self.queries
    }

    pub fn is_reset_params(&self) -> bool {
        self.dirty
    }
}
