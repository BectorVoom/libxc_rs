mod by_id;
mod by_name;
mod removed;

use crate::error::LibxcRsError;
use crate::meta::FunctionalMeta;
use crate::model::FunctionalId;

/// Look up functional metadata by raw integer ID.
/// Returns the static metadata for valid IDs, or an error for unknown/removed IDs.
/// O(1) via sparse array. Per D-01.
pub fn lookup_by_id(id: u16) -> Result<&'static FunctionalMeta, LibxcRsError> {
    // Check removed IDs first per D-03
    if let Some(&(_, replacement_id)) = removed::REMOVED_IDS.iter().find(|&&(r, _)| r == id) {
        let replacement_name = if replacement_id > 0 {
            by_id::REGISTRY_BY_ID
                .get(replacement_id as usize)
                .and_then(|opt| opt.as_ref())
                .map(|m| m.name)
                .unwrap_or("unknown")
        } else {
            "none"
        };
        return Err(LibxcRsError::RemovedFunctionalId {
            removed_id: id,
            replacement_id,
            replacement_name,
        });
    }

    by_id::REGISTRY_BY_ID
        .get(id as usize)
        .and_then(|opt| *opt)
        .ok_or(LibxcRsError::UnknownFunctionalId(id))
}

/// Look up a functional ID by name (case-insensitive).
/// Returns the FunctionalId for valid names, or error for unknown names.
/// O(log n) via binary search. Per D-02.
pub fn lookup_by_name(name: &str) -> Result<FunctionalId, LibxcRsError> {
    let upper = name.to_ascii_uppercase();

    // Check name aliases first
    if let Some(&(_, id)) = removed::NAME_ALIASES
        .iter()
        .find(|&&(n, _)| n.eq_ignore_ascii_case(&upper))
    {
        return Ok(FunctionalId(id));
    }

    if let Ok(idx) = by_name::REGISTRY_BY_NAME.binary_search_by_key(&upper.as_str(), |&(n, _)| n) {
        return Ok(FunctionalId(by_name::REGISTRY_BY_NAME[idx].1));
    }

    let xc_upper = if !upper.starts_with("XC_") {
        format!("XC_{upper}")
    } else {
        upper.clone()
    };

    by_name::REGISTRY_BY_NAME
        .binary_search_by_key(&xc_upper.as_str(), |&(n, _)| n)
        .map(|idx| FunctionalId(by_name::REGISTRY_BY_NAME[idx].1))
        .map_err(|_| LibxcRsError::UnknownFunctionalName(name.to_string()))
}

/// Total number of registered functionals.
pub fn functional_count() -> usize {
    649
}

/// Maximum name length across all functionals.
pub fn max_name_length() -> usize {
    by_name::REGISTRY_BY_NAME
        .iter()
        .map(|(n, _)| n.len())
        .max()
        .unwrap_or(0)
}

/// Get all functional IDs in numeric order.
pub fn all_functional_ids() -> impl Iterator<Item = FunctionalId> {
    by_id::REGISTRY_BY_ID
        .iter()
        .filter_map(|opt| opt.as_ref())
        .map(|m| m.id)
}

/// Library version (matches libxc 7.0.0).
pub fn version() -> (u32, u32, u32) {
    (7, 0, 0)
}

/// Library version string.
pub fn version_string() -> &'static str {
    "7.0.0"
}

/// Library reference string.
pub fn reference_string() -> &'static str {
    "libxc_rs: Rust reimplementation of libxc 7.0.0"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Family, Kind};

    #[test]
    fn test_lookup_lda_x() {
        let meta = lookup_by_id(1).unwrap();
        assert_eq!(meta.name, "XC_LDA_X");
        assert_eq!(meta.family, Family::Lda);
        assert_eq!(meta.kind, Kind::Exchange);
    }

    #[test]
    fn test_lookup_last_id() {
        // XC_GGA_X_Q1D is ID 734 -- the highest known ID
        let meta = lookup_by_id(734).unwrap();
        assert_eq!(meta.name, "XC_GGA_X_Q1D");
    }

    #[test]
    fn test_lookup_unknown_id() {
        let err = lookup_by_id(999).unwrap_err();
        match err {
            LibxcRsError::UnknownFunctionalId(id) => assert_eq!(id, 999),
            other => panic!("expected UnknownFunctionalId, got {other:?}"),
        }
    }

    #[test]
    fn test_lookup_by_name() {
        let id = lookup_by_name("XC_LDA_X").unwrap();
        assert_eq!(id, FunctionalId(1));
    }

    #[test]
    fn test_lookup_by_name_case_insensitive() {
        let id = lookup_by_name("xc_lda_x").unwrap();
        assert_eq!(id, FunctionalId(1));
    }

    #[test]
    fn test_lookup_unknown_name() {
        let err = lookup_by_name("bogus").unwrap_err();
        match err {
            LibxcRsError::UnknownFunctionalName(name) => assert_eq!(name, "bogus"),
            other => panic!("expected UnknownFunctionalName, got {other:?}"),
        }
    }

    #[test]
    fn test_removed_id_104() {
        let err = lookup_by_id(104).unwrap_err();
        match err {
            LibxcRsError::RemovedFunctionalId {
                removed_id,
                replacement_id,
                ..
            } => {
                assert_eq!(removed_id, 104);
                assert_eq!(replacement_id, 0);
            }
            other => panic!("expected RemovedFunctionalId, got {other:?}"),
        }
    }

    #[test]
    fn test_functional_count() {
        assert_eq!(functional_count(), 649);
    }

    #[test]
    fn test_all_ids_count() {
        assert_eq!(all_functional_ids().count(), 649);
    }

    #[test]
    fn test_registry_completeness() {
        for id in all_functional_ids() {
            let meta = lookup_by_id(id.raw()).unwrap();
            assert!(!meta.name.is_empty(), "functional {} has empty name", id.raw());
            // family and kind are always set by the generator
        }
    }

    #[test]
    fn test_version() {
        assert_eq!(version(), (7, 0, 0));
    }

    #[test]
    fn test_name_alias_resolves() {
        let id = lookup_by_name("XC_GGA_X_BGCP").unwrap();
        assert_eq!(id, FunctionalId(38));
    }
}
