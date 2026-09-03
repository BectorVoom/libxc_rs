//! Why do composite functionals disagree with libxc? Compare the mix itself.
//!
//! `composite_oracle.rs` says 52 of 125 composite GGA functionals produce the
//! wrong numbers. This says *which part* of the mix is wrong for each, by
//! reading libxc's own `xc_func_type` through the FFI -- `n_func_aux`,
//! `func_aux[i]->info->number`, `mix_coef[i]`, and each auxiliary's
//! `ext_params` -- and diffing it against what `Functional::new` built.
//!
//! Run with `--nocapture`. This is a diagnostic, not a gate: it always passes
//! and prints a table.

use libxc_rs::functional::Functional;
use libxc_rs::model::{Family, Spin};
use libxc_rs::registry::{all_functional_ids, lookup_by_id};
use libxc_sys::{xc_func_end, xc_func_init, xc_func_type, XC_UNPOLARIZED};

/// What libxc actually built for this functional.
struct CMix {
    aux_ids: Vec<i32>,
    coefs: Vec<f64>,
    /// Per auxiliary: (ext_param name, value), in libxc's own order.
    aux_params: Vec<Vec<(String, f64)>>,
}

fn c_mix(id: u16) -> Option<CMix> {
    let mut t: xc_func_type = unsafe { std::mem::zeroed() };
    if unsafe { xc_func_init(&mut t, id as i32, XC_UNPOLARIZED as i32) } != 0 {
        return None;
    }
    let n = t.n_func_aux as usize;
    let mut out = CMix {
        aux_ids: Vec::with_capacity(n),
        coefs: Vec::with_capacity(n),
        aux_params: Vec::with_capacity(n),
    };
    unsafe {
        for i in 0..n {
            let aux = *t.func_aux.add(i);
            let info = (*aux).info;
            out.aux_ids.push((*info).number);
            out.coefs.push(*t.mix_coef.add(i));

            let ep = &(*info).ext_params;
            let np = ep.n as usize;
            let mut params = Vec::with_capacity(np);
            for k in 0..np {
                let nm = *ep.names.add(k);
                let name = if nm.is_null() {
                    String::new()
                } else {
                    std::ffi::CStr::from_ptr(nm).to_string_lossy().into_owned()
                };
                // `xc_func_type::ext_params` holds the *raw* ext_params, which
                // is the like-for-like counterpart of our own
                // `Functional::ext_params`.
                //
                // Read `->params` instead and the comparison is meaningless:
                // that is the functional's C params struct, which for a
                // transforming setter holds derived quantities and may have
                // more fields than there are ext_params. `gga_x_mpw91` takes
                // `{_bt, _alpha, _expo}` and writes seven struct fields
                // `{a, b, c, d, f, alpha, expo}` with `a = 6*bt/X2S`, so index
                // 0 reads 0.199 where the ext_param is 0.00426 -- a difference
                // that says nothing at all.
                let v = if (*aux).ext_params.is_null() {
                    f64::NAN
                } else {
                    *(*aux).ext_params.add(k)
                };
                params.push((name, v));
            }
            out.aux_params.push(params);
        }
        xc_func_end(&mut t);
    }
    Some(out)
}

#[test]
fn diff_composite_mixes_against_libxc() {
    let mut n_seen = 0;
    let mut n_coef = 0;
    let mut n_ids = 0;
    let mut n_par = 0;
    let mut lines: Vec<String> = Vec::new();

    for id in all_functional_ids() {
        let meta = match lookup_by_id(id.raw()) {
            Ok(m) => m,
            Err(_) => continue,
        };
        if meta.family != Family::Gga || meta.auxiliaries.is_empty() {
            continue;
        }
        let f = match Functional::new(id, Spin::Unpolarized) {
            Ok(f) => f,
            Err(_) => continue,
        };
        let Some(cm) = c_mix(id.raw()) else { continue };
        n_seen += 1;

        let ours = f.auxiliary_functionals();
        let our_ids: Vec<i32> = ours.iter().map(|a| a.meta().id.raw() as i32).collect();
        let mut issues: Vec<String> = Vec::new();

        if our_ids != cm.aux_ids {
            n_ids += 1;
            issues.push(format!("aux ids {our_ids:?} vs libxc {:?}", cm.aux_ids));
        }

        // Mixing coefficients.
        let our_coefs: Vec<f64> = meta.auxiliaries.iter().map(|(_, w)| *w).collect();
        if our_coefs.len() == cm.coefs.len() {
            let bad: Vec<String> = our_coefs
                .iter()
                .zip(cm.coefs.iter())
                .enumerate()
                .filter(|(_, (a, b))| (*a - *b).abs() > 1e-14 * b.abs().max(1.0))
                .map(|(i, (a, b))| format!("coef[{i}] {a} vs {b}"))
                .collect();
            if !bad.is_empty() {
                n_coef += 1;
                issues.push(bad.join(", "));
            }
        }

        // Auxiliary ext_params.
        if our_ids == cm.aux_ids {
            let mut par_bad: Vec<String> = Vec::new();
            for (i, aux) in ours.iter().enumerate() {
                let theirs = &cm.aux_params[i];
                let our_vals = aux.ext_params().unwrap_or(&[]);
                for (k, (nm, cv)) in theirs.iter().enumerate() {
                    if k >= our_vals.len() || !cv.is_finite() {
                        continue;
                    }
                    let ov = our_vals[k];
                    if (ov - cv).abs() > 1e-14 * cv.abs().max(1.0) {
                        par_bad.push(format!("aux[{i}].{nm} {ov} vs {cv}"));
                    }
                }
            }
            if !par_bad.is_empty() {
                n_par += 1;
                issues.push(par_bad.join(", "));
            }
        }

        if !issues.is_empty() {
            lines.push(format!(
                "{:<36} {:>4}  {}",
                meta.name.to_lowercase(),
                id.raw(),
                issues.join(" | ")
            ));
        }
    }

    println!("\n=== composite GGA mixes vs libxc ===");
    println!("examined                 : {n_seen}");
    println!("auxiliary id list differs: {n_ids}");
    println!("mixing coefficient differs: {n_coef}");
    println!("auxiliary ext_param differs: {n_par}");
    println!("\n{} functionals with at least one difference:", lines.len());
    for l in &lines {
        println!("  {l}");
    }
}
