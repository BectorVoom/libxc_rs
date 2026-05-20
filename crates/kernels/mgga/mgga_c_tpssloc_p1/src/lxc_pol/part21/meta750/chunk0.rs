//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2622/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2622<F: Float>(t12339: F, t5314: F, t1831: F, t40059: F, t16336: F, t3872: F, t16060: F, t3865: F, t1369: F, t16123: F, t68: F, t1362: F) -> (F, F, F, F, F, F, F) {
    let t53897 = t12339 * t5314;
    let t53901 = t40059 * t1831;
    let t53903 = t16336 * t3872;
    let t53906 = t16060 * t3865;
    let t53907 = t53906 * t1369;
    let t53909 = t16123 * t68;
    let t53910 = t53909 * t1362;
    (t53897, t53901, t53903, t53906, t53907, t53909, t53910)
}
