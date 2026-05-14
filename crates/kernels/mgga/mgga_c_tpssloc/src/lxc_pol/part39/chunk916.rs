//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 916/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk916<F: Float>(t1174: F, t11754: F, t3247: F, t415: F, t121: F, t3584: F, t248: F, t3243: F, t1227: F, t1229: F, t676: F, t1090: F, t3536: F, t3572: F, t3252: F, t3521: F) -> (F, F, F, F, F, F, F) {
    let t11755 = t1174 * t11754;
    let t11778 = 1.0 / t415 / t3247;
    let t11784 = t121 * t3584;
    let t11786 = t248 * t11784 * t3243;
    let t11787 = t1227 * t11786;
    let t11789 = t676 * t1229;
    let t11791 = t248 * t11789 * t1090;
    let t11792 = t1227 * t11791;
    let t11794 = t3536 * t3572;
    let t11797 = t248 * t3521 * t3252;
    (t11755, t11778, t11787, t11789, t11792, t11794, t11797)
}
