//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1112/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1112<F: Float>(t1339: F, t22827: F, t54591: F, t550: F, t40197: F, t3788: F, t54858: F, t6936: F, t12392: F, t6945: F, t22765: F, t3858: F, t22764: F, t3777: F, t1354: F, t22756: F) -> (F, F, F, F, F, F, F) {
    let t80978 = t22827 * t1339 * t54591 * t550;
    let t80982 = t22827 * t1339 * t40197 * t550;
    let t80985 = t6936 * t3788 * t54858;
    let t80987 = t6945 * t12392;
    let t80989 = t22765 * t3858;
    let t80991 = t3777 * t22764;
    let t80992 = t80991 * t1354;
    let t80994 = t22756 * t3858;
    (t80978, t80982, t80985, t80987, t80989, t80992, t80994)
}
