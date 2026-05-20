//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 931/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk931<F: Float>(t226: F, t235: F, t2690: F, t8344: F, t2613: F, t8342: F, t23139: F, t8339: F, t23171: F, t23228: F, t8335: F, t30623: F, t81651: F, t82074: F) -> (F, F, F, F, F) {
    let t112850 = t226 * t235 * t2690 * t8344;
    let t112853 = t2613 * t8342 * t8344;
    let t112855 = t23139 * t8339;
    let t112863 = F::cast_from(0.16449340668482264365e-1_f64) * t23171 * t23228 * t8335;
    let t112867 = t81651 * t82074 * t30623;
    (t112850, t112853, t112855, t112863, t112867)
}
