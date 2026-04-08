use anyhow::{Context, Result, bail};
use regex::Regex;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match command {
        "generate-registry" => generate_registry()?,
        "help" | "--help" | "-h" => {
            eprintln!("Usage: xtask <command>");
            eprintln!();
            eprintln!("Commands:");
            eprintln!("  generate-registry  Parse C headers and generate registry source files");
        }
        other => bail!("unknown command: {other}"),
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct FuncEntry {
    define_name: String, // e.g. "XC_LDA_X"
    id: u16,
    family: &'static str, // "Family::Lda" etc.
    kind: &'static str,   // "Kind::Exchange" etc.
}

fn parse_family_and_kind(name: &str) -> (&'static str, &'static str) {
    // Strip "XC_" prefix
    let rest = name.strip_prefix("XC_").unwrap_or(name);

    // Determine family (and whether it's a hybrid prefix)
    let after_family = if rest.starts_with("HYB_MGGA_") {
        &rest[9..] // "HYB_MGGA_".len() == 9
    } else if rest.starts_with("HYB_GGA_") {
        &rest[8..] // "HYB_GGA_".len() == 8
    } else if rest.starts_with("HYB_LDA_") {
        &rest[8..] // "HYB_LDA_".len() == 8
    } else if rest.starts_with("MGGA_") {
        &rest[5..] // "MGGA_".len() == 5
    } else if rest.starts_with("GGA_") {
        &rest[4..] // "GGA_".len() == 4
    } else if rest.starts_with("LDA_") {
        &rest[4..] // "LDA_".len() == 4
    } else {
        rest
    };

    let family = if rest.starts_with("HYB_MGGA_") || rest.starts_with("MGGA_") {
        "Family::Mgga"
    } else if rest.starts_with("HYB_GGA_") || rest.starts_with("GGA_") {
        "Family::Gga"
    } else {
        "Family::Lda"
    };

    // Determine kind from what follows the family prefix
    let kind = if after_family.starts_with("XC_") || after_family == "XC" {
        "Kind::ExchangeCorrelation"
    } else if after_family.starts_with("X_") || after_family == "X" {
        "Kind::Exchange"
    } else if after_family.starts_with("C_") || after_family == "C" {
        "Kind::Correlation"
    } else if after_family.starts_with("K_") || after_family == "K" {
        "Kind::Kinetic"
    } else {
        // Fallback: Exchange
        "Kind::Exchange"
    };

    (family, kind)
}

fn generate_registry() -> Result<()> {
    let source_root = find_source_root()?;
    let output_root = find_output_root()?;
    let funcs_path = source_root.join("libxc-master/src/xc_funcs.h");
    let removed_path = source_root.join("libxc-master/src/xc_funcs_removed.h");

    let funcs_content = fs::read_to_string(&funcs_path)
        .with_context(|| format!("failed to read {}", funcs_path.display()))?;
    let removed_content = fs::read_to_string(&removed_path)
        .with_context(|| format!("failed to read {}", removed_path.display()))?;

    // Parse xc_funcs.h
    let re = Regex::new(r#"#define\s+XC_(\w+)\s+(\d+)"#)?;
    let mut entries: BTreeMap<u16, FuncEntry> = BTreeMap::new();

    for cap in re.captures_iter(&funcs_content) {
        let suffix = &cap[1];
        let id: u16 = cap[2].parse()?;
        let define_name = format!("XC_{suffix}");
        let (family, kind) = parse_family_and_kind(&define_name);

        entries.insert(
            id,
            FuncEntry {
                define_name,
                id,
                family,
                kind,
            },
        );
    }

    eprintln!("Parsed {} functionals from xc_funcs.h", entries.len());

    // Parse xc_funcs_removed.h
    let mut removed_ids: Vec<(u16, u16)> = Vec::new();
    let mut name_aliases: Vec<(String, u16)> = Vec::new();

    // Categorize lines
    #[derive(PartialEq)]
    enum Category {
        OldNames,
        AllCaps,
        Removed,
    }
    let mut current_cat = Category::OldNames;

    for line in removed_content.lines() {
        if line.contains("old names kept for compatibility") {
            current_cat = Category::OldNames;
            continue;
        }
        if line.contains("converted to all caps") {
            current_cat = Category::AllCaps;
            continue;
        }
        if line.contains("functionals that were removed") {
            current_cat = Category::Removed;
            continue;
        }

        if let Some(cap) = re.captures(line) {
            let suffix = &cap[1];
            let id: u16 = cap[2].parse()?;
            let define_name = format!("XC_{suffix}");

            match current_cat {
                Category::OldNames | Category::AllCaps => {
                    // These are name aliases mapping to an active ID
                    name_aliases.push((define_name, id));
                }
                Category::Removed => {
                    if entries.contains_key(&id) {
                        // The numeric ID was reassigned to a different functional
                        // in xc_funcs.h. The old name is dead but the ID is alive.
                        // Don't add to REMOVED_IDS (that would break ID lookup for
                        // the new functional). Instead track the removed name so
                        // name-based lookups return a proper error.
                        // We store (removed_name, old_id) -- will be mapped to
                        // replacement by find_hyb_replacement later.
                    } else {
                        // ID is truly gone from active registry
                        removed_ids.push((id, 0));
                    }
                }
            }
        }
    }

    // For removed IDs, we need to check: if the removed name's ID also appears
    // in xc_funcs.h (was reassigned), the replacement is the new functional at that ID.
    // But actually looking at the data:
    // - XC_GGA_X_HERMAN 104 -> ID 104 is XC_GGA_C_WI0 in xc_funcs.h? Let's check.
    // - XC_GGA_XC_B97 167 -> ID 167 is not in xc_funcs.h, but XC_HYB_GGA_XC_B97 is ID 407.
    // Actually, we need to look at this differently. For the "removed" category:
    // The comment says these were GGA variants that became HYB_ variants.
    // Let's find the HYB_ version by name matching.
    // Re-examine: the removed IDs are:
    //   104 (GGA_X_HERMAN) - no replacement
    //   167 (GGA_XC_B97) -> HYB_GGA_XC_B97 exists?
    // etc. We need to search for a HYB_ version of each removed name in the active entries.

    // Rebuild removed_ids with proper replacement logic
    let mut corrected_removed: Vec<(u16, u16)> = Vec::new();
    for &(removed_id, _) in &removed_ids {
        // Find the removed name from the regex captures
        let removed_name = removed_content
            .lines()
            .find_map(|line| {
                re.captures(line).and_then(|cap| {
                    let id: u16 = cap[2].parse().ok()?;
                    if id == removed_id {
                        Some(format!("XC_{}", &cap[1]))
                    } else {
                        None
                    }
                })
            })
            .unwrap_or_default();

        // Try to find a HYB_ version of this functional
        let replacement = find_hyb_replacement(&removed_name, &entries);
        corrected_removed.push((removed_id, replacement));
    }

    eprintln!(
        "Parsed {} removed IDs and {} name aliases from xc_funcs_removed.h",
        corrected_removed.len(),
        name_aliases.len()
    );

    // Generate output files
    generate_meta_file(&output_root, &entries)?;
    generate_by_id_file(&output_root, &entries)?;
    generate_by_name_file(&output_root, &entries)?;
    generate_removed_file(&output_root, &corrected_removed, &name_aliases)?;

    eprintln!("Registry generation complete.");
    Ok(())
}

/// Find a HYB_ replacement for a removed functional.
/// E.g., XC_GGA_XC_B97 (167) -> look for XC_HYB_GGA_XC_B97 in active entries.
fn find_hyb_replacement(removed_name: &str, entries: &BTreeMap<u16, FuncEntry>) -> u16 {
    let rest = removed_name.strip_prefix("XC_").unwrap_or(removed_name);

    // Try inserting "HYB_" before the family prefix
    let hyb_name = if rest.starts_with("MGGA_") {
        format!("XC_HYB_MGGA_{}", &rest[5..])
    } else if rest.starts_with("GGA_") {
        format!("XC_HYB_GGA_{}", &rest[4..])
    } else if rest.starts_with("LDA_") {
        format!("XC_HYB_LDA_{}", &rest[4..])
    } else {
        return 0;
    };

    // Search entries for the hybrid version (case-insensitive to handle
    // e.g. XC_GGA_XC_SB98_1a -> XC_HYB_GGA_XC_SB98_1A)
    let hyb_upper = hyb_name.to_ascii_uppercase();
    for entry in entries.values() {
        if entry.define_name.to_ascii_uppercase() == hyb_upper {
            return entry.id;
        }
    }

    0
}

/// Find the output root: the directory containing Cargo.toml (workspace root / worktree).
fn find_output_root() -> Result<std::path::PathBuf> {
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join("Cargo.toml").exists() {
            return Ok(dir);
        }
        if !dir.pop() {
            bail!("could not find output root (directory containing Cargo.toml)");
        }
    }
}

/// Find the source root: the directory containing libxc-master/ with the C headers.
/// This may differ from the output root in git worktree setups.
fn find_source_root() -> Result<std::path::PathBuf> {
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join("libxc-master").exists() {
            return Ok(dir);
        }
        if !dir.pop() {
            bail!("could not find source root (directory containing libxc-master/)");
        }
    }
}

