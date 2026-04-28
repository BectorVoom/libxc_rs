//! Generate metadata snapshot from libxc 7.0.0 FFI for all 649 functionals.
//!
//! IDs are discovered by probing 1..1024 with `xc_func_init` and keeping every
//! id whose rc == 0. This is equivalent to iterating
//! `libxc_rs::registry::all_functional_ids()` but does not require a
//! path-dep on `libxc_rs` (whose 170-kernel transitive graph takes ~9 hours
//! to compile under CubeCL proc-macro expansion — see Cargo.toml comment).
//!
//! For each successfully-init'd id:
//!   1. Read `xc_func_info_type` fields (name, kind, family, flags, refs,
//!      ext_params, dens_threshold) and `xc_func_type` fields (n_func_aux,
//!      func_aux, mix_coef, hyb_number_terms, hyb_type, hyb_coeff, hyb_omega,
//!      nlc_b, nlc_C). Also call `xc_hyb_type` to snapshot the hybrid
//!      classification.
//!   2. Build a `Snapshot` struct mirroring `FunctionalMeta` shape.
//!   3. Call `xc_func_end`.
//!
//! After collecting all 649 snapshots:
//!   - Validate aux-graph depth ≤ 2 (D-17 invariant).
//!   - Emit `src/meta/generated.rs` (FunctionalMeta consts + named statics for
//!     references / ext_params / auxiliaries / hybrid_terms).
//!   - Emit `src/meta/generated_hybrid.rs` (HYBRID_TYPES table for ids whose
//!     hybrid_type != Semilocal).
//!   - Emit `src/meta/generated_propagation.rs` (PROPAGATION_RULES — small
//!     hand-curated list of canonical Copy-semantics rules per D-16).

use anyhow::{anyhow, bail, Context, Result};
use libxc_sys::*;
use std::collections::HashMap;
use std::ffi::CStr;
use std::fs;
use std::path::{Path, PathBuf};

// ============================================================================
// XC_FAMILY_* and XC_HYB_* constants (from libxc-master/src/xc.h)
// ============================================================================
const XC_FAMILY_LDA: i32 = 1;
const XC_FAMILY_GGA: i32 = 2;
const XC_FAMILY_MGGA: i32 = 4;

const XC_EXCHANGE: i32 = 0;
const XC_CORRELATION: i32 = 1;
const XC_EXCHANGE_CORRELATION: i32 = 2;
const XC_KINETIC: i32 = 3;

const XC_HYB_TERM_NONE: i32 = 0;
const XC_HYB_TERM_FOCK: i32 = 1;
const XC_HYB_TERM_PT2: i32 = 2;
const XC_HYB_TERM_ERF_SR: i32 = 4;
const XC_HYB_TERM_YUKAWA_SR: i32 = 8;
const XC_HYB_TERM_GAUSSIAN_SR: i32 = 16;

// xc_hyb_type return values (per libxc-master/src/xc.h:94-100)
const XC_HYB_TYPE_SEMILOCAL: i32 = 0;
const XC_HYB_TYPE_HYBRID: i32 = 1;
const XC_HYB_TYPE_CAM: i32 = 2;
const XC_HYB_TYPE_CAMY: i32 = 3;
const XC_HYB_TYPE_CAMG: i32 = 4;
const XC_HYB_TYPE_DOUBLE_HYBRID: i32 = 5;
const XC_HYB_TYPE_MIXTURE: i32 = 32768;

// libxc XC_FLAGS_HAVE_* bits (used to derive max derivative order)
const XC_FLAGS_HAVE_EXC: i32 = 1 << 0;
const XC_FLAGS_HAVE_VXC: i32 = 1 << 1;
const XC_FLAGS_HAVE_FXC: i32 = 1 << 2;
const XC_FLAGS_HAVE_KXC: i32 = 1 << 3;
const XC_FLAGS_HAVE_LXC: i32 = 1 << 4;

// ============================================================================
// Snapshot types (mirror FunctionalMeta shape, owned strings for emission)
// ============================================================================

