//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2647/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2647<F: Float>(t16060: F, t3865: F, t1369: F, t16123: F, t68: F, t1362: F, t1831: F, t40292: F, t12345: F, t5314: F, t12211: F, t16296: F) -> (F, F, F, F, F, F) {
    let t53906 = t16060 * t3865;
    let t53907 = t53906 * t1369;
    let t53909 = t16123 * t68;
    let t53910 = t53909 * t1362;
    let t53917 = t40292 * t1831;
    let t53918 = F::new(119.0) / F::new(1152.0) * t53917;
    let t53919 = t12345 * t5314;
    let t53920 = F::new(119.0) / F::new(1152.0) * t53919;
    let t53921 = t12211 * t16296;
    (t53907, t53909, t53910, t53918, t53920, t53921)
}