fn generate_meta_file(root: &Path, entries: &BTreeMap<u16, FuncEntry>) -> Result<()> {
    let path = root.join("src/meta/generated.rs");
    let mut out = String::with_capacity(entries.len() * 300);

    out.push_str("//! Auto-generated by xtask generate-registry. DO NOT EDIT.\n");
    out.push_str("#![allow(non_upper_case_globals)]\n\n");
    out.push_str("use crate::model::{DerivativeOrder, Family, FunctionalFlags, FunctionalId, Kind};\n");
    out.push_str("use crate::meta::FunctionalMeta;\n\n");

    for entry in entries.values() {
        out.push_str(&format!(
            "pub(crate) const {name}: FunctionalMeta = FunctionalMeta {{\n\
             \x20   id: FunctionalId({id}),\n\
             \x20   name: \"{name}\",\n\
             \x20   kind: {kind},\n\
             \x20   family: {family},\n\
             \x20   flags: FunctionalFlags::empty(),\n\
             \x20   references: &[],\n\
             \x20   ext_params: &[],\n\
             \x20   default_density_threshold: 1e-15,\n\
             \x20   auxiliaries: &[],\n\
             \x20   hybrid_terms: &[],\n\
             \x20   nlc_params: None,\n\
             \x20   max_order: DerivativeOrder::Exc,\n\
             }};\n\n",
            name = entry.define_name,
            id = entry.id,
            kind = entry.kind,
            family = entry.family,
        ));
    }

    fs::write(&path, &out).with_context(|| format!("failed to write {}", path.display()))?;
    eprintln!("Generated {} entries in {}", entries.len(), path.display());
    Ok(())
}