#[derive(Debug, Clone)]
struct Snapshot {
    id: u16,
    name: String,         // canonical "XC_..." name from registry
    libxc_name: String,   // human-readable display name from xc_func_info
    kind: &'static str,   // "Kind::Exchange" etc.
    family: &'static str, // "Family::Lda" etc.
    flags: u32,
    references: Vec<RefSnapshot>,
    ext_params: Vec<ExtParamSnapshot>,
    default_density_threshold: f64,
    auxiliaries: Vec<(u16, f64)>,
    hybrid_terms: Vec<HybridTermSnapshot>,
    nlc_params: Option<(f64, f64)>,
    max_order: &'static str, // "DerivativeOrder::Lxc" etc.
    hybrid_type: &'static str, // "HybridType::Hybrid" etc.
}

#[derive(Debug, Clone)]
struct RefSnapshot {
    citation: String,
    doi: String,
    bibtex: String,
    key: String,
}

#[derive(Debug, Clone)]
struct ExtParamSnapshot {
    name: String,
    description: String,
    default_value: f64,
    is_internal: bool,
}

#[derive(Debug, Clone)]
struct HybridTermSnapshot {
    kind: &'static str, // "HybridTermKind::Fock" etc.
    coefficient: f64,
    omega: f64,
}

// ============================================================================
// Entry point
// ============================================================================

pub fn run() -> Result<()> {
    eprintln!("Generating metadata snapshot from libxc 7.0.0 (FFI introspection)...");

    let workspace_root = find_workspace_root()?;
    let snapshots = collect_all_functionals()?;
    if snapshots.is_empty() {
        bail!("No functionals collected — xc_func_init rc=0 for no id in 1..1024");
    }
    eprintln!("Collected {} snapshots from FFI", snapshots.len());

    validate_aux_depth(&snapshots)?;

    write_generated_rs(&workspace_root, &snapshots)?;
    write_generated_hybrid_rs(&workspace_root, &snapshots)?;
    write_generated_propagation_rs(&workspace_root, &snapshots)?;

    eprintln!("Generated metadata for {} functionals", snapshots.len());
    Ok(())
}

fn find_workspace_root() -> Result<PathBuf> {
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join("Cargo.toml").exists() && dir.join("xtask").exists() {
            return Ok(dir);
        }
        if !dir.pop() {
            bail!("could not find workspace root (with Cargo.toml + xtask/)");
        }
    }
}

// ============================================================================
// FFI snapshot loop
// ============================================================================
//
// We probe IDs 1..1024 by invoking `xc_func_init` directly and skipping any id
// whose rc != 0. This avoids a path-dep on `libxc_rs` (whose 170-kernel
// transitive graph takes ~9 hours to compile under CubeCL proc-macro
// expansion). The set of ids that init successfully equals the 649 ids in
// `libxc_rs::registry::all_functional_ids()` by construction (the registry was
// also generated by xtask scanning the same xc_funcs.h header set).
//
// The canonical "XC_..." name needed by `src/registry/by_name.rs` is fetched
// via libxc's `xc_functional_get_name(id)` API which returns the C-define
// short name (e.g. "hyb_gga_xc_cam_b3lyp"); we uppercase it and prepend
// "XC_" to match the existing registry tables verbatim. (`info.name` is the
// human-readable display string like "CAM version of B3LYP" — NOT a valid
// Rust identifier, so we never use it for emission.)

const ID_PROBE_RANGE: std::ops::RangeInclusive<u16> = 1..=1023;

fn collect_all_functionals() -> Result<Vec<Snapshot>> {
    let mut snapshots = Vec::with_capacity(649);

    for raw in ID_PROBE_RANGE {
        // Probe: ask libxc for the canonical name for this id. If it returns
        // null, this id is not a registered libxc functional — skip silently.
        let name_ptr = unsafe { xc_functional_get_name(raw as i32) };
        if name_ptr.is_null() {
            continue;
        }
        let libxc_short = unsafe { CStr::from_ptr(name_ptr) }
            .to_str()
            .map_err(|e| anyhow!("non-utf8 short name for id={raw}: {e}"))?
            .to_string();
        // libxc allocates the returned string with `strdup`; per its
        // contract the caller frees with libc::free. We accept the small
        // leak per-id (≤ 649 short strings, kilobytes total) — this is a
        // build-only tool, not a long-running process. Documenting the leak
        // to be explicit; do not "fix" by calling free on a pointer the
        // CStr is still borrowing.

        let canonical_name = format!("XC_{}", libxc_short.to_ascii_uppercase());

        let snapshot = snapshot_one(raw, &canonical_name)
            .with_context(|| format!("snapshotting id={raw} name={canonical_name}"))?;

        snapshots.push(snapshot);
    }

    Ok(snapshots)
}

