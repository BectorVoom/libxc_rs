//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1341/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1341<F: Float>(t1906: F, t82045: F, t23035: F, t23153: F, t2379: F, t6637: F, t22984: F, t22992: F, t2617: F, t2679: F, t2684: F, t4291: F, t6657: F, t6658: F, t812: F, t82003: F, t82005: F, t82011: F, t82013: F, t82016: F, t82021: F, t82025: F, t82028: F, t82032: F, t82034: F, t82039: F, t82043: F, t829: F, t9612: F, t9958: F) -> F {
    let t82046 = t82045 * t1906;
    let t82047 = F::cast_from(0.27720185200590482541e0_f64) * t82046;
    let t82050 = t23035 * t6637 * t23153 * t2379;
    let t82060 = -F::cast_from(0.82246703342411321825e-2_f64) * t82003 + F::cast_from(0.11514538467937585055e0_f64) * t82005 - t812 * t6657 * t9958 - F::new(3.0) * t2617 * t22984 - F::cast_from(0.19190897446562641759e0_f64) * t82011 - F::cast_from(0.11514538467937585055e0_f64) * t82013 - F::cast_from(0.24674011002723396548e-1_f64) * t82016 - F::cast_from(0.49348022005446793095e-1_f64) * t82021 + F::cast_from(0.49348022005446793095e-1_f64) * t82025 + F::cast_from(0.12337005501361698274e-1_f64) * t82028 - F::cast_from(0.78134368175290755733e-1_f64) * t82032 - F::new(3.0) * t4291 * t82034 * t829 - F::cast_from(0.15626873635058151147e0_f64) * t82039 + F::cast_from(0.82246703342411321825e-2_f64) * t82043 - t82047 + F::cast_from(0.14804406601634037928e0_f64) * t82050 - F::new(3.0) * t812 * t22992 * t2684 - F::new(3.0) * t812 * t22992 * t2679 - F::new(3.0) * t9612 * t6658;
    t82060
}
