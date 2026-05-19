//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1267/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1267<F: Float>(t80970: F, t22827: F, t3788: F, t3792: F, t54770: F, t1339: F, t54591: F, t550: F, t40197: F, t54858: F, t6936: F, t12392: F, t6945: F) -> (F, F, F, F, F, F) {
    let t80971 = F::cast_from(0.43737152435318756759e-3_f64) * t80970;
    let t80974 = t22827 * t3788 * t54770 * t3792;
    let t80978 = t22827 * t1339 * t54591 * t550;
    let t80982 = t22827 * t1339 * t40197 * t550;
    let t80985 = t6936 * t3788 * t54858;
    let t80987 = t6945 * t12392;
    (t80971, t80974, t80978, t80982, t80985, t80987)
}