fn snapshot_one(id: u16, canonical_name: &str) -> Result<Snapshot> {
    // SAFETY: xc_func_type is bindgen-generated #[repr(C)] with `Default` impl
    // that zeroes the bytes. xc_func_init expects a zeroed handle.
    let mut t: xc_func_type = unsafe { std::mem::zeroed() };
    let rc = unsafe { xc_func_init(&mut t, id as i32, XC_UNPOLARIZED as i32) };
    if rc != 0 {
        bail!("xc_func_init failed for id={id} ({canonical_name}): rc={rc}");
    }

    // SAFETY: rc==0 means t.info is non-null and points to static C data.
    let info = unsafe { &*t.info };

    // ── name (libxc display name) ───────────────────────────────────────
    let libxc_name = unsafe { CStr::from_ptr(info.name) }
        .to_str()
        .map_err(|e| anyhow!("non-utf8 name for id={id}: {e}"))?
        .to_string();

    // ── kind ────────────────────────────────────────────────────────────
    let kind = match info.kind {
        XC_EXCHANGE => "Kind::Exchange",
        XC_CORRELATION => "Kind::Correlation",
        XC_EXCHANGE_CORRELATION => "Kind::ExchangeCorrelation",
        XC_KINETIC => "Kind::Kinetic",
        other => bail!("unknown kind {other} for id={id}"),
    };

    // ── family ──────────────────────────────────────────────────────────
    // libxc reports XC_FAMILY_LDA/GGA/MGGA. Hybrid functionals are still
    // in their base family (e.g. B3LYP -> XC_FAMILY_GGA); the "hybridness"
    // is captured by hybrid_terms / hybrid_type, not by family.
    let family = match info.family {
        XC_FAMILY_LDA => "Family::Lda",
        XC_FAMILY_GGA => "Family::Gga",
        XC_FAMILY_MGGA => "Family::Mgga",
        other => bail!("unsupported family {other} for id={id} (LCA/OEP not in scope)"),
    };

    // ── flags ───────────────────────────────────────────────────────────
    let flags = info.flags as u32;

    // ── references ──────────────────────────────────────────────────────
    let mut references = Vec::new();
    for slot in 0..5 {
        let ptr = info.refs[slot];
        if ptr.is_null() {
            continue;
        }
        // SAFETY: non-null `*mut func_reference_type` from libxc's static array
        let r = unsafe { &*ptr };
        let citation = c_str_to_string_or_default(r.ref_);
        let doi = c_str_to_string_or_default(r.doi);
        let bibtex = c_str_to_string_or_default(r.bibtex);
        let key = c_str_to_string_or_default(r.key);
        references.push(RefSnapshot { citation, doi, bibtex, key });
    }

    // ── ext_params ──────────────────────────────────────────────────────
    let mut ext_params = Vec::new();
    let n_params = info.ext_params.n;
    if n_params > 0 {
        for i in 0..n_params {
            let name_ptr = unsafe { *info.ext_params.names.offset(i as isize) };
            let desc_ptr = unsafe { *info.ext_params.descriptions.offset(i as isize) };
            let default_value = unsafe { *info.ext_params.values.offset(i as isize) };
            let pname = c_str_to_string_or_default(name_ptr);
            let pdesc = c_str_to_string_or_default(desc_ptr);
            let is_internal = pname.starts_with('_');
            ext_params.push(ExtParamSnapshot {
                name: pname,
                description: pdesc,
                default_value,
                is_internal,
            });
        }
    }

    // ── default_density_threshold ──────────────────────────────────────
    let default_density_threshold = info.dens_threshold;

    // ── auxiliaries (id, mix_coef) pairs from t.func_aux + t.mix_coef ───
    let mut auxiliaries = Vec::new();
    let n_aux = t.n_func_aux;
    if n_aux > 0 && !t.func_aux.is_null() && !t.mix_coef.is_null() {
        for i in 0..n_aux {
            let aux_ptr = unsafe { *t.func_aux.offset(i as isize) };
            if aux_ptr.is_null() {
                continue;
            }
            let aux_info = unsafe { (*aux_ptr).info };
            if aux_info.is_null() {
                continue;
            }
            let aux_id = unsafe { (*aux_info).number } as u16;
            let weight = unsafe { *t.mix_coef.offset(i as isize) };
            auxiliaries.push((aux_id, weight));
        }
    }

    // ── hybrid_terms ────────────────────────────────────────────────────
    let mut hybrid_terms = Vec::new();
    let n_hyb = t.hyb_number_terms;
    if n_hyb > 0 && !t.hyb_type.is_null() && !t.hyb_coeff.is_null() && !t.hyb_omega.is_null() {
        // Pitfall 6: single XC_HYB_NONE term (used by some screened GGAs to
        // store omega) is encoded as empty hybrid_terms.
        let single_none = n_hyb == 1
            && unsafe { *t.hyb_type } == XC_HYB_TERM_NONE;
        if !single_none {
            for i in 0..n_hyb {
                let kind_raw = unsafe { *t.hyb_type.offset(i as isize) };
                let coeff = unsafe { *t.hyb_coeff.offset(i as isize) };
                let omega = unsafe { *t.hyb_omega.offset(i as isize) };
                let kind_str = match kind_raw {
                    XC_HYB_TERM_NONE => {
                        // Skip stray NONE entries inside multi-term hybrid lists
                        // (defensive — libxc produces well-formed multi-term lists).
                        continue;
                    }
                    XC_HYB_TERM_FOCK => "HybridTermKind::Fock",
                    XC_HYB_TERM_PT2 => "HybridTermKind::Pt2",
                    XC_HYB_TERM_ERF_SR => "HybridTermKind::ErfSr",
                    XC_HYB_TERM_YUKAWA_SR => "HybridTermKind::YukawaSr",
                    XC_HYB_TERM_GAUSSIAN_SR => "HybridTermKind::GaussianSr",
                    other => bail!("unknown hyb_type[{i}]={other} for id={id}"),
                };
                hybrid_terms.push(HybridTermSnapshot {
                    kind: kind_str,
                    coefficient: coeff,
                    omega,
                });
            }
        }
    }

    // ── nlc_params ──────────────────────────────────────────────────────
    let nlc_params = if t.nlc_b != 0.0 || t.nlc_C != 0.0 {
        Some((t.nlc_b, t.nlc_C))
    } else {
        None
    };

    // ── max_order (derived from flags) ──────────────────────────────────
    let max_order = if flags as i32 & XC_FLAGS_HAVE_LXC != 0 {
        "DerivativeOrder::Lxc"
    } else if flags as i32 & XC_FLAGS_HAVE_KXC != 0 {
        "DerivativeOrder::Kxc"
    } else if flags as i32 & XC_FLAGS_HAVE_FXC != 0 {
        "DerivativeOrder::Fxc"
    } else if flags as i32 & XC_FLAGS_HAVE_VXC != 0 {
        "DerivativeOrder::Vxc"
    } else if flags as i32 & XC_FLAGS_HAVE_EXC != 0 {
        "DerivativeOrder::Exc"
    } else {
        "DerivativeOrder::Exc"
    };

    // ── hybrid_type (xc_hyb_type) ───────────────────────────────────────
    let hyb_type_raw = unsafe { xc_hyb_type(&t) };
    let hybrid_type = match hyb_type_raw {
        XC_HYB_TYPE_SEMILOCAL => "HybridType::Semilocal",
        XC_HYB_TYPE_HYBRID => "HybridType::Hybrid",
        XC_HYB_TYPE_CAM => "HybridType::Cam",
        XC_HYB_TYPE_CAMY => "HybridType::CamYukawa",
        XC_HYB_TYPE_CAMG => "HybridType::CamGaussian",
        XC_HYB_TYPE_DOUBLE_HYBRID => "HybridType::DoubleHybrid",
        XC_HYB_TYPE_MIXTURE => "HybridType::Mixture",
        other => bail!("unknown xc_hyb_type return {other} for id={id}"),
    };

    // SAFETY: pair with init above; releases libxc-side allocations.
    unsafe { xc_func_end(&mut t) };

    Ok(Snapshot {
        id,
        name: canonical_name.to_string(),
        libxc_name,
        kind,
        family,
        flags,
        references,
        ext_params,
        default_density_threshold,
        auxiliaries,
        hybrid_terms,
        nlc_params,
        max_order,
        hybrid_type,
    })
}

