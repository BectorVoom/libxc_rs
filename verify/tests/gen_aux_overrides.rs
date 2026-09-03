//! Generator: the auxiliary `ext_params` a composite functional overrides.
//!
//! Run with `LIBXC_RS_WRITE_AUX_OVERRIDES=1` to regenerate
//! `crates/libxc-core/src/meta/generated_aux_overrides.rs`:
//!
//! ```text
//! LIBXC_RS_WRITE_AUX_OVERRIDES=1 cargo test --release \
//!     --manifest-path verify/Cargo.toml --test gen_aux_overrides -- --nocapture
//! ```
//!
//! # Why this is generated from libxc rather than scraped from its C
//!
//! When libxc builds a composite with `xc_mix_init`, the parent's `_init` and
//! `set_ext_params` may override its auxiliaries' parameters -- `lc_blyp` hands
//! its own `_omega` down to `gga_x_ityh`, `sogga` overrides PBE's `_kappa` and
//! `_mu`, HSE06 pins one leg's `_omega` to zero and sets the other's. Those
//! assignments are ordinary C spread across 10+ files in several shapes
//! (literals, locals, `p->hyb_omega[k]`, values computed in the init), and
//! scraping them is exactly the kind of guessing the rest of this pipeline
//! refuses to do.
//!
//! They are, however, directly *observable*: after `xc_func_init`, each
//! auxiliary's `xc_func_type::ext_params` holds the values libxc will actually
//! evaluate with. This walks every functional, compares each auxiliary's
//! ext_params against that auxiliary's own declared defaults, and records the
//! differences. No parsing, no inference -- the table is libxc's own answer.
//!
//! # What it does not capture
//!
//! A snapshot taken at the parent's *default* ext_params. If a caller changes
//! a parent parameter that feeds an auxiliary, the override is stale unless a
//! rule in `meta::composite_setters` or `PROPAGATION_RULES` also describes the
//! relationship. That is why those tables still exist and why HSE06 has one:
//! this fixes the defaults for everything, and the rules keep the functionals
//! that have them correct under mutation too.

use libxc_rs::model::Family;
use libxc_rs::registry::{all_functional_ids, lookup_by_id};
use libxc_sys::{xc_func_end, xc_func_init, xc_func_type, XC_UNPOLARIZED};

/// `(aux slot, ext_param name, value libxc evaluates with)`.
type Override = (usize, String, f64);

fn overrides_for(id: u16) -> Option<Vec<Override>> {
    let mut t: xc_func_type = unsafe { std::mem::zeroed() };
    if unsafe { xc_func_init(&mut t, id as i32, XC_UNPOLARIZED as i32) } != 0 {
        return None;
    }
    let mut out = Vec::new();
    unsafe {
        for i in 0..t.n_func_aux as usize {
            let aux = *t.func_aux.add(i);
            let info = (*aux).info;
            let ep = &(*info).ext_params;
            if (*aux).ext_params.is_null() {
                continue;
            }
            for k in 0..ep.n as usize {
                let live = *(*aux).ext_params.add(k);
                let deflt = *ep.values.add(k);
                if !live.is_finite() {
                    continue;
                }
                // Only record a genuine override. Bit comparison so a
                // deliberate -0.0 counts and a no-op does not.
                if live.to_bits() == deflt.to_bits() {
                    continue;
                }
                let nm = *ep.names.add(k);
                let name = if nm.is_null() {
                    continue;
                } else {
                    std::ffi::CStr::from_ptr(nm).to_string_lossy().into_owned()
                };
                out.push((i, name, live));
            }
        }
        xc_func_end(&mut t);
    }
    Some(out)
}