fn generate_by_id_file(root: &Path, entries: &BTreeMap<u16, FuncEntry>) -> Result<()> {
    let path = root.join("src/registry/by_id.rs");
    let mut out = String::with_capacity(entries.len() * 60 + 500);

    out.push_str("//! Auto-generated by xtask generate-registry. DO NOT EDIT.\n\n");
    out.push_str("use crate::meta::FunctionalMeta;\n");
    out.push_str("use crate::meta::generated;\n\n");
    out.push_str("pub(crate) static REGISTRY_BY_ID: [Option<&'static FunctionalMeta>; 1024] = {\n");
    out.push_str("    let mut table: [Option<&'static FunctionalMeta>; 1024] = [None; 1024];\n");

    for entry in entries.values() {
        out.push_str(&format!(
            "    table[{id}] = Some(&generated::{name});\n",
            id = entry.id,
            name = entry.define_name,
        ));
    }

    out.push_str("    table\n");
    out.push_str("};\n");

    fs::write(&path, &out).with_context(|| format!("failed to write {}", path.display()))?;
    eprintln!("Generated by_id table in {}", path.display());
    Ok(())
}

fn generate_by_name_file(root: &Path, entries: &BTreeMap<u16, FuncEntry>) -> Result<()> {
    let path = root.join("src/registry/by_name.rs");
    let mut out = String::with_capacity(entries.len() * 40 + 200);

    // Sort by name
    let mut sorted: Vec<(&str, u16)> = entries
        .values()
        .map(|e| (e.define_name.as_str(), e.id))
        .collect();
    sorted.sort_by_key(|&(name, _)| name);

    out.push_str("//! Auto-generated by xtask generate-registry. DO NOT EDIT.\n\n");
    out.push_str(&format!(
        "pub(crate) static REGISTRY_BY_NAME: [(&str, u16); {}] = [\n",
        sorted.len()
    ));

    for (name, id) in &sorted {
        out.push_str(&format!("    (\"{name}\", {id}),\n"));
    }

    out.push_str("];\n");

    fs::write(&path, &out).with_context(|| format!("failed to write {}", path.display()))?;
    eprintln!("Generated by_name table in {}", path.display());
    Ok(())
}

fn generate_removed_file(
    root: &Path,
    removed: &[(u16, u16)],
    aliases: &[(String, u16)],
) -> Result<()> {
    let path = root.join("src/registry/removed.rs");
    let mut out = String::with_capacity(1024);

    out.push_str("//! Auto-generated by xtask generate-registry. DO NOT EDIT.\n\n");

    // Removed IDs
    out.push_str("/// Removed functional IDs that should produce RemovedFunctionalId error.\n");
    out.push_str("/// (removed_id, replacement_id) -- replacement_id = 0 means no replacement.\n");
    out.push_str(&format!(
        "pub(crate) static REMOVED_IDS: &[(u16, u16)] = &[\n"
    ));

    for &(removed_id, replacement_id) in removed {
        out.push_str(&format!("    ({removed_id}, {replacement_id}),\n"));
    }

    out.push_str("];\n\n");

    // Name aliases
    out.push_str("/// Name aliases (old name -> active ID) for backward compatibility.\n");
    out.push_str(&format!(
        "pub(crate) static NAME_ALIASES: &[(&str, u16)] = &[\n"
    ));

    for (name, id) in aliases {
        out.push_str(&format!("    (\"{name}\", {id}),\n"));
    }

    out.push_str("];\n");

    fs::write(&path, &out).with_context(|| format!("failed to write {}", path.display()))?;
    eprintln!("Generated removed table in {}", path.display());
    Ok(())
}