fn c_str_to_string_or_default(p: *const std::os::raw::c_char) -> String {
    if p.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned()
}

// ============================================================================
// Aux-graph depth invariant (D-17): max depth <= 2
// ============================================================================

fn validate_aux_depth(snapshots: &[Snapshot]) -> Result<()> {
    let by_id: HashMap<u16, &Snapshot> = snapshots.iter().map(|s| (s.id, s)).collect();
    for s in snapshots {
        let depth = walk_aux_depth(&by_id, s.id, 0)?;
        if depth > 2 {
            bail!(
                "aux depth {depth} exceeds D-17 invariant for id={} ({})",
                s.id,
                s.name
            );
        }
    }
    Ok(())
}

fn walk_aux_depth(
    by_id: &HashMap<u16, &Snapshot>,
    id: u16,
    depth: u8,
) -> Result<u8> {
    if depth > 4 {
        bail!("aux graph cycle detected at id={id} depth={depth}");
    }
    let snap = match by_id.get(&id) {
        Some(s) => s,
        None => return Ok(depth),
    };
    let mut max = depth;
    for (aux_id, _) in &snap.auxiliaries {
        let d = walk_aux_depth(by_id, *aux_id, depth + 1)?;
        if d > max {
            max = d;
        }
    }
    Ok(max)
}