#[test]
fn generate_aux_overrides_table() {
    let mut rows: Vec<(u16, &'static str, Vec<Override>)> = Vec::new();
    let mut n_composite = 0usize;

    for id in all_functional_ids() {
        let Ok(meta) = lookup_by_id(id.raw()) else { continue };
        if meta.auxiliaries.is_empty() {
            continue;
        }
        n_composite += 1;
        let Some(ov) = overrides_for(id.raw()) else { continue };
        if !ov.is_empty() {
            rows.push((id.raw(), meta.name, ov));
        }
    }
    rows.sort_by_key(|r| r.0);

    let total: usize = rows.iter().map(|r| r.2.len()).sum();
    println!("composite functionals            : {n_composite}");
    println!("with auxiliary ext_param overrides: {}", rows.len());
    println!("individual overrides              : {total}");

    let mut by_family = std::collections::BTreeMap::new();
    for (id, _, _) in &rows {
        if let Ok(m) = lookup_by_id(*id) {
            *by_family.entry(format!("{:?}", m.family)).or_insert(0usize) += 1;
        }
    }
    println!("by family                         : {by_family:?}");

    if std::env::var("LIBXC_RS_WRITE_AUX_OVERRIDES").is_err() {
        println!("\n(set LIBXC_RS_WRITE_AUX_OVERRIDES=1 to write the table)");
        // Show a sample so a plain run is still informative.
        for (id, name, ov) in rows.iter().take(6) {
            println!("  {:>4} {:<34} {ov:?}", id, name.to_lowercase());
        }
        return;
    }

    let mut s = String::new();
    s.push_str(
        "//! Auxiliary `ext_params` that a composite functional overrides.\n\
         //!\n\
         //! GENERATED by `verify/tests/gen_aux_overrides.rs` from libxc 7.0.0\n\
         //! itself -- do not hand-edit. Regenerate with:\n\
         //!\n\
         //! ```text\n\
         //! LIBXC_RS_WRITE_AUX_OVERRIDES=1 cargo test --release \\\n\
         //!     --manifest-path verify/Cargo.toml --test gen_aux_overrides -- --nocapture\n\
         //! ```\n\
         //!\n\
         //! When libxc builds a composite with `xc_mix_init`, the parent's init\n\
         //! and setter may override its auxiliaries' parameters: `lc_blyp` hands\n\
         //! its `_omega` down to `gga_x_ityh`, `gga_x_sogga` overrides PBE's\n\
         //! `_kappa` and `_mu`, HSE06 pins one leg's `_omega` to zero. Without\n\
         //! this table each auxiliary ran on its own defaults, which for 47 of\n\
         //! the 125 composite GGAs is a different functional entirely.\n\
         //!\n\
         //! Each row is read straight out of libxc's `xc_func_type` after\n\
         //! `xc_func_init` and compared against that auxiliary's own declared\n\
         //! default, so the values are libxc's own answer rather than a\n\
         //! transcription of its C.\n\
         //!\n\
         //! **Snapshot at the parent's default ext_params.** Changing a parent\n\
         //! parameter that feeds an auxiliary leaves this stale unless\n\
         //! `composite_setters` or `PROPAGATION_RULES` also describes the\n\
         //! relationship -- which is why those tables still exist.\n\
         #![allow(clippy::all)]\n\n\
         use crate::model::FunctionalId;\n\n\
         /// `(parent, aux slot, ext_param name, value)`.\n\
         #[derive(Debug, Clone, Copy, PartialEq)]\n\
         pub struct AuxExtParamOverride {\n    \
             pub parent_id: FunctionalId,\n    \
             pub aux_slot: u8,\n    \
             pub name: &'static str,\n    \
             pub value: f64,\n\
         }\n\n\
         /// Every auxiliary parameter libxc overrides, sorted by parent id.\n\
         pub const AUX_EXT_PARAM_OVERRIDES: &[AuxExtParamOverride] = &[\n",
    );
    for (id, name, ov) in &rows {
        s.push_str(&format!("    // {}\n", name.to_lowercase()));
        for (slot, pname, v) in ov {
            s.push_str(&format!(
                "    AuxExtParamOverride {{ parent_id: FunctionalId({id}), aux_slot: {slot}, \
                 name: {pname:?}, value: {v:?} }},\n"
            ));
        }
    }
    s.push_str("];\n");

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../crates/libxc-core/src/meta/generated_aux_overrides.rs");
    std::fs::write(&path, s).expect("write table");
    println!("\nwrote {} overrides to {}", total, path.display());
}
