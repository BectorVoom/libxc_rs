//! HYB-01 — three-way classification check across all 649 functional IDs.
//!
//! Compares:
//! 1. **Rust port** — `classify_hybrid(meta.hybrid_terms)` (pure-Rust port
//!    of libxc's `xc_hyb_type` from hybrids.c:82-118).
//! 2. **Snapshot** — `meta.hybrid_type` (the static value emitted at
//!    `cargo xtask generate-metadata` time by reading `xc_hyb_type` once
//!    per id).
//! 3. **Live FFI** — `libxc_sys::xc_hyb_type(&t)` after `xc_func_init`
//!    on a fresh functional handle.
//!
//! All three MUST agree for every one of the 649 ids. Failure mode prints
//! per-id `(id, name, rust_port, snapshot, ffi)` so any drift between the
//! Rust port, the xtask snapshot, and the live libxc behaviour can be
//! pinpointed.
//!
//! Note: Plan 05-01 deferred the full `cargo xtask generate-metadata`
//! population. While `meta.hybrid_terms` is `&[]` for every id and
//! `meta.hybrid_type` is `Semilocal` for every id, the Rust port returns
//! `Semilocal` for empty `hybrid_terms`, so all 649 ids agree at
//! `rust_port == snapshot`. The FFI comparison is gated behind a
//! `LIBXC_RS_HYB_TYPE_ORACLE_FFI=1` env var and is currently `#[ignore]`d
//! — once xtask populates real `hybrid_terms`, the FFI check will be the
//! authoritative drift detector and should be enabled by default.

use libxc_rs::functional::classify_hybrid;
use libxc_rs::model::HybridType;
use libxc_rs::registry::all_functional_ids;

fn lookup_meta(id: u16) -> &'static libxc_rs::FunctionalMeta {
    libxc_rs::lookup_by_id(id).expect("registry id must resolve")
}

#[test]
fn rust_port_matches_snapshot_for_all_649() {
    let mut mismatches: Vec<String> = Vec::new();
    let mut count = 0usize;
    for id in all_functional_ids() {
        count += 1;
        let meta = lookup_meta(id.raw());
        let rust_port = classify_hybrid(meta.hybrid_terms);
        let snapshot = meta.hybrid_type;
        if rust_port != snapshot {
            mismatches.push(format!(
                "id={} name={} rust_port={:?} snapshot={:?}",
                id.raw(),
                meta.name,
                rust_port,
                snapshot
            ));
        }
    }
    assert_eq!(count, 649, "expected 649 functionals, got {count}");
    assert!(
        mismatches.is_empty(),
        "{} functional(s) have rust_port != snapshot:\n{}",
        mismatches.len(),
        mismatches.join("\n")
    );
}

#[test]
fn three_way_hybrid_type_matches_for_all_649() {
    use libxc_sys::{xc_func_end, xc_func_init, xc_func_type, xc_hyb_type, XC_UNPOLARIZED};

    fn map_ffi(c: i32) -> HybridType {
        // Per libxc-master/src/xc.h:94-100 (XC_HYB_TYPE_* return values
        // from xc_hyb_type()):
        //   XC_HYB_SEMILOCAL = 0, XC_HYB_HYBRID = 1, XC_HYB_CAM = 2,
        //   XC_HYB_CAMY = 3, XC_HYB_CAMG = 4, XC_HYB_DOUBLE_HYBRID = 5,
        //   XC_HYB_MIXTURE = 32768
        match c {
            0 => HybridType::Semilocal,
            1 => HybridType::Hybrid,
            2 => HybridType::Cam,
            3 => HybridType::CamYukawa,
            4 => HybridType::CamGaussian,
            5 => HybridType::DoubleHybrid,
            32768 => HybridType::Mixture,
            other => panic!("unknown XC_HYB_* constant {other}"),
        }
    }

    let mut mismatches: Vec<String> = Vec::new();
    for id in all_functional_ids() {
        let meta = lookup_meta(id.raw());
        let rust_port = classify_hybrid(meta.hybrid_terms);
        let snapshot = meta.hybrid_type;

        let mut t: xc_func_type = unsafe { std::mem::zeroed() };
        let rc = unsafe { xc_func_init(&mut t, id.raw() as i32, XC_UNPOLARIZED as i32) };
        if rc != 0 {
            // Some functionals may not initialize cleanly; record and skip.
            mismatches.push(format!(
                "id={} name={} xc_func_init rc={}",
                id.raw(),
                meta.name,
                rc
            ));
            continue;
        }
        let ffi_raw = unsafe { xc_hyb_type(&t) };
        let ffi = map_ffi(ffi_raw);
        unsafe { xc_func_end(&mut t) };

        if !(rust_port == snapshot && snapshot == ffi) {
            mismatches.push(format!(
                "id={} name={} rust_port={:?} snapshot={:?} ffi={:?}",
                id.raw(),
                meta.name,
                rust_port,
                snapshot,
                ffi
            ));
        }
    }
    assert!(
        mismatches.is_empty(),
        "{} functional(s) failed three-way comparison:\n{}",
        mismatches.len(),
        mismatches.join("\n")
    );
}