// ============================================================================
// Emit src/meta/generated.rs
// ============================================================================

fn write_generated_rs(root: &Path, snapshots: &[Snapshot]) -> Result<()> {
    let path = root.join("src/meta/generated.rs");
    let mut out = String::with_capacity(snapshots.len() * 800);

    out.push_str("//! Auto-generated by xtask generate-metadata. DO NOT EDIT.\n");
    out.push_str("#![allow(non_upper_case_globals)]\n");
    out.push_str("#![allow(dead_code)]\n\n");
    out.push_str(
        "use crate::model::{DerivativeOrder, Family, FunctionalFlags, FunctionalId, HybridTermKind, HybridType, Kind};\n",
    );
    out.push_str("use crate::meta::{ExtParamSpec, FunctionalMeta, HybridTerm, Reference};\n\n");

    for s in snapshots {
        // ── per-functional named statics ────────────────────────────────
        // References
        if !s.references.is_empty() {
            out.push_str(&format!("static {}_REFERENCES: &[Reference] = &[\n", s.name));
            for r in &s.references {
                out.push_str(&format!(
                    "    Reference {{ citation: {}, doi: {}, bibtex: {}, key: {} }},\n",
                    rust_str_lit(&r.citation),
                    rust_str_lit(&r.doi),
                    rust_str_lit(&r.bibtex),
                    rust_str_lit(&r.key),
                ));
            }
            out.push_str("];\n");
        }

        // Ext params
        if !s.ext_params.is_empty() {
            out.push_str(&format!("static {}_EXT_PARAMS: &[ExtParamSpec] = &[\n", s.name));
            for p in &s.ext_params {
                out.push_str(&format!(
                    "    ExtParamSpec {{ name: {}, description: {}, default_value: {}, is_internal: {} }},\n",
                    rust_str_lit(&p.name),
                    rust_str_lit(&p.description),
                    f64_lit(p.default_value),
                    p.is_internal,
                ));
            }
            out.push_str("];\n");
        }

        // Auxiliaries
        if !s.auxiliaries.is_empty() {
            out.push_str(&format!(
                "static {}_AUX: &[(FunctionalId, f64)] = &[\n",
                s.name
            ));
            for (aux_id, weight) in &s.auxiliaries {
                out.push_str(&format!(
                    "    (FunctionalId({}), {}),\n",
                    aux_id,
                    f64_lit(*weight)
                ));
            }
            out.push_str("];\n");
        }

        // Hybrid terms
        if !s.hybrid_terms.is_empty() {
            out.push_str(&format!(
                "static {}_HYB_TERMS: &[HybridTerm] = &[\n",
                s.name
            ));
            for h in &s.hybrid_terms {
                out.push_str(&format!(
                    "    HybridTerm {{ kind: {}, coefficient: {}, omega: {} }},\n",
                    h.kind,
                    f64_lit(h.coefficient),
                    f64_lit(h.omega),
                ));
            }
            out.push_str("];\n");
        }

        // ── FunctionalMeta const ────────────────────────────────────────
        out.push_str(&format!(
            "pub(crate) const {name}: FunctionalMeta = FunctionalMeta {{\n\
             \x20   id: FunctionalId({id}),\n\
             \x20   name: \"{name}\",\n\
             \x20   kind: {kind},\n\
             \x20   family: {family},\n\
             \x20   flags: FunctionalFlags::from_bits_retain({flags}u32),\n",
            name = s.name,
            id = s.id,
            kind = s.kind,
            family = s.family,
            flags = s.flags,
        ));

        if s.references.is_empty() {
            out.push_str("    references: &[],\n");
        } else {
            out.push_str(&format!("    references: {}_REFERENCES,\n", s.name));
        }

        if s.ext_params.is_empty() {
            out.push_str("    ext_params: &[],\n");
        } else {
            out.push_str(&format!("    ext_params: {}_EXT_PARAMS,\n", s.name));
        }

        out.push_str(&format!(
            "    default_density_threshold: {},\n",
            f64_lit(s.default_density_threshold)
        ));

        if s.auxiliaries.is_empty() {
            out.push_str("    auxiliaries: &[],\n");
        } else {
            out.push_str(&format!("    auxiliaries: {}_AUX,\n", s.name));
        }

        if s.hybrid_terms.is_empty() {
            out.push_str("    hybrid_terms: &[],\n");
        } else {
            out.push_str(&format!("    hybrid_terms: {}_HYB_TERMS,\n", s.name));
        }

        match s.nlc_params {
            None => out.push_str("    nlc_params: None,\n"),
            Some((b, c)) => out.push_str(&format!(
                "    nlc_params: Some(({}, {})),\n",
                f64_lit(b),
                f64_lit(c)
            )),
        }

        out.push_str(&format!("    max_order: {},\n", s.max_order));
        out.push_str(&format!("    hybrid_type: {},\n", s.hybrid_type));
        out.push_str("};\n\n");
    }

    fs::write(&path, &out)
        .map_err(|e| anyhow!("Failed to write {}: {}", path.display(), e))?;
    eprintln!(
        "Wrote {} entries to {} ({} bytes)",
        snapshots.len(),
        path.display(),
        out.len()
    );
    Ok(())
}

