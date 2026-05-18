//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1199/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1199<F: Float>(t1215: F, t2250: F, t2244: F, t1388: F, t3734: F, t1351: F, t3719: F, t1307: F, t3791: F, t12240: F, t1352: F, t3850: F) -> (F, F, F, F, F, F, F, F) {
    let t52531 = t2250 * t1215;
    let t52537 = t2244 * t1215;
    let t53789 = t1388 * t3734;
    let t54542 = t1351 * t3734;
    let t54591 = t3719 * t1351;
    let t54770 = t3791 * t1307;
    let t54858 = t12240 * t1351;
    let t55003 = t1352 * t3850;
    (t52531, t52537, t53789, t54542, t54591, t54770, t54858, t55003)
}
