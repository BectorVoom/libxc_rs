//! Errno enrichment for "removed functional ID" errors (06-02b-T1).
//!
//! `LibxcRsError::RemovedFunctionalId { removed_id, replacement_id, replacement_name }`
//! is raised by `src/registry/mod.rs::lookup_by_id` for the handful of libxc IDs
//! that were removed in 7.0.0. The compat layer surfaces it via the int errno
//! mechanism; these helpers extract / format the replacement payload.
//!
//! NOTE: `src/registry/removed.rs` (the `REMOVED_IDS` table) is a *private* module,
//! so this helper reaches the data through the public `registry::lookup_by_id`
//! error path rather than importing the table directly.

use crate::error::LibxcRsError;

/// Returns `Some((replacement_id, replacement_name))` if `id` is a removed functional ID,
/// `None` if `id` is a live functional (or simply unknown).
pub fn replacement_for(id: u16) -> Option<(u16, &'static str)> {
    match crate::registry::lookup_by_id(id) {
        Err(LibxcRsError::RemovedFunctionalId {
            replacement_id,
            replacement_name,
            ..
        }) => Some((replacement_id, replacement_name)),
        _ => None,
    }
}

/// Format a `RemovedFunctionalId` error's payload for the thread-local errno message.
/// Returns `None` for any other error variant.
pub fn format_removed_message(e: &LibxcRsError) -> Option<String> {
    match e {
        LibxcRsError::RemovedFunctionalId {
            removed_id,
            replacement_id,
            replacement_name,
        } => Some(format!(
            "ID {removed_id} removed; use {replacement_id} ({replacement_name}) instead"
        )),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replacement_for_live_functional_is_none() {
        // lda_x (id 1) is a live functional — not removed.
        assert_eq!(replacement_for(1), None);
    }

    #[test]
    fn format_removed_message_ignores_other_errors() {
        assert_eq!(
            format_removed_message(&LibxcRsError::UninitializedHandle),
            None
        );
        let msg = format_removed_message(&LibxcRsError::RemovedFunctionalId {
            removed_id: 999,
            replacement_id: 1,
            replacement_name: "lda_x",
        });
        assert_eq!(msg.as_deref(), Some("ID 999 removed; use 1 (lda_x) instead"));
    }
}