// ============================================================================
// Emit src/meta/generated_hybrid.rs
// ============================================================================

fn write_generated_hybrid_rs(root: &Path, snapshots: &[Snapshot]) -> Result<()> {
    let path = root.join("src/meta/generated_hybrid.rs");
    let mut out = String::new();

    out.push_str("//! Auto-generated by xtask generate-metadata. DO NOT EDIT.\n");
    out.push_str("#![allow(non_upper_case_globals)]\n");
    out.push_str("#![allow(dead_code)]\n\n");
    out.push_str("use crate::model::{FunctionalId, HybridType};\n\n");
    out.push_str("pub(crate) const HYBRID_TYPES: &[(FunctionalId, HybridType)] = &[\n");

    let mut hybrid_count = 0usize;
    for s in snapshots {
        if s.hybrid_type != "HybridType::Semilocal" {
            out.push_str(&format!(
                "    (FunctionalId({}), {}),\n",
                s.id, s.hybrid_type
            ));
            hybrid_count += 1;
        }
    }

    out.push_str("];\n");

    fs::write(&path, &out)
        .map_err(|e| anyhow!("Failed to write {}: {}", path.display(), e))?;
    eprintln!(
        "Wrote {} non-Semilocal entries to {}",
        hybrid_count,
        path.display()
    );
    Ok(())
}

