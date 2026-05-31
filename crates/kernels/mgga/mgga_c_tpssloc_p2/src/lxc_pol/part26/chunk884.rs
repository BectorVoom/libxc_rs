//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 884/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk884<F: Float>(t1023: F, t10426: F, t4582: F, t1005: F, t3082: F, t1004: F, t3088: F, t1036: F, t3094: F, t1929: F, t35: F, t364: F) -> (F, F, F, F, F) {
    let t10432 = t10426 * t1023;
    let t10433 = t4582 * t10432;
    let t10436 = t1005 * t3082;
    let t10438 = t1004 * t3088;
    let t10441 = t3094 * t1036;
    let t10444 = F::cast_from(1.0_f64) / t35 / t1929;
    let t10445 = t364 * t10444;
    (t10433, t10436, t10438, t10441, t10445)
}
