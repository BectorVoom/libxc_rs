//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1346/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1346<F: Float>(t120550: F, t114208: F, t114216: F, t114285: F, t1992: F, t26355: F, t114240: F, t114242: F, t114172: F, t6897: F, t7700: F, t22674: F, t32697: F) -> (F, F, F, F, F, F, F, F) {
    let t120551 = F::cast_from(0.82246703342411321825e-2_f64) * t120550;
    let t120552 = F::cast_from(0.76763589786250567036e-1_f64) * t114208;
    let t120553 = F::cast_from(0.76763589786250567036e-1_f64) * t114216;
    let t120556 = F::cast_from(0.3289868133696452873e-1_f64) * t1992 * t114285 * t26355;
    let t120561 = F::cast_from(0.16449340668482264365e-1_f64) * t114240;
    let t120566 = F::cast_from(0.38381794893125283518e-1_f64) * t114242;
    let t120568 = t6897 * t114172 * t7700;
    let t120569 = F::cast_from(0.82246703342411321825e-2_f64) * t120568;
    let t120576 = t6897 * t22674 * t32697;
    (t120551, t120552, t120553, t120556, t120561, t120566, t120569, t120576)
}