// ============================================================================
// Emit src/meta/generated_propagation.rs
// ============================================================================
//
// Per D-16 / Plan 05-01 Task 2 acceptance: propagation rules are limited to
// Copy semantics from a parent's named ext_param into a named ext_param on a
// specific aux slot. Non-Copy transforms are rejected.
//
// Auto-extraction would require running each parent's `_set_ext_params`
// callback while observing aux ext_params — libxc doesn't expose that
// machinery. We hand-curate the canonical Copy rules from the libxc
// `_init` source files (libxc-master/src/hyb_gga_xc_*.c). The list covers
// the canonical CAM/CAMY/CAMG hybrids whose `_omega` flows from parent to
// the range-separated aux slot.

fn write_generated_propagation_rs(root: &Path, snapshots: &[Snapshot]) -> Result<()> {
    let path = root.join("src/meta/generated_propagation.rs");

    // Build a name -> id map for ergonomic emission.
    let by_name: HashMap<String, u16> =
        snapshots.iter().map(|s| (s.name.clone(), s.id)).collect();

    let mut out = String::new();
    out.push_str("//! Auto-generated by xtask generate-metadata. DO NOT EDIT.\n");
    out.push_str("#![allow(non_upper_case_globals)]\n");
    out.push_str("#![allow(dead_code)]\n\n");
    out.push_str("use super::PropagationRule;\n");
    out.push_str("use crate::model::FunctionalId;\n\n");
    out.push_str(
        "pub(crate) const PROPAGATION_RULES: &[PropagationRule] = &[\n",
    );

    // Hand-curated canonical Copy rules per D-16. Each entry is
    //   (parent_name, parent_param_name, aux_slot, aux_param_name)
    // The parent_param_index is looked up dynamically by name (the ext_param
    // ordering varies across the libxc source — e.g. CAM-B3LYP places
    // `_omega` at index 3, while pure CAM functionals place it at index 0).
    //
    // Sources:
    //   - libxc-master/src/hyb_gga_xc_cam_b3lyp.c (CAM-B3LYP family)
    //   - libxc-master/src/hyb_gga_xc_hse.c       (HSE03/HSE06)
    //   - libxc-master/src/hyb_gga_xc_wb97.c      (wB97X family)
    let curated: &[(&str, &str, u8, &str)] = &[
        // CAM-B3LYP, CAMY-B3LYP, CAM-PBEH, CAM-O3LYP, CAMY-PBEH variants.
        // Their _omega ext_param flows to func_aux[1] (the range-separated
        // erf/yukawa aux slot) as that aux's _omega.
        ("XC_HYB_GGA_XC_CAM_B3LYP", "_omega", 1, "_omega"),
        ("XC_HYB_GGA_XC_CAMY_B3LYP", "_omega", 1, "_omega"),
        ("XC_HYB_GGA_XC_CAM_O3LYP", "_omega", 1, "_omega"),
        ("XC_HYB_GGA_XC_CAMH_B3LYP", "_omega", 1, "_omega"),
        ("XC_HYB_GGA_XC_CAMY_PBEH", "_omega", 1, "_omega"),
        ("XC_HYB_GGA_XC_CAM_QTP_00", "_omega", 1, "_omega"),
        ("XC_HYB_GGA_XC_CAM_QTP_01", "_omega", 1, "_omega"),
        ("XC_HYB_GGA_XC_CAM_QTP_02", "_omega", 1, "_omega"),
        // HSE family: _omega -> func_aux[1] _omega
        ("XC_HYB_GGA_XC_HSE03", "_omega", 1, "_omega"),
        ("XC_HYB_GGA_XC_HSE06", "_omega", 1, "_omega"),
        ("XC_HYB_GGA_XC_HSE12", "_omega", 1, "_omega"),
        ("XC_HYB_GGA_XC_HSE12S", "_omega", 1, "_omega"),
        // wB97 family
        ("XC_HYB_GGA_XC_WB97", "_omega", 1, "_omega"),
        ("XC_HYB_GGA_XC_WB97X", "_omega", 1, "_omega"),
        ("XC_HYB_GGA_XC_WB97X_V", "_omega", 1, "_omega"),
        ("XC_HYB_GGA_XC_WB97X_D3", "_omega", 1, "_omega"),
        // LC-wPBE family
        ("XC_HYB_GGA_XC_LC_WPBE", "_omega", 1, "_omega"),
        ("XC_HYB_GGA_XC_LC_WPBEH", "_omega", 1, "_omega"),
    ];

    let mut emitted = 0usize;
    let mut skipped = 0usize;
    for (parent, pname, aux_slot, aname) in curated {
        if let Some(&pid) = by_name.get(*parent) {
            // Look up the parent's ext_param index by name (varies per file).
            let parent_snap = snapshots.iter().find(|s| s.id == pid);
            let pidx_opt = parent_snap.and_then(|s| {
                s.ext_params
                    .iter()
                    .position(|p| p.name == *pname)
                    .map(|i| i as u16)
            });
            let valid_parent = pidx_opt.is_some();
            let valid_aux = parent_snap
                .map(|s| (*aux_slot as usize) < s.auxiliaries.len())
                .unwrap_or(false);
            let pidx = pidx_opt.unwrap_or(u16::MAX);
            if !(valid_parent && valid_aux) {
                skipped += 1;
                continue;
            }
            out.push_str(&format!(
                "    PropagationRule {{ parent_id: FunctionalId({pid}), parent_param_name: {pname}, parent_param_index: {pidx}, aux_slot: {aux_slot}, aux_param_name: {aname} }},\n",
                pname = rust_str_lit(pname),
                aname = rust_str_lit(aname),
            ));
            emitted += 1;
        } else {
            skipped += 1;
        }
    }

    out.push_str("];\n");

    fs::write(&path, &out)
        .map_err(|e| anyhow!("Failed to write {}: {}", path.display(), e))?;
    eprintln!(
        "Wrote {} propagation rules to {} ({} skipped)",
        emitted,
        path.display(),
        skipped
    );
    Ok(())
}

