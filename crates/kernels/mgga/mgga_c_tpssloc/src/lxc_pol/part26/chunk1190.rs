//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1190/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1190<F: Float>(t22575: F, t83886: F, t22581: F, t6876: F, t191: F, t192: F, t9419: F, t2020: F, t12451: F, t3701: F, t1983: F, t2019: F, t1874: F, t45640: F, t12823: F, t6525: F) -> (F, F, F, F, F, F) {
    let t83888 = 18.0 * t83886 * t22575;
    let t83896 = 6.0 * t6876 * t22581;
    let t83904 = t9419 * t191 * t192;
    let t83905 = t83904 * t2020;
    let t83911 = t3701 * t12451;
    let t83913 = t1983 * t2019 * t83911;
    let t83917 = 2.0 * t45640 * t1874;
    let t83919 = 6.0 * t12823 * t6525;
    (t83888, t83896, t83905, t83913, t83917, t83919)
}
