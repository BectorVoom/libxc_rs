//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 810/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk810<F: Float>(t2684: F, t6605: F, t6612: F, t30719: F, t808: F, t8344: F, t226: F, t235: F, t2690: F, t2613: F, t8342: F, t23139: F, t8339: F, t23171: F, t23228: F, t8335: F) -> (F, F, F, F, F, F) {
    let t112843 = t6605 * t6612 * t2684;
    let t112846 = t808 * t30719 * t8344;
    let t112850 = t226 * t235 * t2690 * t8344;
    let t112853 = t2613 * t8342 * t8344;
    let t112855 = t23139 * t8339;
    let t112863 = 0.16449340668482264365e-1 * t23171 * t23228 * t8335;
    (t112843, t112846, t112850, t112853, t112855, t112863)
}
