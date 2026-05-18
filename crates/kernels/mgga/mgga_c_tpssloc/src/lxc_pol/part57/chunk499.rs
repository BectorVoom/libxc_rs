//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 499/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk499<F: Float>(t6197: F, t6237: F, t466: F, t1760: F, t3598: F, t491: F, t6224: F, t3612: F, t1734: F, t1751: F, t1246: F, t6218: F) -> (F, F, F, F, F, F, F) {
    let t6238 = t6197 + t6237;
    let t6239 = t466 * t6238;
    let t6243 = t1760 * t1760;
    let t6244 = t3598 * t6243;
    let t6252 = t491 * t6224;
    let t6253 = t6252 * t3612;
    let t6256 = t1751 * t1734;
    let t6257 = t6256 * t1246;
    let t6260 = t491 * t6218;
    (t6238, t6239, t6244, t6252, t6253, t6257, t6260)
}