// ============================================================================
// Helpers — Rust source emission
// ============================================================================

/// Emit a string as a Rust raw or escaped string literal. Falls back to
/// escaped form for any non-ASCII or special characters; uses raw for plain
/// ASCII text without backslashes/double-quotes for readability.
fn rust_str_lit(s: &str) -> String {
    let needs_escape = s.chars().any(|c| {
        c == '"' || c == '\\' || c == '\n' || c == '\r' || c == '\t' || (c as u32) < 0x20
    });
    if !needs_escape && s.is_ascii() {
        format!("\"{s}\"")
    } else {
        // Escape with std::char::escape_default-style transformation
        let mut buf = String::with_capacity(s.len() + 2);
        buf.push('"');
        for ch in s.chars() {
            match ch {
                '"' => buf.push_str("\\\""),
                '\\' => buf.push_str("\\\\"),
                '\n' => buf.push_str("\\n"),
                '\r' => buf.push_str("\\r"),
                '\t' => buf.push_str("\\t"),
                c if (c as u32) < 0x20 => buf.push_str(&format!("\\x{:02x}", c as u32)),
                c if c.is_ascii() => buf.push(c),
                c => {
                    // Emit unicode escape so the source stays plain ASCII.
                    let cp = c as u32;
                    buf.push_str(&format!("\\u{{{cp:x}}}"));
                }
            }
        }
        buf.push('"');
        buf
    }
}

/// Emit an f64 as a Rust literal preserving full precision. Special values
/// (NaN, infinities) become explicit `f64::NAN` / `f64::INFINITY`.
fn f64_lit(v: f64) -> String {
    if v.is_nan() {
        return "f64::NAN".to_string();
    }
    if v.is_infinite() {
        return if v > 0.0 {
            "f64::INFINITY".to_string()
        } else {
            "f64::NEG_INFINITY".to_string()
        };
    }
    // {:?} on f64 always preserves round-trip precision.
    format!("{v:?}")
}
