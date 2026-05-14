//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 843/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk843<F: Float>(t1799: F, t212: F, t12199: F, t5202: F, t12225: F, t2586: F, t2371: F, t5154: F, t12365: F, t1827: F, t3789: F, t5234: F, t3798: F, t1824: F, t3792: F, t12345: F, t1831: F) -> (F, F, F, F, F, F, F, F) {
    let t16095 = t212 * t1799;
    let t16108 = t12199 * t5202;
    let t16118 = t12225 * t16095;
    let t16119 = t2586 * t16118;
    let t16164 = t5154 * t2371;
    let t16211 = t12365 * t1827;
    let t16285 = t5234 * t3789;
    let t16288 = t5234 * t3798;
    let t16311 = t1824 * t3792;
    let t16317 = t12345 * t1831;
    (t16108, t16119, t16164, t16211, t16285, t16288, t16311, t16317)
}
